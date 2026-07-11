//! ダブリング（functional graph 上で k 個先を O(log k) で求める）。
//!
//! ```
//! use cplib::graph::doubling::*;
//! // next[i] = (i+1) % 5 のサイクル
//! let nxt: Vec<usize> = (0..5).map(|i| (i + 1) % 5).collect();
//! let d = Doubling::new(&nxt, 1_000_000_000);
//! assert_eq!(d.kth(0, 7), 2);   // 0 から 7 歩 = (7 % 5) = 2
//! ```

pub struct Doubling {
    table: Vec<Vec<u32>>,
    log: usize,
}

impl Doubling {
    /// `next[i]` は i の 1 歩先（0..n）。`max_k` 歩まで対応。
    pub fn new(next: &[usize], max_k: u64) -> Self {
        let n = next.len();
        let mut log = 1;
        while (1u64 << log) <= max_k {
            log += 1;
        }
        let mut table = vec![vec![0u32; n]; log];
        for i in 0..n {
            table[0][i] = next[i] as u32;
        }
        for k in 1..log {
            for i in 0..n {
                table[k][i] = table[k - 1][table[k - 1][i] as usize];
            }
        }
        Doubling { table, log }
    }

    /// start から k 歩先の頂点
    pub fn kth(&self, mut start: usize, k: u64) -> usize {
        for i in 0..self.log {
            if (k >> i) & 1 == 1 {
                start = self.table[i][start] as usize;
            }
        }
        start
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cycle_and_chain() {
        let nxt: Vec<usize> = (0..6).map(|i| (i + 1) % 6).collect();
        let d = Doubling::new(&nxt, 1_000_000_000_000);
        for start in 0..6 {
            for k in 0..30u64 {
                assert_eq!(d.kth(start, k), ((start as u64 + k) % 6) as usize);
            }
        }
        // 自己ループ終端
        let nxt2 = vec![1, 2, 3, 3]; // 3 は自己ループ
        let d2 = Doubling::new(&nxt2, 100);
        assert_eq!(d2.kth(0, 100), 3);
    }
}
