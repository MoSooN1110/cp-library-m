# Fable Report — cplib 監査 (2026-07-12)

Fable 5 による cp-library (cplib) の全体監査。観点は「使いやすさ・強さ・美しさ」。
この監査に基づく変更は本レポートの末尾「実施計画」に従って同日中に着手する。

## 総評

**基盤は非常に健全。** 213 unit test + 97 doctest が全緑、AGENTS.md の規約
（1 概念 1 ファイル / 自己完結 / crate:: 参照による依存解決 / doctest 必須 / expander 両立）が
実際に守られており、`_pending/` の移行も完了している。モジュール数 90 超で
AtCoder 水〜橙帯の典型はほぼカバーしている。

一方で **「コンテスト本番の最初の 30 秒」を支える道具（入力スキャナ）が無い**こと、
文字列・グラフ・数論に古典的な欠落が数点あることが「強さ」の穴。
API の一貫性はおおむね良好（未コミットの segtree/lazy_segtree i64 ラッパーは良い方向）。

## 良い点（維持すべき資産）

- **規約と CI が機械的に守られる構造**: expander が `crate::cat::name` 参照を推移解決する
  設計は、cargo のコンパイル検査と依存解決を同一のソースから得ており美しい。
- **テスト文化**: ほぼ全モジュールにナイーブ比較 or ランダムテストがある。
- **統合の判断**: wavelet_matrix×4 → 1 本、hld×2 → 1 本など、canonical 実装への
  統合が済んでいる。dijkstra_top2 を別モジュールにせず dijkstra に統合したのも正しい。
- **未コミット作業（segtree/lazy_segtree の i64 ラッパー）**: `RangeAddSum::add/sum` の
  ように定型用途を短く書ける。方向性は正しく、そのまま活かす。

## 指摘事項

### A. 欠落ライブラリ（優先度順）

| # | モジュール | 理由 |
|---|---|---|
| A1 | `misc::io`（高速入力スキャナ） | 最大の使いやすさの穴。現状、提出のたびに手書きの read が必要。proconio 相当をマクロなし（expander 安全）で提供する |
| A2 | `graph::two_sat` | 典型頻出。既存の `graph::scc` を再利用でき、追加コストが小さい |
| A3 | `ds::li_chao_tree` | 既存 CHT は**傾き単調追加限定**。任意順の直線/線分挿入を補完する |
| A4 | `string::manacher` | 回文半径。旧アーカイブにはあるが新体系に未移行 |
| A5 | `string::kmp` | MP/KMP 失敗関数・周期・検索。Z とは使い分けがある |
| A6 | `math::lagrange_interpolation` | f(0..=n) 標本から f(x)。Σi^k 系や DP 多項式化で頻出 |
| A7 | `ds::rollback_dsu` | undo 付き UF。オフライン動的連結性・Mo on tree の部品 |
| A8 | `graph::eulerian` | オイラー路/閉路（有向・無向、Hierholzer） |
| A9 | `math::sqrt_mod` | Tonelli–Shanks。平方剰余は稀だが自作は事故りやすい |
| A10 | `math::discrete_log` | BSGS。同上 |
| A11 | `algo::mo` | Mo's algorithm の枠組み（add/remove クロージャ駆動） |
| A12 | `string::aho_corasick` | 複数パターン照合。trie があるので体系上も自然な拡張 |

このほか将来候補（今回は見送り）: FPS の sqrt/Taylor shift/合成、持続化データ構造、
Dinic の scaling、一般マッチング、SMAWK、オンライン畳み込み。

### B. リファクタリング / 一貫性

- **B1. `ds::convex_hull_trick` の doc に制約を明記**: 「傾き降順・クエリ任意（内部三分探索）」
  という制約は doc 先頭 1 行にあるが、Li Chao 追加後は相互参照を書き、使い分けを明確にする。
- **B2. `examples/sample_main.rs` の肥大化**: 新モジュールを足すたびに main が伸びる。
  expander smoke の目的には「代表的な依存パターン（依存推移・型パラメータ・マクロなし）」が
  踏めていれば十分。今回追加分は代表 2–3 個（io, two_sat あたり）だけ足す。
- **B3. README 収録表の欠落**: `graph::{bellman_ford,bfs?,lca,euler_tour,lowlink,max_flow,min_cost_flow,mst,topo_sort,tree_diameter,two_coloring,warshall_floyd,doubling,functional_graph,bipartite_matching,grid_components}`、
  `ds::{cumsum,cumsum_2d,weighted_dsu,sparse_table,binary_trie,segment_set,convex_hull_trick,median_set,bitset,fenwick 系}`、
  `string::{lcs,run_length_encoding,suffix_array,trie}`、`math::{digit_sum,diophantine,divisors,euler_phi,floor_sum,garner,gauss_xor,lucas,number 一部,ratio,xor_basis}`、
  `algo::{binary_search,chmin_chmax,compress,inversion,lis,max_rectangle,next_permutation,product,ternary_search}`、
  `misc::{ordered_float,xorshift}`、`dp::subset_sum` が表に載っていない。
  **README の表は「全収録モジュールの索引」として全行埋める**（半端な表は索引として機能しない）。
- **B4. INDEX.md の役目終了**: `_pending/` が空になったので、INDEX.md は削除するか
  「移行完了」の 1 行に置き換える（README からの参照も更新）。
- **B5. 命名の軽微な揺れ**: `from_slice_range_add_sum` はやや冗長だが、
  ジェネリック `from_slice` と衝突しないための明示的命名であり許容。変更しない。

### C. 触らないもの

- `_archive_*` は規約どおり不変。
- 既存モジュールの API 破壊は行わない（提出済みコードとの互換のため）。
- 未コミットの worktree 変更（segtree ラッパー等）は前セッションの成果。巻き戻さない。

## 実施計画（本セッション）

1. A1–A12 を優先度順に `src/` へ追加（各: doctest + 単体テスト、AGENTS.md 規約順守）。
2. B1（CHT doc 相互参照）、B2（sample_main へ代表を最小追加）。
3. B3: README 収録表を全モジュールで埋める。B4: INDEX.md を整理。
4. `cargo test --lib` / doctest / expander smoke を緑にして完了報告。
   コミットはユーザー判断に委ねる（worktree に積む）。

## 実施結果（同日追記）

ユーザー指示により方針変更: 実装は 7 本で区切り、以降は候補リスト化を優先した。

- **追加済みモジュール（各 doctest + ナイーブ比較/ランダムテスト付き）**:
  `misc::io`（A1）, `graph::two_sat`（A2）, `ds::li_chao_tree`（A3）,
  `string::manacher`（A4）, `string::kmp`（A5）, `math::lagrange_interpolation`（A6）,
  `ds::rollback_dsu`（A7）。A8–A12 は未着手（候補リストに収録済み）。
- **B1–B4 完了**: CHT↔LiChao 相互参照 / sample_main に io・two_sat・manacher 追加
  （scc への依存推移も smoke で検証）/ README 収録表を全 104 モジュールに拡充 /
  INDEX.md を「移行完了」化。
- **候補リスト**: [`Fable_Candidates.md`](./Fable_Candidates.md) に
  高度典型 154 ＋ 赤レベル 110 ＋ 世界最上位 30 ＝ **計 294 候補** を優先度・依存・
  参考リンク付きで整理（ユーザー要望による拡張）。
- **検証**: `cargo test --lib` 230 passed / doctest 104 passed / expander smoke
  （展開 → rustc 単体コンパイル → 実行）全て成功。worktree は未コミット
  （コミットはユーザー判断）。
