#![allow(non_snake_case)]
//@expand
use cplib::algo::product::*;
use cplib::dp::binomial_distribution::*;
use cplib::dp::digit_dp::*;
use cplib::dp::iwi::*;
use cplib::ds::dsu::*;
use cplib::ds::lazy_segtree::*;
use cplib::ds::segtree::*;
use cplib::geometry::angle_sort::*;
use cplib::geometry::basic::*;
use cplib::geometry::float::{LineF, PointF};
use cplib::geometry::rectangle_union::*;
use cplib::graph::components::*;
use cplib::graph::dfs::*;
use cplib::graph::dijkstra::*;
use cplib::graph::grid_bfs::grid_bfs;
use cplib::graph::grid_components::*;
use cplib::math::fps::*;
use cplib::math::linear_recurrence::*;
use cplib::math::matrix::*;
use cplib::math::modint::*;
use cplib::math::number::*;
use cplib::math::prime::*;
use cplib::misc::recursive::*;
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
    let seg = SegTree::from_slice(&[1i64, 3, 2, 5, 4], i64::MIN, |x, y| x.max(y));
    println!("max={}", seg.prod(1..4));

    // lazy segtree: range add / range sum
    let mut lz = LazySegTree::from_slice(
        &[(1i64, 1i64), (2, 1), (3, 1)],
        (0, 0),
        |p: (i64, i64), q: (i64, i64)| (p.0 + q.0, p.1 + q.1),
        0i64,
        |f: i64, x: (i64, i64)| (x.0 + f * x.1, x.1),
        |f: i64, g: i64| f + g,
    );
    lz.apply_range(0..2, 5);
    println!("lazysum={}", lz.all_prod().0);

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
}
