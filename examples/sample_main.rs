#![allow(non_snake_case)]
//@expand
use cplib::algo::product::*;
use cplib::algo::mo_algorithm::*;
use cplib::dp::binomial_distribution::*;
use cplib::dp::digit_dp::*;
use cplib::dp::iwi::*;
use cplib::ds::dsu::*;
use cplib::ds::lazy_segtree::*;
use cplib::ds::segtree::*;
use cplib::ds::sliding_window_min::*;
use cplib::ds::swag::*;
use cplib::geometry::angle_sort::*;
use cplib::geometry::basic::*;
use cplib::geometry::convex_hull_int::*;
use cplib::geometry::float::{LineF, PointF};
use cplib::geometry::rectangle_union::*;
use cplib::graph::components::*;
use cplib::graph::cycle_detection::*;
use cplib::graph::dfs::*;
use cplib::graph::dijkstra::*;
use cplib::graph::eulerian_path::*;
use cplib::graph::grid_bfs::grid_bfs;
use cplib::graph::grid_components::*;
use cplib::graph::two_sat::*;
use cplib::math::fps::*;
use cplib::math::linear_recurrence::*;
use cplib::math::matrix::*;
use cplib::math::matrix_mod::{determinant, solve_linear};
use cplib::math::modint::*;
use cplib::math::number::*;
use cplib::math::prime::*;
use cplib::math::quotient_ranges::*;
use cplib::misc::io::*;
use cplib::misc::recursive::*;
use cplib::string::manacher::*;
use cplib::string::rolling_hash::*;
use cplib::string::substring::*;

fn main() {
    // matrix (depends on modint): fib(10)
    let a = Matrix::from(vec![
        vec![Mint::from(1i64), Mint::from(1i64)],
        vec![Mint::from(1i64), Mint::from(0i64)],
    ]);
    println!("fib10={}", a.pow(10).a[0][1]);

    // segtree range max
    let seg = MaxSegTree::from_slice_max(&[1, 3, 2, 5, 4]);
    println!("max={}", seg.prod(1..4));

    // lazy segtree: range add / range sum
    let mut lz = RangeAddSum::from_slice_range_add_sum(&[1, 2, 3]);
    lz.add(0..2, 5);
    println!("lazysum={}", lz.sum(0..3));

    // dsu
    let mut d = Dsu::new(4);
    d.merge(0, 1);
    println!("same={}", d.same(0, 1));

    // dijkstra
    let mut g = vec![vec![]; 3];
    g[0].push((1usize, 2u64));
    g[0].push((2usize, 10u64));
    g[1].push((2usize, 3u64));
    println!("dist2={}", dijkstra(&g, 0)[2]);
    println!("dist2_second={}", dijkstra_top2(&g, 0)[2][1]);

    // connected components
    let ug = vec![vec![1], vec![0], vec![]];
    println!("components={}", connected_component_count(&ug));
    println!("dfs_order_len={}", dfs_order(&ug, 0).len());

    // grid bfs
    let passable = vec![vec![true, true], vec![false, true]];
    println!("grid_dist={}", grid_bfs(&passable, (0, 0))[1][1]);
    println!("grid_connected={}", is_grid_connected(&passable));

    // digit dp
    println!("digit_sum_count={}", count_by_digit_sum_leq(20, 10, 2));
    println!("binom={:.3}", fair_binomial_prob(4, 2));
    println!("iwi={}", max_iwi_removable_len("iwiiwi"));
    println!("product_count={}", product_k_n(2, 3).count());
    println!("mo_first={}", mo_order(10, &[(0, 3), (2, 5)])[0]);

    // number theory
    println!("crt={:?}", crt(&[2, 3], &[3, 5]));
    println!(
        "linrec={}",
        linear_recurrence_kth(
            &[Mint::new(0), Mint::new(1)],
            &[Mint::new(1), Mint::new(1)],
            10
        )
    );
    println!(
        "fps_inv_last={}",
        inv_series(&[Mint::new(1), Mint::new(-1)], 5)[4]
    );
    println!("quotients={}", quotient_values(10).len());
    println!(
        "det={}",
        determinant(vec![vec![Mint::new(1), Mint::new(2)], vec![Mint::new(3), Mint::new(4)]])
    );
    println!(
        "solve_x0={}",
        solve_linear(
            vec![vec![Mint::new(1), Mint::new(2)], vec![Mint::new(3), Mint::new(4)]],
            vec![Mint::new(5), Mint::new(11)]
        )
        .unwrap()[0]
    );

    // prime
    println!("isprime={}", is_prime(1_000_000_007));
    let mut fib_rec =
        MemoizedRecursiveFunction::new(|f, n: usize| if n < 2 { n } else { f(n - 1) + f(n - 2) });
    println!("memo_fib10={}", fib_rec.call(10));

    // rolling hash
    let rh = RollingHash::new(b"abcabc");
    println!("rh={}", rh.eq(0..3, 3..6));
    println!("substring={}", substring("aβcd", 1, 3));

    // geometry
    let mut angle_points = [Point::new(0, 1), Point::new(1, 0), Point::new(0, -1)];
    sort_points_by_angle(&mut angle_points);
    println!("angle_first=({}, {})", angle_points[0].x, angle_points[0].y);
    println!(
        "hull_len={}",
        convex_hull(
            &[Point::new(0, 0), Point::new(1, 0), Point::new(1, 1), Point::new(0, 1)],
            false
        )
        .len()
    );
    println!(
        "float_dist={:.1}",
        LineF::from_two_points(PointF::new(-1.0, 1.0), PointF::new(1.0, 1.0))
            .distance(PointF::new(0.0, 0.0))
    );

    println!(
        "intersect={}",
        segments_intersect(
            Point::new(0, 0),
            Point::new(4, 0),
            Point::new(2, -1),
            Point::new(2, 1),
        )
    );

    let rects = [Rectangle::new(0, 0, 3, 2), Rectangle::new(1, 1, 4, 3)];
    println!("rect_union={}", rectangle_union_area(&rects));

    // io scanner (from_str なので入力待ちしない)
    let mut sc = Scanner::from_str("7 40");
    let (a, b): (i64, i64) = (sc.read(), sc.read());
    println!("scan_sum={}", a + b);

    // 2-SAT (scc への依存推移を expander smoke で踏む)
    let mut ts = TwoSat::new(2);
    ts.add_clause(0, true, 1, true);
    ts.add_unit(0, false);
    println!("two_sat_x1={}", ts.solve().unwrap()[1]);
    println!("cycle_len={}", find_cycle_directed(&vec![vec![1], vec![2], vec![0]]).unwrap().len());
    println!(
        "euler_len={}",
        eulerian_circuit_directed(3, &[(0, 1), (1, 2), (2, 0)]).unwrap().len()
    );

    // manacher
    println!("longest_pal={:?}", longest_palindrome(b"mosoon"));

    // sliding window / SWAG
    println!("slide_min={:?}", sliding_window_min(&[3, 1, 4, 1, 5], 3));
    let mut swag = Swag::new(0i64, |a, b| a + b);
    swag.push(10);
    swag.push(20);
    println!("swag_sum={}", swag.fold());
}
