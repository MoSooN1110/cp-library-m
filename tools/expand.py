#!/usr/bin/env python3
"""cplib expander — 提出用 1 ファイル生成ツール。

使い方:
    python tools/expand.py path/to/main.rs > submit.rs
    python tools/expand.py path/to/main.rs -o submit.rs

main.rs 側で使うライブラリを次のどちらかで指定する:
    use cplib::ds::segtree::*;      # 開発時は本物の crate 参照（IDE 補完が効く）
    //@use ds::segtree               # あるいは明示マーカー

expander は依存 (`crate::...` 参照) を自動解決し、必要なモジュールだけを
クレートと同じ階層 (`pub mod cat { pub mod name { ... } }`) で先頭に inline する。
main.rs 内の `cplib::` は `crate::` に書き換える。
"""
import argparse
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
LIB_ROOT = os.path.dirname(HERE)       # .../library
SRC = os.path.join(LIB_ROOT, "src")


def build_registry():
    """leaf module path (e.g. 'ds::segtree') -> ソースファイルの絶対パス。"""
    reg = {}
    for dirpath, dirnames, filenames in os.walk(SRC):
        for fn in filenames:
            if not fn.endswith(".rs"):
                continue
            stem = fn[:-3]
            full = os.path.join(dirpath, fn)
            if stem == "lib":
                continue
            # 同名ディレクトリを持つ .rs は名前空間インデックス (math.rs 等) → leaf ではない
            if os.path.isdir(os.path.join(dirpath, stem)):
                continue
            rel = os.path.relpath(full, SRC)[:-3]  # ds/segtree
            modpath = rel.replace(os.sep, "::")
            reg[modpath] = full
    return reg


def strip_body(text):
    """inline 用に module doc(//!) と #[cfg(test)] ブロックを除去。"""
    # #[cfg(test)] ... { balanced }
    out = []
    i = 0
    while True:
        m = re.search(r"#\[cfg\(test\)\]", text[i:])
        if not m:
            out.append(text[i:])
            break
        start = i + m.start()
        out.append(text[i:start])
        # 次の '{' を探して balanced に飛ばす
        brace = text.find("{", start)
        if brace == -1:
            i = len(text)
            break
        depth = 0
        j = brace
        while j < len(text):
            c = text[j]
            if c == "{":
                depth += 1
            elif c == "}":
                depth -= 1
                if depth == 0:
                    j += 1
                    break
            j += 1
        i = j
    text = "".join(out)
    # module doc comment を除去
    lines = [ln for ln in text.splitlines() if not ln.lstrip().startswith("//!")]
    return "\n".join(lines).strip("\n")


def find_deps(body, registry):
    deps = set()
    for mod in registry:
        if re.search(r"\bcrate::" + re.escape(mod) + r"\b", body):
            deps.add(mod)
    return deps


def resolve(requested, registry):
    """要求モジュール集合から推移的閉包を返す。body もキャッシュして返す。"""
    bodies = {}
    seen = set()
    stack = list(requested)
    while stack:
        mod = stack.pop()
        if mod in seen:
            continue
        if mod not in registry:
            sys.exit(f"[expand] 未知のモジュール: {mod}")
        seen.add(mod)
        raw = open(registry[mod], encoding="utf-8").read()
        body = strip_body(raw)
        bodies[mod] = body
        for d in find_deps(body, registry):
            if d != mod:
                stack.append(d)
    return seen, bodies


def emit_modules(mods, bodies):
    """cat::name の集合を pub mod ツリーとして出力。"""
    tree = {}
    for mod in sorted(mods):
        segs = mod.split("::")
        node = tree
        for s in segs[:-1]:
            node = node.setdefault(s, {})
        node[segs[-1]] = {"__body__": bodies[mod]}

    def render(node, indent):
        pad = "    " * indent
        parts = []
        for key in sorted(node):
            val = node[key]
            if "__body__" in val:
                parts.append(f"{pad}pub mod {key} {{")
                for ln in val["__body__"].splitlines():
                    parts.append((pad + "    " + ln) if ln else "")
                parts.append(f"{pad}}}")
            else:
                parts.append(f"{pad}pub mod {key} {{")
                parts.append(render(val, indent + 1))
                parts.append(f"{pad}}}")
        return "\n".join(parts)

    return render(tree, 0)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("main", help="提出元 main.rs")
    ap.add_argument("-o", "--output", help="出力先（省略時は stdout）")
    args = ap.parse_args()

    src = open(args.main, encoding="utf-8").read()
    registry = build_registry()

    requested = set()
    # 1) use cplib::a::b...
    for mod in registry:
        if re.search(r"\bcplib::" + re.escape(mod) + r"\b", src):
            requested.add(mod)
    # 2) //@use a::b マーカー
    marker_mods = re.findall(r"//@use\s+([A-Za-z0-9_:]+)", src)
    for mod in marker_mods:
        requested.add(mod)

    if not requested:
        sys.stderr.write("[expand] 警告: cplib モジュール参照が見つかりません。\n")

    mods, bodies = resolve(requested, registry)

    # main.rs 変換: マーカー行を use に、cplib:: を crate:: に
    def marker_to_use(m):
        return f"use crate::{m.group(1)}::*;"

    user = re.sub(r"//@use\s+([A-Za-z0-9_:]+)\s*", marker_to_use, src)
    user = re.sub(r"\bcplib::", "crate::", user)

    # inner attribute (#![...]) はファイル先頭にしか置けないので巻き上げる
    inner_attrs = re.findall(r"^[ \t]*#!\[.*\]\s*$", user, flags=re.M)
    user = re.sub(r"^[ \t]*#!\[.*\]\s*$\n?", "", user, flags=re.M)

    header = "// generated by tools/expand.py — do not edit\n"
    modblock = emit_modules(mods, bodies) if mods else ""
    # モジュールはクレート直下にマウントする（library crate と同じ階層 → 内部の
    # `crate::cat::name` 参照がそのまま解決する）。
    out = "#![allow(dead_code, unused_imports, unused_macros, unused_variables)]\n"
    for a in inner_attrs:
        out += a.strip() + "\n"
    out += header
    if modblock:
        out += modblock + "\n\n"
    out += user
    if not out.endswith("\n"):
        out += "\n"

    if args.output:
        open(args.output, "w", encoding="utf-8").write(out)
        used = ", ".join(sorted(mods)) if mods else "(none)"
        sys.stderr.write(f"[expand] {args.output} <- modules: {used}\n")
    else:
        sys.stdout.write(out)


if __name__ == "__main__":
    main()
