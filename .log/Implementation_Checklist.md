# Implementation Checklist

Source: `.log/Fable_Candidates.md` (2026-07-12). まず優先度 S/A を実装対象として管理する。

## S Priority

- [x] `ds::sliding_window_min` - monotone deque による固定長窓 min/max
- [x] `ds::swag` - foldable queue
- [x] `algo::mo_algorithm` - Mo 法のクエリ順序生成
- [x] `graph::eulerian_path` - オイラー路/閉路
- [x] `graph::cycle_detection` - 有向/無向閉路検出
- [x] `math::quotient_ranges` - floor(n/i) 一定区間列挙
- [x] `math::matrix_mod` - mod p 行列の rank/det/inv/solve
- [x] `geometry::convex_hull_int` - 整数座標凸包

## A Priority

- [ ] `ds::disjoint_sparse_table`
- [ ] `ds::fenwick_2d`
- [ ] `ds::persistent_segtree`
- [ ] `ds::offline_dynamic_connectivity`
- [ ] `ds::dynamic_cht`
- [ ] `ds::dsu_bipartite`
- [ ] `graph::block_cut_tree`
- [ ] `graph::two_edge_cc`
- [ ] `graph::hopcroft_karp`
- [ ] `graph::hungarian`
- [ ] `graph::mcf_negative`
- [ ] `graph::tree_hash`
- [ ] `graph::namori`
- [ ] `math::sqrt_mod`
- [ ] `math::discrete_log`
- [ ] `math::primitive_root`
- [ ] `math::min_factor_table`
- [ ] `math::matrix_semiring`
- [ ] `math::divisor_transform`
- [ ] `string::aho_corasick`
- [ ] `string::lcp_range`
- [ ] `geometry::rotating_calipers`
- [ ] `geometry::point_in_polygon`
- [ ] `algo::divide_conquer_opt`
- [ ] `algo::aliens_trick`
- [ ] `misc::dynamic_modint`
- [ ] `misc::neighbors`
