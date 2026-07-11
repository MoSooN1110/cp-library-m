#![allow(non_snake_case)]
use cplib::math::matrix::*;
use cplib::math::modint::*;
use cplib::math::number::*;
use cplib::math::prime::*;
use cplib::ds::segtree::*;
use cplib::ds::lazy_segtree::*;
use cplib::ds::dsu::*;
use cplib::graph::dijkstra::*;
use cplib::string::rolling_hash::*;

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
    g[1].push((2usize, 3u64));
    println!("dist2={}", dijkstra(&g, 0)[2]);

    // number theory
    println!("crt={:?}", crt(&[2, 3], &[3, 5]));

    // prime
    println!("isprime={}", is_prime(1_000_000_007));

    // rolling hash
    let rh = RollingHash::new(b"abcabc");
    println!("rh={}", rh.eq(0..3, 3..6));
}
