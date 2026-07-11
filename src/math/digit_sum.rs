//! 桁和（指定した基数での各桁の和）。
//!
//! ```
//! use cplib::math::digit_sum::*;
//! assert_eq!(digit_sum(1234, 10), 10);
//! assert_eq!(digit_sum(0b1011, 2), 3);   // popcount 相当
//! ```

pub fn digit_sum(mut n: u64, base: u64) -> u64 {
    assert!(base >= 2);
    let mut s = 0;
    while n > 0 {
        s += n % base;
        n /= base;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(digit_sum(0, 10), 0);
        assert_eq!(digit_sum(9999, 10), 36);
        assert_eq!(digit_sum(255, 16), 30); // 0xFF -> 15+15
        for n in 0..1000u64 {
            let expect: u64 = n.to_string().bytes().map(|c| (c - b'0') as u64).sum();
            assert_eq!(digit_sum(n, 10), expect);
        }
    }
}
