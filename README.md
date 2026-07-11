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
│   ├── ds/               dsu, fenwick, segtree, ...
│   └── geometry/         basic, ...
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
| `math::vector_lcm` | 配列全体の gcd/lcm と checked lcm |
| `math::gaussian_elimination` | 実数行列の掃き出し法（rank・RREF・連立一次方程式） |
| `algo::bit_enumeration` | ビット全探索・部分集合/部分 mask 列挙 |
| `ds::dsu` | Union-Find（ACL 互換 API） |
| `ds::fenwick` | Fenwick Tree（点加算・区間和、ジェネリック） |
| `ds::segtree` | Segment Tree（モノイド、点更新・区間積） |
| `ds::segtree_2d` | 2D Segment Tree（点更新・矩形区間積） |
| `ds::lazy_segtree` | 遅延伝播セグメント木（ACL 準拠、区間作用・区間積） |
| `ds::dynamic_segtree` | 動的セグメント木（大きな添字範囲での疎な点更新・区間積） |
| `ds::slope_trick` | Slope Trick（区分線形凸関数の最小値管理） |
| `ds::multiset` | 多重集合（BTreeMap ベース、順序境界つき） |
| `ds::wavelet_matrix` | Wavelet Matrix（区間頻度・k 番目・range_freq、値域 `u64`） |
| `ds::segtree_beats` | Segment Tree Beats（区間 chmin/chmax・区間加算・区間和、`i64`） |
| `ds::implicit_treap` | Implicit Treap（挿入・削除・分割・結合 O(log n) の可変長列） |
| `graph::dijkstra` | ダイクストラ（距離・経路復元・2 番目に短い距離） |
| `graph::bfs` | BFS（重みなし最短路） |
| `graph::components` | 無向グラフの連結成分分解（成分 ID・成分リスト・連結判定） |
| `graph::grid_bfs` | グリッド上の 4 近傍 BFS・多始点 BFS・01 BFS |
| `graph::scc` | 強連結成分分解（Kosaraju、トポロジカル順） |
| `graph::hld` | Heavy-Light Decomposition（LCA・パス/部分木区間分解） |
| `graph::rerooting` | 全方位木 DP（rerooting DP） |
| `geometry::basic` | 整数座標の基本幾何（点・外積・ccw・線分交差・多角形面積） |
| `dp::digit_dp` | 桁 DP（上限以下の整数の桁和分布・個数・総和） |
| `string::rolling_hash` | ローリングハッシュ（mod 2^61-1） |
| `string::z_algorithm` | Z-algorithm |

未移行分は `_pending/` にあり、順次 `src/` へ移行していきます（一覧は [`INDEX.md`](./INDEX.md)）。
方針は [`AGENTS.md`](./AGENTS.md) を参照。

## ライセンス

個人用ライブラリ。自由に利用可。
