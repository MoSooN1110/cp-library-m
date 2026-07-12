# cplib 拡張候補リスト — 高度典型 154 選＋赤レベル 110 選＋世界最上位 30 選 (2026-07-12, Fable)

「高度典型」（水〜橙以上で問われる応用典型）を中心に、cplib に**まだ無い**追加候補を
カテゴリ別に列挙する。出典体系: Library Checker (yosupo) の問題群 / cp-algorithms /
Nyaan・ei1333(Luzhiled)・maspy 各ライブラリの索引 / AtCoder 高難度回の頻出テク。
実装はゆっくり進める前提で、優先度は次の 4 段階。

- **S**: 出題頻度が高く効果大。次に実装すべき
- **A**: 頻出。中期で揃えたい
- **B**: 時々出る。余裕があれば
- **C**: 稀・保険。必要になった時でよい

状態欄: `—`=未着手 / `部分`=近い機能が既存 / 済モジュールはこの表に載せない。
本日追加済み: `misc::io`, `graph::two_sat`, `ds::li_chao_tree`, `ds::rollback_dsu`,
`string::manacher`, `string::kmp`, `math::lagrange_interpolation`（→ README 参照）。

## 1. データ構造 (ds) — 32

| # | 候補 | 概要 | 優先度 | 備考 |
|---|---|---|---|---|
| 1 | sliding_window_min | 単調 deque による区間最小/最大 O(1) 償却 | **S** | 基本典型だが未収録 |
| 2 | swag | Foldable queue（半群の窓集約。min 以外も可） | **S** | #1 の一般化 |
| 3 | disjoint_sparse_table | 冪等でない半群の静的区間積 O(1) | A | sparse_table 補完 |
| 4 | merge_sort_tree | 区間内の値 ≤ x 数え上げ（各ノードにソート列） | B | wavelet で代替可 |
| 5 | fenwick_2d | 2D BIT（点加算・矩形和） | A | segtree_2d より軽い |
| 6 | rect_add_rect_sum | 矩形加算・矩形和（2D imos / オフライン BIT） | B | |
| 7 | persistent_segtree | 永続セグ木（区間 k-th smallest、過去版参照） | A | |
| 8 | persistent_array | 永続配列（永続 DSU の部品） | C | |
| 9 | persistent_dsu | 永続 Union-Find | C | #7,#8 依存 |
| 10 | offline_dynamic_connectivity | 時間セグ木 + undo DSU で辺の追加削除に答える | A | rollback_dsu 済で部品あり |
| 11 | link_cut_tree | 動的木（パス集約・辺の付け替え） | B | 実装重 |
| 12 | euler_tour_tree | 動的木（部分木集約） | C | |
| 13 | static_top_tree | 静的 top tree（部分木 DP の一点更新高速化） | B | 近年頻度上昇 |
| 14 | meldable_heap | 併合可能ヒープ（skew/leftist） | B | archive に skew あり |
| 15 | interval_heap | 両端優先度付きキュー | C | archive にあり |
| 16 | removable_heap | 遅延削除付きヒープ 2 本組 | B | multiset で代替可 |
| 17 | dynamic_cht | LineContainer（BTree、傾き挿入任意・クエリ任意） | A | li_chao と使い分け |
| 18 | kd_tree | K-D tree(最近点・矩形内列挙) | C | |
| 19 | fast_int_set | 64 分木 integer set（O(w/6)、BTreeSet 高速代替） | B | |
| 20 | sortable_segtree | 区間ソートクエリ | C | |
| 21 | wavelet_range_sum | Wavelet Matrix + 累積和（区間 k 小さい方の和） | B | 既存 WM 拡張 |
| 22 | binary_trie_xor_all | Binary Trie に全体 xor 作用・k-th 取得 | B | 既存拡張、部分 |
| 23 | mo_algorithm | Mo 法の枠組み（add/remove クロージャ駆動） | **S** | algo 配置でも可 |
| 24 | mo_rollback | 回すだけ Mo（削除なし・rollback） | B | #23 派生 |
| 25 | mo_on_tree | 木上の Mo（Euler tour + パスクエリ） | B | #23 派生 |
| 26 | sqrt_decomposition | 平方分割の汎用バケット枠組み | B | |
| 27 | dsu_bipartite | 二部性判定つき DSU（オンライン奇閉路検出） | A | weighted_dsu(mod 2) で部分 |
| 28 | dsu_group | 一般群のポテンシャル付き DSU | C | weighted_dsu の一般化 |
| 29 | sparse_table_2d | 2D 静的矩形 min | C | |
| 30 | monotone_cht_o1 | クエリも単調な CHT の O(1) ポインタ版 | C | 既存 CHT 拡張 |
| 31 | queue_undo | undo 可能キュー / queue aggregation | C | |
| 32 | top2_tracker | 種類別 top2 管理（rerooting 等の部品） | C | |

## 2. グラフ (graph) — 38

| # | 候補 | 概要 | 優先度 | 備考 |
|---|---|---|---|---|
| 33 | eulerian_path | オイラー路/閉路（有向・無向, Hierholzer） | **S** | |
| 34 | cycle_detection | 閉路検出と復元（有向・無向） | **S** | 意外と未収録 |
| 35 | block_cut_tree | 二重頂点連結成分分解 + block-cut tree | A | lowlink 済で部品あり |
| 36 | two_edge_cc | 二重辺連結成分縮約（bridge tree） | A | 同上 |
| 37 | dominator_tree | 支配木（Lengauer–Tarjan） | B | |
| 38 | hopcroft_karp | 二部マッチング O(E√V)（既存の高速化） | A | 既存は Kuhn 系 |
| 39 | hungarian | 重み付き二部マッチング（割当問題） O(n³) | A | |
| 40 | general_matching | 一般グラフ最大マッチング（blossom） | B | |
| 41 | mcf_negative | 負辺初期ポテンシャル対応 min cost flow | A | 既存 MCF 拡張 |
| 42 | flow_with_lower_bounds | 下限付き流量 (b-flow) の変形 | B | max_flow 上のヘルパ |
| 43 | project_selection | 燃やす埋める定式化ヘルパ | B | max_flow 応用 |
| 44 | gomory_hu | 全点対最小カット木 | C | |
| 45 | stoer_wagner | 全域最小カット | C | |
| 46 | arborescence | 最小有向全域木（Chu–Liu/Edmonds） | B | |
| 47 | k_shortest_paths | k 最短路（Yen / Eppstein） | C | |
| 48 | complement_bfs | 補グラフ上の BFS/連結成分（未使用頂点 set） | B | |
| 49 | dial | Dial 法（小整数重み Dijkstra） | C | |
| 50 | johnson | 全点対最短路（負辺、n 回 Dijkstra） | C | warshall で代替可 |
| 51 | min_mean_cycle | 最小平均長閉路 | C | |
| 52 | steiner_tree | 部分集合 DP の Steiner 木 O(3^k n) | B | |
| 53 | max_independent_set | 最大独立集合（n≤40 半分全列挙） | B | |
| 54 | max_clique | 最大クリーク（分枝限定） | C | |
| 55 | chromatic_number | 彩色数 O(2^n n) | C | |
| 56 | triangle_enumeration | 三角形列挙 O(E√E) | B | |
| 57 | three_edge_cc | 三重辺連結成分 | C | |
| 58 | tree_hash | 木ハッシュ（根付き/自由木の同型判定） | A | |
| 59 | pruefer | Prüfer 列の相互変換 | B | |
| 60 | matrix_tree | 行列木定理（全域木数え上げ） | B | mod 行列式(#91)依存 |
| 61 | namori | ナモリグラフ分解（サイクル+木部分） | A | functional_graph 拡張 |
| 62 | min_path_cover | DAG 最小パス被覆（マッチング帰着ヘルパ） | C | |
| 63 | dag_longest_path | DAG 最長路（topo+DP ヘルパ） | B | 半自明だが頻出 |
| 64 | lexicographic_topo | 辞書順最小トポロジカル順序 | C | topo_sort 拡張 |
| 65 | offline_lca_tarjan | オフライン LCA | C | 既存 doubling で足りる |
| 66 | auxiliary_tree_dp | 補助木上 DP のテンプレ | C | auxiliary_tree 応用例 |
| 67 | two_sat_at_most_one | 高々 1 つ true 制約の O(n) エンコード | C | two_sat 拡張 |
| 68 | eulerian_lexicographic | 辞書順最小オイラー路 | C | #33 派生 |
| 69 | centroid_contour | 重心分解の等高線集約（距離 k 集計） | B | centroid 応用 |
| 70 | virtual_tree_queries | クエリ毎の圧縮木テンプレ | C | auxiliary_tree 応用 |

## 3. 数論 (math) — 20

| # | 候補 | 概要 | 優先度 | 備考 |
|---|---|---|---|---|
| 71 | sqrt_mod | 平方剰余 Tonelli–Shanks | A | |
| 72 | discrete_log | BSGS（非互質 mod 対応） | A | |
| 73 | primitive_root | 原始根 | A | NTT 素数以外の畳み込みにも |
| 74 | multiplicative_order | 位数 | B | #73 と同居可 |
| 75 | jacobi_symbol | ヤコビ記号 | C | #71 と同居可 |
| 76 | kth_root_mod | mod p の k 乗根 | C | #72,#73 依存 |
| 77 | quotient_ranges | 商列挙（floor(n/i) が一定の区間分割） | **S** | 数論頻出の割に未収録 |
| 78 | min_25_sieve | 乗法的関数の総和 O(n^{3/4}/log) | C | prime_count 済で部品 |
| 79 | totient_sum | Σφ / Mertens（Dirichlet 前処理付き漸化） | C | |
| 80 | segmented_sieve | 区間篩（大きい n の区間素数列挙） | B | |
| 81 | min_factor_table | 最小素因数表による高速素因数分解列挙 | A | archive にあり移行 |
| 82 | stern_brocot | Stern–Brocot 木上の探索（最良有理近似） | B | |
| 83 | continued_fraction | 連分数展開 | C | |
| 84 | pell | Pell 方程式 | C | |
| 85 | factorial_prime_power | n! の素因数 p の指数 / n! mod p^e | B | 巨大 nCr 用 |
| 86 | granville_binom | 一般 mod の nCr（素数冪 + CRT） | C | lucas 済の拡張 |
| 87 | isqrt | 128bit 安全な整数平方根・k 乗根 | B | 小粒だが事故防止 |
| 88 | carmichael | Carmichael λ 関数 | C | |
| 89 | tetration_mod | a↑↑b mod m | C | |
| 90 | rational_approx | 分数のオーバーフロー安全比較・近似 | C | ratio 済の拡張 |

## 4. 線形代数・FPS (math) — 24

| # | 候補 | 概要 | 優先度 | 備考 |
|---|---|---|---|---|
| 91 | matrix_mod | mod p 行列: 行列式・rank・逆行列・連立一次 | **S** | 現状 gauss は f64 のみ |
| 92 | matrix_semiring | 一般半環行列累乗（min-plus で最短路 k 回等） | A | matrix は Mint 専用 |
| 93 | char_poly | 特性多項式（Hessenberg 化） | C | |
| 94 | f2_rank_solve | F2 行列の rank・解空間（bitset gauss） | B | gauss_xor 拡張 |
| 95 | simplex | 線形計画（単体法） | C | |
| 96 | adaptive_simpson | 適応 Simpson 数値積分 | B | misc 配置 |
| 97 | fps_sqrt | FPS の平方根 | B | sqrt_mod(#71) 依存 |
| 98 | fps_taylor_shift | f(x+c) の係数 O(n log n) | B | |
| 99 | fps_composition | FPS 合成・逆関数 | C | 実装重 |
| 100 | multipoint_eval | 多点評価（subproduct tree） | B | |
| 101 | poly_interpolation | n 点補間 O(n log² n) | C | #100 依存 |
| 102 | subset_convolution | 部分集合畳み込み O(2^n n²) | B | hadamard 済の上位 |
| 103 | divisor_transform | 約数/倍数系 zeta–moebius・gcd/lcm 畳み込み | A | |
| 104 | dirichlet | Dirichlet 畳み込み・冪 | C | |
| 105 | relaxed_convolution | オンライン畳み込み | C | |
| 106 | chirp_z | Chirp-Z 変換（任意長・任意点等比 DFT） | C | |
| 107 | ntt_triple_prime | 3 素数 NTT + garner の任意 mod 畳み込み | B | fft 15bit 済の精度保険 |
| 108 | stirling | スターリング数 第1種/第2種の行を O(n log n) | B | |
| 109 | bernoulli | ベルヌーイ数（FPS） | C | |
| 110 | partition_number | 分割数 P(n)（五角数定理） | B | |
| 111 | bell | ベル数 | C | |
| 112 | power_sums | Σ i^k のヘルパ（lagrange 応用の定型化） | C | lagrange 済で代替可 |
| 113 | q_analog | q-二項係数 | C | |
| 114 | matroid_intersection | マトロイド交差 | C | 上級保険 |

## 5. 文字列 (string) — 12

| # | 候補 | 概要 | 優先度 | 備考 |
|---|---|---|---|---|
| 115 | aho_corasick | 複数パターン同時照合（trie+失敗リンク） | A | trie 済で自然な拡張 |
| 116 | suffix_automaton | 接尾辞オートマトン（部分文字列の万能装置） | B | |
| 117 | eertree | 回文木（相異なる回文部分文字列） | B | |
| 118 | lyndon | Lyndon 分解（Duval） | B | |
| 119 | min_rotation | 最小回転（Booth） | B | |
| 120 | sa_is | SA-IS（O(n) 接尾辞配列） | C | 既存 O(n log² n) で足りる |
| 121 | lcp_range | 任意 2 接尾辞の LCP（SA + sparse table） | A | 既存 SA/LCP のヘルパ |
| 122 | distinct_substrings | 相異なる部分文字列数 | B | SA/LCP ヘルパ |
| 123 | longest_common_substring | 2 文字列の最長共通部分文字列 | B | SA or hash |
| 124 | wildcard_match | ワイルドカード付き照合（FFT） | C | |
| 125 | dynamic_rolling_hash | 1 点更新対応ハッシュ（セグ木 affine） | C | |
| 126 | runs | run（極大周期部分文字列）列挙 | C | |

## 6. 幾何 (geometry) — 12

| # | 候補 | 概要 | 優先度 | 備考 |
|---|---|---|---|---|
| 127 | convex_hull_int | 整数座標凸包（Andrew, 退化対応） | **S** | float 版しか無い |
| 128 | rotating_calipers | 最遠点対・凸多角形の幅 | A | #127 依存 |
| 129 | point_in_polygon | 点の多角形内判定（整数, 境界区別） | A | |
| 130 | convex_contains | 凸多角形内判定 O(log n) | B | |
| 131 | convex_cut | 凸多角形の直線切断 | B | |
| 132 | halfplane_intersection | 半平面交差 | B | |
| 133 | min_enclosing_circle | 最小包含円（期待 O(n)） | B | |
| 134 | minkowski_sum | 凸多角形の Minkowski 和 | B | |
| 135 | circle_tangents | 円の共通接線・円と点の接線 | C | float 拡張 |
| 136 | pick | Pick の定理・線分上の格子点数 | B | 小粒 |
| 137 | dynamic_convex_hull | 動的凸包（BTree 上下包） | C | |
| 138 | geometry_3d | 3D 基本（外積・平面・凸包） | C | |

## 7. DP・アルゴリズム技法 (algo/dp) — 10

| # | 候補 | 概要 | 優先度 | 備考 |
|---|---|---|---|---|
| 139 | divide_conquer_opt | 分割統治 DP 最適化（決定単調性） | A | |
| 140 | knuth_opt | Knuth 最適化（区間 DP O(n²)） | B | |
| 141 | aliens_trick | Alien DP（個数制約をペナルティ二分探索へ） | A | 枠組み+例 |
| 142 | monotone_minima | monotone minima / SMAWK | B | #139 の部品 |
| 143 | parallel_binary_search | 並列二分探索 | B | |
| 144 | zobrist | Zobrist ハッシュ（集合の一致判定） | B | 小粒 |
| 145 | meet_in_middle | 半分全列挙ヘルパ（部分和列挙+二分） | B | |
| 146 | automaton_dp | オートマトン合成型の汎用桁 DP | B | digit_dp は特化型 |
| 147 | offline_query_sort | クエリ平方分割/イベントソートの定型 | C | |
| 148 | exact_cover | Dancing Links（数独型厳密被覆） | C | |

## 8. その他 (misc) — 6

| # | 候補 | 概要 | 優先度 | 備考 |
|---|---|---|---|---|
| 149 | dynamic_modint | 実行時 MOD の modint | A | 現状はソース書換えで切替 |
| 150 | modint_const_generic | `Mint<const M>` 化（998244353/1e9+7 併存） | B | #149 と択一。API 互換注意 |
| 151 | printer | 高速出力（BufWriter ラッパ、Vec 一括出力） | B | io 拡張 |
| 152 | timer | 経過時間管理（ヒューリスティック用） | C | |
| 153 | annealing | 焼きなまし/ビームサーチ枠組み | C | マラソン用 |
| 154 | neighbors | グリッド 4/8 近傍イテレータ | A | archive の adjacent4/8 を現代化 |

## 9. 赤レベル（世界レベル）— 110

AtCoder 赤〜、ICPC WF・CodeForces Div.1 終盤で問われる領域。優先度は原則 C 扱い
（問題駆動で必要になった時に実装）だが、⭐ を付けたものは日本の赤コーダー環境では
出題実績が比較的多く、先行投資の価値がある。

| # | 候補 | 概要 | 備考 |
|---|---|---|---|
| 155 | top_tree | 動的木でパス・部分木の両方を集約 | link-cut の上位互換、実装最重量級 |
| 156 | lct_subtree | 部分木クエリ対応 link-cut tree | #155 の軽量代替 |
| 157 | retroactive_ds | 遡及データ構造（過去の操作列を編集） | |
| 158 | kinetic_segment_tree | Kinetic Segment Tree（一次関数群の min 追跡） | |
| 159 | persistent_treap | 永続平衡二分木（列の永続分割・結合） | |
| 160 | hlpp | Push-Relabel / HLPP 最大流（Dinic が落ちる制約用） | |
| 161 | network_simplex | ネットワーク単体法（大規模最小費用流・凸費用） | |
| 162 | weighted_blossom | 重み付き一般マッチング | 実装最難関の一つ |
| 163 | dulmage_mendelsohn | DM 分解（二部グラフの一意被覆構造） | ⭐ |
| 164 | vertex_connectivity | 頂点連結度（flow 反復） | |
| 165 | bipartite_edge_coloring | 二部グラフ辺彩色 O(E log E) | |
| 166 | max_density_subgraph | 最大密度部分グラフ（分数計画＋flow） | |
| 167 | arboricity | 森林分解 / Nash-Williams 密度 | |
| 168 | planarity | 平面性判定・平面埋め込み | |
| 169 | half_gcd | 多項式 half-GCD O(n log² n)（有理関数近似） | |
| 170 | fps_composition_fast | FPS 合成・逆関数 O(n log n)（Kinoshita–Li） | ⭐ 近年の定番化が進む |
| 171 | power_projection | べき乗射影（合成の双対） | #170 と対 |
| 172 | poly_matrix_det | 多項式行列の行列式 | |
| 173 | p_recursive | ホロノミック数列の第 n 項（分数なし Bostan-Mori 拡張） | |
| 174 | factorial_sqrt | n! mod p を O(√p log p)（多点評価） | ⭐ |
| 175 | multivariate_conv | 多変数切断冪級数の畳み込み | |
| 176 | sparse_fps | 疎 FPS の pow/exp/log O(nk) | ⭐ |
| 177 | lll | LLL 格子基底簡約 | |
| 178 | poly_factorize | F_p 上の多項式因数分解（Berlekamp / Cantor-Zassenhaus） | |
| 179 | ecm | 楕円曲線法の素因数分解（64bit 超） | |
| 180 | nimber | Nim 積・Nim 冪（xor 畳み込みの nimber 版） | |
| 181 | lgv | LGV 補題ヘルパ（非交差経路の行列式） | ⭐ 要 #91 |
| 182 | permanent | パーマネント（Ryser O(2^n n)） | |
| 183 | burnside | Burnside / Polya 数え上げの枠組み | ⭐ 橙帯でも出る |
| 184 | larsch | LARSCH / オンライン monge DP 最適化 | ⭐ |
| 185 | minplus_concave | min-plus 畳み込み（凹列, SMAWK） | ⭐ |
| 186 | slope_trick_tree | 木上 slope trick（マージ可能な区分線形凸） | |
| 187 | kinetic_tournament | Kinetic Tournament | |
| 188 | ukkonen | 接尾辞木のオンライン構築 | |
| 189 | generalized_sam | 複数文字列の一般化接尾辞オートマトン | |
| 190 | lz_factorization | Lempel-Ziv 分解 | |
| 191 | myers_bitparallel | ビット並列編集距離（Myers） | |
| 192 | delaunay | Delaunay 三角形分割 / Voronoi 図 | |
| 193 | convex_hull_3d | 3D 凸包 | |
| 194 | bentley_ottmann | 線分交差列挙スイープ | |
| 195 | segment_tree_merge | セグ木マージ（部分木ごとの値集合を O(n log n) で統合） | ⭐ 木上の集合 DP で頻出 |
| 196 | persistent_lazy_segtree | 永続遅延セグメント木 | |
| 197 | dynamic_wavelet | 動的 Wavelet（挿入・削除つき rank/select/k-th） | |
| 198 | fractional_cascading | Fractional cascading（多列二分探索の O(log n) 化） | |
| 199 | sqrt_tree | Sqrt Tree（半群の静的区間積 O(1)・前計算 O(n log log n)） | |
| 200 | online_dynamic_connectivity | オンライン動的連結性（Holm–de Lichtenberg–Thorup） | ⭐ |
| 201 | chan_dynamic_hull | 完全動的凸包（挿入・削除 O(log² n)） | |
| 202 | range_mode_query | 区間最頻値（平方分割 O(n√n)） | |
| 203 | succinct_rmq | 簡潔 RMQ（±1RMQ、2n+o(n) bit・クエリ O(1)） | |
| 204 | xor_mst | XOR 最小全域木（Borůvka + binary trie） | ⭐ 出題実績多数 |
| 205 | dynamic_msf_offline | オフライン動的最小全域森（時間分割統治 + LCT/undo） | ⭐ |
| 206 | tutte_matrix | Tutte 行列による乱択一般マッチング | |
| 207 | karger_stein | 乱択全域最小カット（Karger–Stein） | |
| 208 | suurballe | 辺素な 2 本の最短路対（Suurballe） | ⭐ |
| 209 | spqr_tree | SPQR tree（3 連結成分の構造木） | |
| 210 | cactus | サボテングラフの構造分解・マッチング | |
| 211 | treewidth_dp | 小木幅グラフの木分解と DP（nice tree decomposition） | ⭐ |
| 212 | chordal | 弦グラフ認識（LexBFS・完全消去順序・彩色） | |
| 213 | best_theorem | BEST 定理（有向オイラー閉路の数え上げ） | ⭐ 要 #60 |
| 214 | degree_sequence | 次数列の実現可能性（Erdős–Gallai）と構成（Havel–Hakimi） | |
| 215 | negative_cycle_canceling | 負閉路消去による最小費用流（負閉路許容） | |
| 216 | t_join | T-join・一般グラフの中国人郵便配達 | |
| 217 | parity_shortest_path | 偶奇制約付き最短路（奇路/偶路） | |
| 218 | graph_isomorphism | 一般グラフ同型判定（canonical labeling） | |
| 219 | bitset_reachability | DAG 到達可能性の bitset 圧縮（O(nm/64)） | |
| 220 | hitting_time | ランダムウォークの到達期待値（連立一次方程式） | 要 #91 |
| 221 | minimum_ratio_cycle | 最小比率閉路（Dinkelbach + Bellman–Ford） | |
| 222 | pohlig_hellman | Pohlig–Hellman（位数が滑らかな離散対数） | 要 #72 |
| 223 | cornacchia | Cornacchia 法（x²+dy²=p、二平方和分解） | ⭐ |
| 224 | smith_normal_form | Smith 標準形（整数行列、アーベル群構造） | |
| 225 | bareiss | Bareiss 法（分数なし整数行列式） | |
| 226 | wiedemann | 黒箱線形代数（疎行列の最小多項式・連立・行列式） | ⭐ |
| 227 | resultant | 終結式・部分終結式（多項式の共通根判定） | |
| 228 | newton_identities | Newton の恒等式（冪和 ⇔ 基本対称式の変換） | |
| 229 | cyclotomic_poly | 円分多項式 | |
| 230 | euler_transform | オイラー変換（多重集合構成の母関数変換） | ⭐ 分割数系の一般化 |
| 231 | sample_point_shift | 標本点シフト（f(0..n) から f(c..c+n) を O(n log n)） | ⭐ #174 の部品 |
| 232 | falling_factorial_base | 下降冪基底との相互変換 | |
| 233 | pade | Padé 近似・有理関数再構成 | 要 #169 |
| 234 | powerful_number_trick | Powerful number trick（乗法的関数の総和） | 要 #78 系 |
| 235 | dirichlet_hyperbola | Dirichlet 双曲線法（Σ f*g の O(√n) 評価） | |
| 236 | squarefree_count | 無平方数の数え上げ（Möbius + 篩） | |
| 237 | lattice_count_cf | 直線下の格子点数の連分数法（一般化 floor_sum: Σ x^a y^b） | ⭐ |
| 238 | bigint | 多倍長整数（FFT 乗算・除算・基数変換） | 赤レベル数論の土台 |
| 239 | frobenius_form | Frobenius 標準形（行列累乗 O(n³ + n² log k)） | |
| 240 | sum_of_geom_poly | Σ r^i · i^d の閉形式（等比×多項式和） | |
| 241 | lagrange_inversion | ラグランジュ反転公式（陰関数の係数抽出） | ⭐ |
| 242 | binary_splitting | 分割統治による級数・階乗の厳密評価 | 要 #238 |
| 243 | cdawg | CDAWG(圧縮有向非巡回語グラフ) | |
| 244 | fm_index | BWT・FM-index（省メモリ全文検索） | |
| 245 | de_bruijn | de Bruijn 列の構成 | |
| 246 | palindromic_series | 回文分解 DP（eertree の series links で O(n log n)） | ⭐ |
| 247 | k_mismatch | k 不一致文字列照合（FFT + kangaroo jump） | |
| 248 | baker_bird | 2D パターン照合（Baker–Bird） | |
| 249 | lcs_bitparallel | LCS のビット並列化 O(nm/64) | ⭐ |
| 250 | aho_corasick_online | 追加クエリ対応 Aho–Corasick（二進マージ） | 要 #115 |
| 251 | line_arrangement | 直線アレンジメント・点-直線双対 | |
| 252 | polygon_boolean | 多角形のブール演算（交差・合併・差） | |
| 253 | polygon_triangulation | 多角形の三角形分割（耳刈り/単調分割） | |
| 254 | circle_union | 円の和集合面積 | ⭐ |
| 255 | convex_layers | 凸包の層分解（onion peeling） | |
| 256 | point_location | 平面点位置決定（永続構造スイープ O(log n)） | |
| 257 | geometric_median | 幾何中央値（Weiszfeld 反復） | |
| 258 | convex_intersection | 凸多角形同士の交差 O(n+m) | |
| 259 | surreal | 超現実数・partizan ゲーム理論 | |
| 260 | green_hackenbush | Green Hackenbush（竹・木・グラフの Grundy） | |
| 261 | misere_nim | Misère Nim・genus 理論 | |
| 262 | octal_games | 八進ゲームと Grundy 数の周期性 | |
| 263 | aliens_2d | 多重 Alien trick（2 パラメータ同時ペナルティ探索） | |
| 264 | dinkelbach | Dinkelbach 法（分数計画の一般枠組み） | #221 の一般化 |

## 10. 世界最上位レベル — 30

ICPC WF 最難・CF Div.1 E/F・AGC 終盤クラス。ほぼ全て「出たら書く」領域だが、
リスト化しておくことで問題駆動の実装判断を速くする。⭐ は最上位帯の中では出題実績が多いもの。

| # | 候補 | 概要 | 備考 |
|---|---|---|---|
| 265 | frederickson_topology | Topology tree（worst-case 動的 MST・動的 2 辺連結） | |
| 266 | top_tree_nonlocal | Top tree の非局所探索（動的な重心・直径・最遠点） | 要 #155 |
| 267 | graph_voronoi | グラフ上のボロノイ図（多始点 Dijkstra 分割 + 境界辺集約） | ⭐ |
| 268 | lct_dinic | Link-cut tree による blocking flow O(mn log n) | |
| 269 | mcf_cost_scaling | コストスケーリング最小費用流 O(n²m log(nC)) | |
| 270 | planar_max_flow | 平面グラフ最大流（双対グラフ最短路で O(n log n)） | |
| 271 | planar_separator | 平面分離定理による分割統治 | |
| 272 | series_parallel | 直列並列グラフの認識と DP | |
| 273 | fkt | FKT アルゴリズム（平面グラフの完全マッチング数、Pfaffian） | |
| 274 | tutte_polynomial | Tutte 多項式（n≤20 程度、信頼性多項式・彩色数を統一） | |
| 275 | chromatic_polynomial | 彩色多項式（subset convolution で O(2^n n²)） | 要 #102 |
| 276 | subgraph_counting | 小パターン部分グラフ数え上げ（行列積・三角形 O(n^ω)） | |
| 277 | weighted_matroid_intersection | 重み付きマトロイド交差 | 要 #114 |
| 278 | matroid_union | マトロイド和（k 本の素な全域木など） | 要 #114 |
| 279 | submodular_repr | 高次擬ブール関数の最小カット表現（一般化燃やす埋める） | 要 #43 |
| 280 | semiring_kleene | 半環上の Kleene 閉包（経路の正規表現的集約・確率遷移） | 要 #92 |
| 281 | kth_term_algebraic | 代数的母関数の第 n 項（Newton 反復 + P-recursive 化） | 要 #173 |
| 282 | q_series | q 級数（Euler の恒等式・Jacobi triple product による高速展開） | |
| 283 | gauss_lattice_2d | 2 次元格子基底簡約（最近格子点・最短ベクトル） | ⭐ 幾何数論の難問部品 |
| 284 | integer_half_gcd | 多倍長整数の subquadratic GCD（half-GCD 整数版） | 要 #169,#238 |
| 285 | hook_length | フック長公式・標準 Young 盤の数え上げ | ⭐ |
| 286 | rsk | RSK 対応（増加部分列の分布・Schur 関数和公式） | #285 と対 |
| 287 | continuous_dp | 連続値の積分 DP（確率密度を多項式区分で持つ） | ⭐ AGC 系確率問題 |
| 288 | square_counting | 相異なる平方部分文字列の数え上げ（runs 応用） | 要 #126 |
| 289 | klee_3d | Klee の測度問題 3D（直方体和集合の体積 O(n^{3/2} log n)） | |
| 290 | halfspace_intersection_3d | 3D 半空間交差（双対 3D 凸包帰着） | 要 #138 |
| 291 | farthest_voronoi | 最遠点ボロノイ図 | |
| 292 | cache_efficient_segtree | キャッシュ効率レイアウト（Eytzinger/暗黙 B-tree）の高速セグ木 | 定数倍最適化の切り札 |
| 293 | simd_optimization | SIMD・ビットスライスによる定数倍高速化テク集 | AVX2/NEON。提出環境依存に注意 |
| 294 | game_thermography | 温度理論（thermography、combinatorial game の最適手番評価） | 要 #259 |

## 参考リンク集（実装の出典・照合先）

他エージェントが実装を進める際は、**Library Checker に対応問題があればそれを正とし**、
実装の参考には下記ライブラリ群を当たること（Rust への移植時は AGENTS.md 規約に合わせる）。

### 総合カタログ
- Library Checker（判定付き問題集。カテゴリ網羅・テストの正解基準）: https://judge.yosupo.jp/
- cp-algorithms（英語。アルゴリズム解説の標準）: https://cp-algorithms.com/
- The Ultimate Topic List（YouKn0wWho。~500 トピックの難易度付き索引）: https://blog.shahjalalshohag.com/topic-list/
- Codeforces Catalog（解説記事の集約）: https://codeforces.com/catalog
- OI Wiki（中国語。網羅性最強クラス）: https://oi-wiki.org/

### 実装ライブラリ（読める実装の宝庫）
- AtCoder Library (ACL): https://github.com/atcoder/ac-library
- ac-library-rs（ACL の Rust 移植。本 repo の流儀に近い）: https://github.com/rust-lang-ja/ac-library-rs
- Nyaan's Library（FPS・数論・行列が特に厚い）: https://nyaannyaan.github.io/library/
- ei1333's Library / Luzhiled（データ構造・グラフが厚い）: https://ei1333.github.io/library/
- maspy's Library（数え上げ・FPS 最先端。§9-10 の多くを収録）: https://github.com/maspypy/library
- KACTL（ICPC 向け高密度実装。blossom・半平面交差など）: https://github.com/kth-competitive-programming/kactl
- suisen's Library: https://suisen-cp.github.io/cp-library-cpp/

### 解説記事（ピンポイント）
- maspy のブログ（FPS 入門連載・Euler 変換・合成など §4/§9 系）: https://maspypy.com/
- noshi91 のブログ（Slope trick・OfflineDynamicConnectivity・データ構造理論）: https://noshi91.hatenablog.com/
- snuke のブログ（Manacher・Z・SA など文字列基礎の定番解説）: https://snuke.hatenablog.com/
- drken の Qiita（典型総整理。§1-2 の背景理解）: https://qiita.com/drken1215
- 競プロ典型 90 問（高度典型の出題例と解説）: https://github.com/E869120/kyopro_educational_90
- FPS 合成 O(n log n) は Kinoshita–Li "Power Series Composition in Near-Linear Time"
  (FOCS 2024, arXiv 検索) と Library Checker `composition_of_formal_power_series` を参照

## 実装順の推奨（S → 抜粋 A）

1. **S 帯**: sliding_window_min + swag / mo_algorithm / eulerian_path / cycle_detection /
   quotient_ranges / matrix_mod / convex_hull_int
2. **A 帯前半**: dynamic_cht / fenwick_2d / block_cut_tree / two_edge_cc / hopcroft_karp /
   hungarian / tree_hash / namori / sqrt_mod / discrete_log / primitive_root /
   divisor_transform / aho_corasick / lcp_range / rotating_calipers / point_in_polygon /
   divide_conquer_opt / aliens_trick / dynamic_modint / neighbors / min_factor_table /
   persistent_segtree / offline_dynamic_connectivity / dsu_bipartite / mcf_negative /
   matrix_semiring
3. B/C 帯は必要になった問題駆動で。

実装時は AGENTS.md 規約（1 概念 1 ファイル・doctest・ナイーブ比較テスト・expander 安全）に従う。
