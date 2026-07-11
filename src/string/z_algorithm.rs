//! Z-algorithm。`z[i]` = s と s[i..] の最長共通接頭辞長（z[0]=|s|）。
//!
//! ```
//! use cplib::string::z_algorithm::*;
//! let z = z_algorithm(b"aaabaab");
//! assert_eq!(z, vec![7, 2, 1, 0, 2, 1, 0]);
//! ```

pub fn z_algorithm<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0usize; n];
    if n == 0 {
        return z;
    }
    z[0] = n;
    let (mut l, mut r) = (0usize, 0usize);
    for i in 1..n {
        if i < r {
            z[i] = z[i - l].min(r - i);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

#[cfg(test)]
mod tests {
    use super::*;
    fn brute(s: &[u8]) -> Vec<usize> {
        let n = s.len();
        (0..n)
            .map(|i| {
                let mut k = 0;
                while i + k < n && s[k] == s[i + k] {
                    k += 1;
                }
                k
            })
            .collect()
    }
    #[test]
    fn matches_brute() {
        let cases: &[&[u8]] = &[b"", b"a", b"aaaaa", b"aaabaab", b"abababab", b"mississippi"];
        for &c in cases {
            assert_eq!(z_algorithm(c), brute(c));
        }
        // ランダム（小アルファベット）
        let mut x: u64 = 12345;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..200 {
            let n = (rng() as usize) % 30;
            let s: Vec<u8> = (0..n).map(|_| b'a' + (rng() % 3) as u8).collect();
            assert_eq!(z_algorithm(&s), brute(&s));
        }
    }
}
