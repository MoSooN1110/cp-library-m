# cp-library

競技プログラミング用 Rust ライブラリ。**cargo で常時コンパイル検査**され、
**expander** で依存を自動解決して提出用 1 ファイルに展開できます。

## 構成

```
.
├── Cargo.toml            crate 名 = cplib
├── src/
│   ├── lib.rs            モジュールツリー宣言
│   ├── math/             modint, matrix, ...
│   └── ds/               dsu, fenwick, segtree, ...
├── tools/expand.py       提出用 expander
├── examples/             サンプル解答
├── _pending/             未移行の抽出ライブラリ（順次 src/ へ移行）
└── _archive_*/           旧実装・統合済み実装
```

## 開発〜提出フロー

**1. 解答を書く**（`cplib` を依存に持つ crate で、IDE 補完が効く状態で書く）

```rust
use cplib::ds::segtree::*;
use cplib::math::modint::*;

fn main() {
    let seg = SegTree::from_slice(&[1i64, 3, 2], i64::MIN, |a, b| a.max(b));
    println!("{}", seg.prod(0..3));
}
```

`//@use ds::segtree` というマーカーコメントでも指定できます。

**2. 提出用に展開**（依存ライブラリだけを inline した 1 ファイルを生成）

```bash
python3 tools/expand.py main.rs -o submit.rs
```

- `use cplib::...` / `//@use ...` から使用モジュールを検出
- 各モジュール内の `crate::...` 参照から**依存を推移的に自動解決**
- 必要なモジュールだけをクレートと同じ階層で先頭に inline
- `submit.rs` は **cplib 非依存で単体コンパイル可能**

## テスト

```bash
cargo test --lib                        # 全ライブラリのコンパイル＋単体テスト
python3 tools/expand.py examples/sample_main.rs -o /tmp/s.rs && rustc /tmp/s.rs -o /tmp/s && /tmp/s
```

CI（GitHub Actions）でも上記を自動実行しています。

## 収録ライブラリ（移行済み）

| module | 内容 |
|---|---|
| `math::modint` | mod 演算 `Mint` ＋組合せ `Comb`（nCr/nPr/nHr）。MOD 内蔵 |
| `math::matrix` | `Mint` 行列（積・累乗）。`math::modint` に依存 |
| `math::prime` | 線形篩・素数列挙・Miller-Rabin・Pollard rho 素因数分解 |
| `math::number` | gcd/lcm・拡張ユークリッド・一般 mod 逆元・CRT |
| `ds::dsu` | Union-Find（ACL 互換 API） |
| `ds::fenwick` | Fenwick Tree（点加算・区間和、ジェネリック） |
| `ds::segtree` | Segment Tree（モノイド、点更新・区間積） |
| `ds::lazy_segtree` | 遅延伝播セグメント木（ACL 準拠、区間作用・区間積） |
| `ds::multiset` | 多重集合（BTreeMap ベース、順序境界つき） |
| `graph::dijkstra` | ダイクストラ（距離・経路復元） |
| `graph::bfs` | BFS（重みなし最短路） |
| `graph::scc` | 強連結成分分解（Kosaraju、トポロジカル順） |
| `string::rolling_hash` | ローリングハッシュ（mod 2^61-1） |
| `string::z_algorithm` | Z-algorithm |

未移行分は `_pending/` にあり、順次 `src/` へ移行していきます（一覧は [`INDEX.md`](./INDEX.md)）。
方針は [`AGENTS.md`](./AGENTS.md) を参照。

## ライセンス

個人用ライブラリ。自由に利用可。
