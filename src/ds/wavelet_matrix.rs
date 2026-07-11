//! Wavelet Matrix（静的な数列に対する順位・頻度・k 番目クエリ）。
//!
//! 値域 `u64`。構築 O(n log σ)、各クエリ O(log σ)（σ = 値の種類数の上限＝2^height）。
//!
//! ```
//! use cplib::ds::wavelet_matrix::*;
//! let wm = WaveletMatrix::new(&[5, 4, 5, 5, 2, 1, 5, 6, 1, 3, 7, 0]);
//! assert_eq!(wm.access(0), 5);
//! assert_eq!(wm.freq(0, 12, 5), 4);
//! assert_eq!(wm.kth_smallest(0, 12, 0), Some(0));
//! assert_eq!(wm.kth_largest(0, 12, 0), Some(7));
//! assert_eq!(wm.less_than(0, 12, 5), 6);
//! assert_eq!(wm.range_freq(0, 12, 3, 6), 6);
//! assert_eq!(wm.quantile(0, 12, 2), wm.kth_smallest(0, 12, 2));
//! ```

/// 内部用 succinct bit vector。`access` / `rank1` を O(1) で提供。
struct BitVector {
    bits: Vec<u64>,
    // 64bit ワードごとの 1 の累積個数（prefix[i] = bits[0..i) の 1 の総数）
    prefix: Vec<u32>,
}

impl BitVector {
    fn new(raw: &[bool]) -> Self {
        let words = raw.len().div_ceil(64);
        let mut bits = vec![0u64; words];
        for (i, &b) in raw.iter().enumerate() {
            if b {
                bits[i >> 6] |= 1 << (i & 63);
            }
        }
        let mut prefix = Vec::with_capacity(words + 1);
        prefix.push(0);
        let mut sum = 0u32;
        for &w in &bits {
            sum += w.count_ones();
            prefix.push(sum);
        }
        Self { bits, prefix }
    }

    #[inline]
    fn access(&self, i: usize) -> bool {
        (self.bits[i >> 6] >> (i & 63)) & 1 != 0
    }

    /// [0, i) に含まれる 1 の個数。
    #[inline]
    fn rank1(&self, i: usize) -> usize {
        let word = i >> 6;
        let off = i & 63;
        let mut cnt = self.prefix[word] as usize;
        if off != 0 {
            cnt += (self.bits[word] & ((1u64 << off) - 1)).count_ones() as usize;
        }
        cnt
    }
}

/// Wavelet Matrix 本体。
pub struct WaveletMatrix {
    levels: Vec<BitVector>, // MSB -> LSB
    zeros: Vec<usize>,      // 各レベルで 0 側に振り分けられた要素数
    height: u32,
    len: usize,
}

impl WaveletMatrix {
    /// 数列 `a` から構築する。O(n log σ)。
    pub fn new(a: &[u64]) -> Self {
        let len = a.len();
        let max = a.iter().copied().max().unwrap_or(0);
        let height = if max == 0 {
            1
        } else {
            64 - max.leading_zeros()
        };

        let mut levels = Vec::with_capacity(height as usize);
        let mut zeros = Vec::with_capacity(height as usize);

        let mut cur = a.to_vec();
        for bit in (0..height).rev() {
            let mut raw = vec![false; len];
            let mut zs = Vec::with_capacity(len);
            let mut os = Vec::with_capacity(len);
            for (i, &v) in cur.iter().enumerate() {
                if (v >> bit) & 1 == 1 {
                    raw[i] = true;
                    os.push(v);
                } else {
                    zs.push(v);
                }
            }
            zeros.push(zs.len());
            zs.extend(os);
            cur = zs;
            levels.push(BitVector::new(&raw));
        }

        Self {
            levels,
            zeros,
            height,
            len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    fn rank1(&self, lvl: usize, i: usize) -> usize {
        self.levels[lvl].rank1(i)
    }

    /// a[idx] を取得する。O(log σ)。
    pub fn access(&self, mut idx: usize) -> u64 {
        assert!(idx < self.len);
        let mut val = 0u64;
        for lvl in 0..self.height as usize {
            val <<= 1;
            if self.levels[lvl].access(idx) {
                val |= 1;
                idx = self.zeros[lvl] + self.rank1(lvl, idx);
            } else {
                idx -= self.rank1(lvl, idx);
            }
        }
        val
    }

    /// [l, r) における値 x の出現数。
    pub fn freq(&self, mut l: usize, mut r: usize, x: u64) -> usize {
        if l >= r || (self.height < 64 && x >= 1u64 << self.height) {
            return 0;
        }
        for lvl in 0..self.height as usize {
            let bit = (x >> (self.height as usize - lvl - 1)) & 1;
            let rl = self.rank1(lvl, l);
            let rr = self.rank1(lvl, r);
            if bit == 0 {
                l -= rl;
                r -= rr;
            } else {
                l = self.zeros[lvl] + rl;
                r = self.zeros[lvl] + rr;
            }
        }
        r - l
    }

    /// [l, r) で k 番目 (0-based) に小さい値。
    pub fn kth_smallest(&self, mut l: usize, mut r: usize, mut k: usize) -> Option<u64> {
        if l >= r || k >= r - l {
            return None;
        }
        let mut val = 0u64;
        for lvl in 0..self.height as usize {
            let zeros_l = l - self.rank1(lvl, l);
            let zeros_r = r - self.rank1(lvl, r);
            let zc = zeros_r - zeros_l;
            val <<= 1;
            if k < zc {
                l = zeros_l;
                r = zeros_r;
            } else {
                val |= 1;
                k -= zc;
                l = self.zeros[lvl] + self.rank1(lvl, l);
                r = self.zeros[lvl] + self.rank1(lvl, r);
            }
        }
        Some(val)
    }

    /// [l, r) で k 番目 (0-based) に大きい値。
    pub fn kth_largest(&self, l: usize, r: usize, k: usize) -> Option<u64> {
        if l >= r || k >= r - l {
            return None;
        }
        self.kth_smallest(l, r, r - l - k - 1)
    }

    /// [l, r) で x 未満の要素数。
    pub fn less_than(&self, mut l: usize, mut r: usize, x: u64) -> usize {
        if l >= r || x == 0 {
            return 0;
        }
        if self.height < 64 && x >= 1u64 << self.height {
            return r - l;
        }
        let mut cnt = 0usize;
        for lvl in 0..self.height as usize {
            let bit = (x >> (self.height as usize - lvl - 1)) & 1;
            let rl = self.rank1(lvl, l);
            let rr = self.rank1(lvl, r);
            let zeros_l = l - rl;
            let zeros_r = r - rr;
            if bit == 1 {
                cnt += zeros_r - zeros_l;
                l = self.zeros[lvl] + rl;
                r = self.zeros[lvl] + rr;
            } else {
                l = zeros_l;
                r = zeros_r;
            }
        }
        cnt
    }

    /// [l, r) において値が [low, high) に含まれる要素数。
    pub fn range_freq(&self, l: usize, r: usize, low: u64, high: u64) -> usize {
        if low >= high || l >= r {
            return 0;
        }
        self.less_than(l, r, high) - self.less_than(l, r, low)
    }

    /// [l, r) で k 番目 (0-based) に小さい値（`kth_smallest` の別名）。
    pub fn quantile(&self, l: usize, r: usize, k: usize) -> Option<u64> {
        self.kth_smallest(l, r, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_freq(a: &[u64], l: usize, r: usize, x: u64) -> usize {
        a[l..r].iter().filter(|&&v| v == x).count()
    }
    fn naive_kth_smallest(a: &[u64], l: usize, r: usize, k: usize) -> Option<u64> {
        if k >= r - l {
            return None;
        }
        let mut v = a[l..r].to_vec();
        v.sort();
        Some(v[k])
    }
    fn naive_less_than(a: &[u64], l: usize, r: usize, x: u64) -> usize {
        a[l..r].iter().filter(|&&v| v < x).count()
    }

    #[test]
    fn known_values() {
        let a = [5u64, 4, 5, 5, 2, 1, 5, 6, 1, 3, 7, 0];
        let wm = WaveletMatrix::new(&a);
        assert_eq!(wm.len(), a.len());
        for i in 0..a.len() {
            assert_eq!(wm.access(i), a[i]);
        }
        assert_eq!(wm.freq(0, a.len(), 5), 4);
        assert_eq!(wm.freq(2, 7, 5), 3);
    }

    #[test]
    fn random_cross_check() {
        let mut x: u64 = 88172645463325252;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for trial in 0..30 {
            let n = 1 + (rng() % 60) as usize;
            let maxv = 1 + (rng() % 16) as u64;
            let a: Vec<u64> = (0..n).map(|_| rng() % maxv).collect();
            let wm = WaveletMatrix::new(&a);
            assert_eq!(wm.len(), n);

            for i in 0..n {
                assert_eq!(wm.access(i), a[i], "trial {trial} access {i}");
            }

            for _ in 0..40 {
                let l = (rng() as usize) % n;
                let r = l + 1 + (rng() as usize) % (n - l);
                let x_val = rng() % (maxv + 2);

                assert_eq!(wm.freq(l, r, x_val), naive_freq(&a, l, r, x_val), "freq");
                assert_eq!(
                    wm.less_than(l, r, x_val),
                    naive_less_than(&a, l, r, x_val),
                    "less_than"
                );

                let low = rng() % (maxv + 2);
                let high = rng() % (maxv + 2);
                let (low, high) = if low <= high {
                    (low, high)
                } else {
                    (high, low)
                };
                let expect_rf = a[l..r].iter().filter(|&&v| v >= low && v < high).count();
                assert_eq!(wm.range_freq(l, r, low, high), expect_rf, "range_freq");

                let k = (rng() as usize) % (r - l);
                assert_eq!(
                    wm.kth_smallest(l, r, k),
                    naive_kth_smallest(&a, l, r, k),
                    "kth_smallest"
                );
                assert_eq!(wm.quantile(l, r, k), wm.kth_smallest(l, r, k), "quantile");

                let mut sorted = a[l..r].to_vec();
                sorted.sort();
                let expect_largest = sorted[sorted.len() - 1 - k];
                assert_eq!(wm.kth_largest(l, r, k), Some(expect_largest), "kth_largest");
            }
            // 範囲外・空区間
            assert_eq!(wm.kth_smallest(0, n, n), None);
            assert_eq!(wm.freq(3, 3, 0), 0);
        }
    }
}
