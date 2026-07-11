//! ヒストグラム中の最大長方形面積（スタック, O(n)）。
//!
//! ```
//! use cplib::algo::max_rectangle::*;
//! assert_eq!(max_rectangle_in_histogram(&[2, 1, 5, 6, 2, 3]), 10);
//! ```

pub fn max_rectangle_in_histogram(heights: &[u64]) -> u64 {
    let n = heights.len();
    let mut best = 0u64;
    let mut stack: Vec<(usize, u64)> = Vec::new(); // (開始位置, 高さ)
    for i in 0..=n {
        let h = if i < n { heights[i] } else { 0 };
        let mut start = i;
        while let Some(&(s, sh)) = stack.last() {
            if sh > h {
                best = best.max(sh * (i - s) as u64);
                start = s;
                stack.pop();
            } else {
                break;
            }
        }
        if i < n {
            stack.push((start, h));
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    fn brute(h: &[u64]) -> u64 {
        let n = h.len();
        let mut best = 0;
        for i in 0..n {
            let mut mn = u64::MAX;
            for j in i..n {
                mn = mn.min(h[j]);
                best = best.max(mn * (j - i + 1) as u64);
            }
        }
        best
    }
    #[test]
    fn matches_brute() {
        assert_eq!(max_rectangle_in_histogram(&[]), 0);
        assert_eq!(max_rectangle_in_histogram(&[5]), 5);
        let mut x: u64 = 7;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..300 {
            let n = (rng() as usize) % 15;
            let h: Vec<u64> = (0..n).map(|_| rng() % 10).collect();
            assert_eq!(max_rectangle_in_histogram(&h), brute(&h));
        }
    }
}
