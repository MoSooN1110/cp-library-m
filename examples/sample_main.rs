#![allow(non_snake_case)]
use cplib::math::matrix::*;
use cplib::math::modint::*;
use cplib::ds::segtree::*;
use cplib::ds::dsu::*;

fn main() {
    // matrix (depends on modint) : fib(10)
    let a = Matrix::from(vec![
        vec![Mint::from(1i64), Mint::from(1i64)],
        vec![Mint::from(1i64), Mint::from(0i64)],
    ]);
    println!("fib10={}", a.pow(10).a[0][1]);

    // segtree range max
    let seg = SegTree::from_slice(&[1i64, 3, 2, 5, 4], i64::MIN, |x, y| x.max(y));
    println!("max={}", seg.prod(1..4));

    // dsu
    let mut d = Dsu::new(4);
    d.merge(0, 1);
    println!("same={}", d.same(0, 1));
}
