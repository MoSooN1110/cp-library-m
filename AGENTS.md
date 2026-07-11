# AGENTS.md — cp-library (cplib) 開発方針

このリポジトリは競技プログラミング用 Rust ライブラリ `cplib`。
**「常にコンパイルが通る cargo クレート」＋「依存を自動解決して提出用1ファイルへ展開する expander」** を核とする。
新しいライブラリを追加・移行する際はこの方針に従うこと。

## 全体像

```
src/lib.rs                pub mod <category>;   ← カテゴリ宣言のみ
src/<category>.rs         pub mod <name>;       ← 名前空間宣言のみ
src/<category>/<name>.rs  ← 実装本体（= leaf module）。1 概念 1 ファイル
tools/expand.py           提出用 expander
_pending/                 未移行の抽出ライブラリ（移行の元ネタ。順次 src/ へ）
_archive_*/               旧実装・統合済み実装（保管のみ、触らない）
```

- クレート名は `cplib`。カテゴリ: `math`, `ds`, `graph`, `string`, `geometry`（必要に応じ追加）。
- モジュール名・ファイル名は **snake_case**。1 つの概念につき 1 ファイル・1 canonical 実装。

## モジュールを書くときのルール（expander と両立させるため必須）

1. **自己完結**。外部テンプレの `MOD` / `read_vec` などグローバルに依存しない。
   定数や補助関数はモジュール内に持つ。
2. **他モジュールへの依存は必ず `use crate::<category>::<name>::...;` で書く**。
   - expander はモジュール本文中の `crate::cat::name` 参照を走査して依存を推移解決する。
   - cargo もこの参照でコンパイル検査するので、書けば正しさが保証される。
3. **公開 API は `pub`**。提出時は leaf module が `pub mod name { ... }` として inline され、
   利用側は `use crate::cat::name::*;` で使う。
4. **module doc は `//!` で先頭に**。使用例を rustdoc の ```` ```rust ```` ブロックで 1 つ書く
   （`cargo test` の doctest で例も検証される）。expander は `//!` を除去する。
5. **単体テストは `#[cfg(test)] mod tests { ... }`**。expander が展開時に丸ごと除去する。
   ナイーブ解との突き合わせ・既知値・ランダムテストを最低 1 つ入れる。
6. **やってはいけないこと**（expander が壊れる/汚れる）:
   - エクスポートするマクロ内で `$crate` を使う（expander は `$crate` を書き換えない）。
     マクロはできるだけ避け、必要なら crate 相対パスを使わない形にする。
   - 文字列リテラルやコメントに `crate::cat::name` という並びを書く（誤検出の元）。
   - モジュールファイルに crate レベル inner attribute（`#![...]`）を書く。

## 追加・移行の手順

1. `_pending/` の元コードを読み、canonical 実装を `src/<cat>/<name>.rs` に書く（上のルール順守）。
2. `src/<cat>.rs` に `pub mod <name>;`、必要なら `src/lib.rs` に `pub mod <cat>;` を追加。
3. `cargo test --lib` が緑になるまで直す（doctest 含む）。
4. `examples/sample_main.rs` 等で expand→単体コンパイルできることを確認（CI でも実行）。
5. 移行元は `_pending/` から削除（履歴に残る）。README の収録表を更新。

## 重複の統合方針

同概念の複数版（例: wavelet_matrix×4, hld×2, trie×3, dijkstra 複数, segtree 派生）は
**最も汎用・高速・テスト可能な 1 本に統合**し、他は移行しない（`_pending` に残すか削除）。
迷ったら「汎用性 > 速度 > 記述量」の順で選ぶ。

## テスト/CI

- ローカル: `cargo test --lib`（全モジュール＋doctest）。
- CI (`.github/workflows/ci.yml`): `cargo test --lib` ＋ expander smoke（展開→`rustc`→実行）。
- 新モジュールは必ずテスト付きで追加し、CI を緑に保つ。

## expander の使い方（参考）

```bash
# 開発時（cplib を依存に持つ crate で、IDE 補完付きで書く）
#   use cplib::ds::segtree::*;   または   //@use ds::segtree
python3 tools/expand.py main.rs -o submit.rs   # 依存ごと 1 ファイルへ
```
