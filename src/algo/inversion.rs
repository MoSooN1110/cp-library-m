//! 転倒数（BIT で O(n log n)）。`ds::fenwick` を再利用。
//!
//! ```
//! use cplib::algo::inversion::*;
//! assert_eq!(inversion_number(&[3, 1, 2]), 2);   // (3,1),(3,2)
//! assert_eq!(inversion_number(&[1, 2, 3]), 0);
//! ```
use crate::ds::fenwick::Fenwick;

/// 転倒数（i<j かつ a[i]>a[j] の個数）。
pub fn inversion_number<T: Ord + Clone>(a: &[T]) -> u64 {
    // 座標圧縮
    let mut sorted: Vec<T> = a.to_vec();
    sorted.sort();
    sorted.dedup();
    let k = sorted.len();
    let mut bit = Fenwick::<i64>::new(k);
    let mut inv = 0u64;
    for (seen, x) in a.iter().enumerate() {
        let r = sorted.partition_point(|v| v < x); // rank
        // すでに置いた要素のうち x より大きいものの数
        inv += seen as u64 - bit.sum(0..r + 1) as u64;
        bit.add(r, 1);
    }
    inv
}

#[cfg(test)]
mod tests {
    use super::*;
    fn brute(a: &[i64]) -> u64 {
        let mut c = 0u64;
        for i in 0..a.len() {
            for j in i + 1..a.len() {
                if a[i] > a[j] {
                    c += 1;
                }
            }
        }
        c
    }
    #[test]
    fn matches_brute() {
        let mut x: u64 = 999;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..300 {
            let n = (rng() as usize) % 20;
            let a: Vec<i64> = (0..n).map(|_| (rng() % 10) as i64).collect();
            assert_eq!(inversion_number(&a), brute(&a));
        }
    }
}
