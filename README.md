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
# main.rs 内の //@expand 行にライブラリを差し込む。マーカーがなければ先頭に差し込む
python3 tools/expand.py src/main -o submit.rs   # src/main.rs を読む
```

- `use cplib::...` / `//@use ...` から使用モジュールを検出
- 各モジュール内の `crate::...` 参照から**依存を推移的に自動解決**
- 必要なモジュールだけをクレートと同じ階層で先頭に inline
- `//@expand` / `// @expand` / `// cplib:expand` の 1 行コメントで inline 位置を予約可能
- `submit.rs` は **cplib 非依存で単体コンパイル可能**

## テスト

```bash
cargo test --lib                        # 全ライブラリのコンパイル＋単体テスト
python3 tools/expand.py examples/sample_main.rs -o /tmp/s.rs && rustc /tmp/s.rs -o /tmp/s && /tmp/s
```

CI（GitHub Actions）でも上記を自動実行しています。

## 収録ライブラリ

| module | 内容 |
|---|---|
| `math::modint` | mod 演算 `Mint` ＋組合せ `Comb`（nCr/nPr/nHr）。MOD 内蔵 |
| `math::matrix` | `Mint` 行列（積・累乗）。`math::modint` に依存 |
| `math::matrix_mod` | mod 998244353 行列（rank・行列式・逆行列・連立一次方程式）。`math::modint` に依存 |
| `math::prime` | 線形篩・素数列挙・Miller-Rabin・Pollard rho 素因数分解 |
| `math::number` | gcd/lcm・拡張ユークリッド・一般 mod 逆元・CRT |
| `math::vector_lcm` | 配列全体の gcd/lcm と checked lcm |
| `math::gaussian_elimination` | 実数行列の掃き出し法（rank・RREF・連立一次方程式） |
| `math::convolution` | NTT 畳み込み（mod 998244353）。`math::modint` に依存 |
| `math::fps` | 形式的冪級数（加減乗算・微積分・逆元・log/exp/pow・除算）。`math::convolution` に依存 |
| `math::fft` | f64 FFT（実数畳み込み・任意 mod 乗算、15bit 分割） |
| `math::hadamard` | Walsh–Hadamard 変換と XOR/AND/OR 畳み込み |
| `math::prime_count` | 高速素数計数 π(n)（Lucy_Hedgehog 系、O(n^(3/4))） |
| `math::linear_recurrence` | Berlekamp-Massey と Bostan-Mori による線形漸化式の推定・k 番目項 |
| `math::lagrange_interpolation` | ラグランジュ補間（連続標本 O(n)・任意標本 O(n²)、mod 素数） |
| `math::quotient_ranges` | 商 `n/i` が一定になる区間列挙 |
| `math::digit_sum` | 桁和・桁列挙 |
| `math::diophantine` | 一次不定方程式 ax+by=c の整数解 |
| `math::divisors` | 約数列挙・約数個数 |
| `math::euler_phi` | オイラー φ 関数 |
| `math::floor_sum` | floor_sum（Σ floor((a*i+b)/m)、ACL 互換） |
| `math::garner` | Garner のアルゴリズム（CRT 合成） |
| `math::gauss_xor` | F2（xor）掃き出し法 |
| `math::lucas` | Lucas の定理（巨大 n の nCr mod 素数） |
| `math::ratio` | 有理数（正規化・厳密比較） |
| `math::xor_basis` | xor 基底（線形独立性・最大 xor） |
| `algo::binary_search` | 汎用二分探索（判定関数の境界） |
| `algo::bit_enumeration` | ビット全探索・部分集合/部分 mask 列挙 |
| `algo::chmin_chmax` | chmin / chmax |
| `algo::compress` | 座標圧縮 |
| `algo::grid_transform` | グリッドの回転・転置・反転、回転＋平行移動での合同判定 |
| `algo::inversion` | 転倒数（BIT） |
| `algo::lis` | 最長増加部分列 |
| `algo::max_rectangle` | ヒストグラム・グリッドの最大長方形 |
| `algo::mo_algorithm` | Mo 法のクエリ順序生成 |
| `algo::next_permutation` | 順列の辞書順列挙 |
| `algo::product` | 同一基数・可変基数の直積列挙 iterator |
| `algo::ternary_search` | 三分探索（整数・実数） |
| `ds::dsu` | Union-Find（ACL 互換 API） |
| `ds::weighted_dsu` | 重み付き Union-Find（差分制約） |
| `ds::rollback_dsu` | Rollback 可能 Union-Find（undo・snapshot、オフライン動的連結性の部品） |
| `ds::sliding_window_min` | 固定長窓の最小値・最大値（単調 deque） |
| `ds::swag` | Sliding Window Aggregation（半群 queue 集約） |
| `ds::fenwick` | Fenwick Tree（点加算・区間和、ジェネリック） |
| `ds::cumsum` | 累積和（区間和 O(1)） |
| `ds::cumsum_2d` | 2D 累積和（矩形和 O(1)） |
| `ds::segtree` | Segment Tree（モノイド、点更新・区間積）。`i64` の max/min/sum コンストラクタ付き |
| `ds::segtree_2d` | 2D Segment Tree（点更新・矩形区間積） |
| `ds::lazy_segtree` | 遅延伝播セグメント木（ACL 準拠、区間作用・区間積）。`i64` の区間加算/区間更新 × max/min/sum ラッパー付き |
| `ds::dynamic_segtree` | 動的セグメント木（大きな添字範囲での疎な点更新・区間積） |
| `ds::segtree_beats` | Segment Tree Beats（区間 chmin/chmax・区間加算・区間和、`i64`） |
| `ds::sparse_table` | Sparse Table（静的区間 min/max O(1)） |
| `ds::wavelet_matrix` | Wavelet Matrix（区間頻度・k 番目・range_freq、値域 `u64`） |
| `ds::implicit_treap` | Implicit Treap（挿入・削除・分割・結合 O(log n) の可変長列） |
| `ds::slope_trick` | Slope Trick（区分線形凸関数の最小値管理） |
| `ds::multiset` | 多重集合（BTreeMap ベース、順序境界つき） |
| `ds::median_set` | 中央値集合（2 ヒープ。中央値・絶対偏差和） |
| `ds::binary_trie` | Binary Trie（xor 最小/最大・k 番目） |
| `ds::segment_set` | 区間の集合管理（マージ・所属判定・mex） |
| `ds::convex_hull_trick` | Convex Hull Trick（傾き単調追加・最小値クエリ）。任意順挿入は `ds::li_chao_tree` |
| `ds::li_chao_tree` | Li Chao Tree（直線/線分の任意順挿入・1 点最小値） |
| `ds::bitset` | 可変長 bitset（シフト・論理演算） |
| `graph::bfs` | BFS（重みなし最短路） |
| `graph::dfs` | 非再帰 DFS（訪問順・到達判定、重み付き隣接リスト対応） |
| `graph::dijkstra` | ダイクストラ（距離・経路復元・2 番目に短い距離） |
| `graph::bellman_ford` | ベルマンフォード（負辺最短路・負閉路検出） |
| `graph::warshall_floyd` | ワーシャルフロイド（全点対最短路） |
| `graph::mst` | 最小全域木（Kruskal）。`ds::dsu` に依存 |
| `graph::topo_sort` | トポロジカルソート（Kahn。閉路検出付き） |
| `graph::scc` | 強連結成分分解（Kosaraju、トポロジカル順） |
| `graph::two_sat` | 2-SAT（充足判定と割り当て構成）。`graph::scc` に依存 |
| `graph::lowlink` | 橋・関節点（lowlink） |
| `graph::components` | 無向グラフの連結成分分解（成分 ID・成分リスト・連結判定） |
| `graph::cycle_detection` | 有向/無向グラフの閉路検出と復元 |
| `graph::two_coloring` | 二部グラフ判定・2 彩色 |
| `graph::bipartite_matching` | 二部グラフ最大マッチング（Kuhn の増加路法） |
| `graph::max_flow` | 最大流（Dinic） |
| `graph::min_cost_flow` | 最小費用流 |
| `graph::grid_bfs` | グリッド上の 4 近傍 BFS・多始点 BFS・01 BFS |
| `graph::grid_components` | グリッド上の 4 近傍連結成分分解・連結判定 |
| `graph::lca` | 最小共通祖先（ダブリング。距離・k 個上の祖先） |
| `graph::euler_tour` | オイラーツアー（部分木の区間対応、スタック安全） |
| `graph::eulerian_path` | 有向/無向グラフのオイラー路・閉路構成 |
| `graph::hld` | Heavy-Light Decomposition（LCA・パス/部分木区間分解） |
| `graph::rerooting` | 全方位木 DP（rerooting DP） |
| `graph::auxiliary_tree` | Auxiliary Tree（virtual tree。指定頂点集合＋LCA の圧縮木） |
| `graph::centroid` | 木の重心・重心分解（centroid decomposition） |
| `graph::tree_diameter` | 木の直径（2 回 BFS/DFS） |
| `graph::doubling` | ダブリング（functional graph で k 個先を O(log k)） |
| `graph::functional_graph` | Functional graph のサイクル検出 |
| `graph::retrograde_analysis` | 後退解析（ゲームグラフの勝ち/負け/引き分け判定） |
| `geometry::basic` | 整数座標の基本幾何（点・外積・ccw・線分交差・多角形面積） |
| `geometry::angle_sort` | 原点まわりの偏角ソート（整数点は外積比較） |
| `geometry::convex_hull_int` | 整数座標凸包（Andrew monotone chain） |
| `geometry::float` | 浮動小数点幾何（点・直線・円・凸包・最近点対） |
| `geometry::rectangle_union` | 軸平行半開矩形の和集合面積（座標圧縮スイープライン） |
| `string::rolling_hash` | ローリングハッシュ（mod 2^61-1） |
| `string::z_algorithm` | Z-algorithm |
| `string::kmp` | MP/KMP（失敗関数・パターン検索・最小周期） |
| `string::manacher` | Manacher（全中心の回文半径・最長回文） |
| `string::suffix_array` | 接尾辞配列（O(n log² n)）＋ LCP 配列（Kasai） |
| `string::lcs` | 最長共通部分列 |
| `string::trie` | トライ木 |
| `string::run_length_encoding` | ランレングス圧縮 |
| `string::substring` | 文字インデックス半開区間からの部分文字列取得 |
| `dp::digit_dp` | 桁 DP（上限以下の整数の桁和分布・個数・総和） |
| `dp::subset_sum` | 部分和 DP |
| `dp::binomial_distribution` | 公平な二項分布の確率表・確率・累積確率 |
| `dp::iwi` | `iwi` 型消去ルールの最大消去長を求める区間 DP |
| `misc::io` | 高速入力スキャナ（stdin 一括読み・型付き read/vec、マクロ不使用） |
| `misc::xorshift` | 軽量擬似乱数（xorshift64、シード再現可能） |
| `misc::ordered_float` | 全順序 f64 ラッパ（BinaryHeap/ソート用） |
| `misc::recursive` | マクロを使わない再帰クロージャ・メモ化再帰クロージャ |

今後の拡張候補（高度典型 154 ＋ 赤レベル 110 ＋ 世界最上位 30 ＝ 294 個）は
[`.log/Fable_Candidates.md`](./.log/Fable_Candidates.md)、監査レポートは
[`.log/Fable_Report.md`](./.log/Fable_Report.md) を参照。方針は [`AGENTS.md`](./AGENTS.md) を参照。

## ライセンス

個人用ライブラリ。自由に利用可。
