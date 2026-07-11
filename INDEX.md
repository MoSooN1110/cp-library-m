# library index

競技プログラミング用ライブラリ集。各ファイルはコピペで使う想定です。
大半は Zed スニペット `rust.json`（`lib_*` / `mlib_*`）から抽出したもので、
`MOD` / `read_vec` などテンプレ側の基本定義に依存するものがあります。

- **`modint.rs`**: 手作りの正準版。MOD 内蔵・組合せ(`Comb`: nCr/nPr/nHr)内包で**単体動作**（検証済み）。
- `_archive_oldstyle/`: 旧 CamelCase 実装（`SEG`/`UFT`/`LCA` 等。`lib_*` 版の旧バージョンの可能性）。
- `_archive_modint/`: `modint.rs` に統合済みの旧 modint/組合せ実装。

## 重複が残っている主なもの（今後の統合候補）
- wavelet: `wavelet_matrix.rs`〜`wavelet_matrix4.rs`
- hld: `hld.rs` / `hld_2.rs`  ／ trie: `trie.rs` / `trie2.rs` / `binarytrie.rs`
- dijkstra / segtree / matrix なども複数版あり

## ds  (27)
- `2d_segtree.rs`
- `AccSum2D.rs`
- `Commulative2D.rs`
- `bitbruteforce.rs`
- `bitset.rs`
- `btree_multiset.rs`
- `disjoint_set_union.rs`
- `dp_bitdp_subset.rs`
- `dynamic_segtree.rs`
- `ft64.rs`
- `implicit_treap.rs`
- `lazy_segtree_range_affine_range_add.rs`
- `median_set.rs`
- `multiset.rs`
- `multiset_i64.rs`
- `rheap.rs`
- `seg_lazy.rs`
- `segment_dp.rs`
- `segment_set.rs`
- `segment_sum.rs`
- `segment_sum_2d.rs`
- `segtree.rs`
- `segtree_beats.rs`
- `slope_trick.rs`
- `union_find.rs`
- `vec_to_map.rs`
- `weighted_dsu.rs`

## graph  (34)
- `auxiliary_tree.rs`
- `backward_analysis_of_game_graph.rs`
- `bellman_ford.rs`
- `bipartiate_graph_judgement.rs`
- `bipartite_matching.rs`
- `centoroid_decomp.rs`
- `dfs_order_gen.rs`
- `dijkstra.rs`
- `dijkstra_top2.rs`
- `doubling.rs`
- `euler_tour.rs`
- `graph_bellman_ford.rs`
- `graph_connect_num.rs`
- `graph_dijkstra.rs`
- `grid_bfs.rs`
- `grid_congruence.rs`
- `groop_looping.rs`
- `group_looping.rs`
- `hld.rs`
- `hld_2.rs`
- `lca.rs`
- `lowlink.rs`
- `max_flow.rs`
- `min_cost_flow.rs`
- `min_cost_flow2.rs`
- `mst.rs`
- `rerooting.rs`
- `rerooting_2.rs`
- `scc.rs`
- `topo_sort.rs`
- `tree_decompose.rs`
- `tree_diameter.rs`
- `treediameter.rs`
- `warshall_floyd.rs`

## math  (34)
- `com_lucas.rs`
- `combtable_dp.rs`
- `crt_garner.rs`
- `digit_sum.rs`
- `divisor.rs`
- `euler_phi_function.rs`
- `ext_gcd.rs`
- `ext_gcd_lde.rs`
- `faster_prime_factorization.rs`
- `fft.rs`
- `fht.rs`
- `fps.from_snippet.rs`
- `fps.rs`
- `gaussian_elimination.rs`
- `gcd.rs`
- `martix_pow.rs`
- `matrix.rs`
- `matrix_multiplication.rs`
- `matrix_pow_2.rs`
- `min_linear_mod.rs`
- `modint.rs`
- `nnt.rs`
- `osak_prime_factorization.rs`
- `polynomial.rs`
- `prime_count.rs`
- `prime_factoriaztion.rs`
- `prime_table.rs`
- `ratio.rs`
- `store_modint.rs`
- `vector_lcm.rs`
- `wavelet_matrix.rs`
- `wavelet_matrix2.rs`
- `wavelet_matrix3.rs`
- `wavelet_matrix4.rs`

## string  (11)
- `acl_rust_string.rs`
- `binarytrie.rs`
- `inversion_number.rs`
- `lcs.rs`
- `rolling_hash.rs`
- `substr.rs`
- `suffix_array.rs`
- `trie.rs`
- `trie2.rs`
- `z_algo.rs`
- `zalgo_2.rs`

## geometry  (8)
- `anguler_sort.rs`
- `convex_hull_trick.rs`
- `geom.rs`
- `geometory.rs`
- `histgram_max_area.rs`
- `line_segment_crossing.rs`
- `max_histogram.rs`
- `tech_rectangle_union.rs`

## dp  (1)
- `digit_dp.rs`

## misc  (19)
- `XorShift.rs`
- `binary_search.rs`
- `binsearch.rs`
- `bmbm.rs`
- `chmax.rs`
- `f64total.rs`
- `golden_sep_search.rs`
- `judge_grid_connect.rs`
- `lis.rs`
- `macro_recursive.rs`
- `next_permutation.rs`
- `product.rs`
- `rec_macro.rs`
- `run_length_encoding.rs`
- `stack_dfs.rs`
- `ternaly_search.rs`
- `total_ord.rs`
- `vector_accumulation.rs`
- `vector_compression.rs`

