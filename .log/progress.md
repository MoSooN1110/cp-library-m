# Progress Log

## 2026-07-11

Objective: make this Rust competitive programming library more complete and easy to use while keeping it as a compiling Cargo crate and preserving expander compatibility.

### Migrated / Added Modules

- `ds::dynamic_segtree`
  - Sparse point-update/range-product segment tree for huge index ranges.
  - Added doctest and unit tests, including random comparison against a vector.
- `ds::segtree_2d`
  - 2D segment tree for point updates and rectangular range products.
  - Added tests for sum/max and random brute-force comparison.
- `ds::slope_trick`
  - Convex piecewise-linear function operations.
  - Added tests for one-sided functions, absolute value additions, shifts, and small-domain brute force.
- `graph::hld`
  - Heavy-Light Decomposition with LCA, distance, subtree intervals, and path ranges.
  - Added random tree tests against brute-force LCA/path/subtree checks.
- `graph::rerooting`
  - Generic rerooting DP.
  - Added distance-sum example and brute-force BFS comparison.
- `geometry::basic`
  - Integer-coordinate geometry: `Point`, dot/cross, `ccw`, segment intersection, polygon area.
  - Replaced `_pending/line_segment_crossing.rs`.
  - Fixed collinear disjoint segment handling and doctest expectations.
- `math::gaussian_elimination`
  - Real-valued RREF/rank/linear-system solver.
  - Fixed inconsistent-system detection by avoiding RHS-column rank confusion.
- `math::vector_lcm`
  - Slice gcd/lcm and checked lcm helpers.
- `graph::dijkstra`
  - Integrated `dijkstra_top2` into existing Dijkstra module instead of adding a separate module.
  - Added doctest and brute-force DAG tests for second-shortest distinct distances.
- `graph::components`
  - Undirected connected components: IDs, component lists, count, connectivity, extra edges needed.
  - Replaced `_pending/graph_connect_num.rs`.
- `graph::grid_bfs`
  - 4-neighbor grid BFS, multi-source BFS, and 0-1 BFS.
  - Replaced `_pending/grid_bfs.rs`.
- `algo::bit_enumeration`
  - Subset masks, selected index iteration, submask iteration.
  - Replaced `_pending/bitbruteforce.rs`.
- `dp::digit_dp`
  - Digit vectors and digit-sum distribution/count/sum for `0..=n`.
  - Replaced `_pending/digit_dp.rs`.

### Documentation / Index Updates

- Updated `README.md` library table for newly migrated modules.
- Updated `INDEX.md` to match the actual remaining `_pending/` files.
- Updated `examples/sample_main.rs` so expander smoke covers newer modules:
  - `geometry::basic`
  - `graph::dijkstra::dijkstra_top2`
  - `graph::components`
  - `graph::grid_bfs`
  - `dp::digit_dp`

### Verification Performed

Latest successful checks:

```bash
cargo test
python3 tools/expand.py examples/sample_main.rs -o /tmp/cplib_sample.rs
rustc /tmp/cplib_sample.rs -o /tmp/cplib_sample
/tmp/cplib_sample
comm -3 <(find _pending -maxdepth 1 -type f | sort) <(grep -o '_pending/[^`]*\.rs' INDEX.md | sort)
git diff --check
```

Observed successful test counts at the latest full run:

- `130` unit tests passed.
- `74` doctests passed.
- Expander output compiled and ran standalone.

Sample output included:

```text
fib10=55
max=5
lazysum=16
same=true
dist2=5
dist2_second=10
components=2
grid_dist=2
digit_sum_count=3
crt=Some((8, 15))
isprime=true
rh=true
intersect=true
```

### Current Worktree Notes

- The worktree is intentionally dirty with many migrated modules not yet committed.
- Deleted `_pending/` files correspond to migrated modules.
- Do not revert user/previous-agent changes. Continue building on the current state.
- `INDEX.md` has been kept mechanically consistent with current `_pending/`.

### Remaining Pending Areas

Current `_pending/` still contains work in these areas:

- Data structures: implicit treap, segment-tree variants, wavelet matrix variants, segtree beats.
- Graph: auxiliary tree, centroid decomposition, game graph analysis, DFS order, tree decomposition, remaining HLD/rerooting variants.
- Math: FFT/FHT/FPS/polynomial/prime count.
- Geometry: angular sort, remaining floating geometry snippets, rectangle union.
- DP/misc/string: comb table DP, segment DP, substring helper, recursive macros, stack DFS, product helper.


## 2026-07-12 (Fable)

- Audit: `.log/Fable_Report.md`（使いやすさ/強さ/美しさの観点で監査）。
- New modules (all with doctests + naive/random tests):
  `misc::io` (Scanner), `graph::two_sat`, `ds::li_chao_tree`, `ds::rollback_dsu`,
  `string::manacher`, `string::kmp`, `math::lagrange_interpolation`.
- Housekeeping: README 収録表を全 104 モジュールの完全索引に再生成、
  INDEX.md を移行完了ノートに置換、CHT doc に li_chao_tree への相互参照、
  sample_main に io/two_sat/manacher を追加（two_sat 経由で scc の依存推移を smoke 検証）。
- Roadmap: `.log/Fable_Candidates.md` に拡張候補 294 個
  （高度典型 154 / 赤レベル 110 / 世界最上位 30、優先度 S/A/B/C・依存・参考リンク付き）。
- Verification: cargo test --lib 230 passed / doctest 104 passed / expander smoke OK.
- Worktree remains uncommitted per user-decides-commits policy.

## 2026-07-12 (Fable candidates, S priority)

- Added `.log/Implementation_Checklist.md` tracking S/A items from `.log/Fable_Candidates.md`.
- Implemented all S-priority candidate modules:
  `ds::sliding_window_min`, `ds::swag`, `algo::mo_algorithm`,
  `graph::cycle_detection`, `graph::eulerian_path`,
  `math::quotient_ranges`, `math::matrix_mod`, `geometry::convex_hull_int`.
- Each module has a module doc example and unit tests with known cases and/or brute/random checks.
