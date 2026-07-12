//! Mo 法のクエリ順序生成。半開区間 `[l, r)` のクエリを移動量が小さくなる順に並べる。
//!
//! ```
//! use cplib::algo::mo_algorithm::*;
//!
//! let queries = [(0, 3), (2, 5), (1, 4)];
//! let order = mo_order(6, &queries);
//! assert_eq!(order.len(), queries.len());
//! ```

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MoQuery {
    pub l: usize,
    pub r: usize,
    pub index: usize,
}

pub fn mo_order(n: usize, queries: &[(usize, usize)]) -> Vec<usize> {
    mo_order_with_block(n, queries, default_block_size(n, queries.len()))
}

pub fn mo_order_with_block(n: usize, queries: &[(usize, usize)], block: usize) -> Vec<usize> {
    assert!(block > 0);
    let mut ord: Vec<usize> = (0..queries.len()).collect();
    for &(l, r) in queries {
        assert!(l <= r && r <= n);
    }
    ord.sort_by_key(|&i| {
        let (l, r) = queries[i];
        let b = l / block;
        if b % 2 == 0 {
            (b, r)
        } else {
            (b, usize::MAX - r)
        }
    });
    ord
}

pub fn mo_queries(n: usize, queries: &[(usize, usize)]) -> Vec<MoQuery> {
    mo_order(n, queries)
        .into_iter()
        .map(|i| MoQuery { l: queries[i].0, r: queries[i].1, index: i })
        .collect()
}

fn default_block_size(n: usize, q: usize) -> usize {
    if q == 0 {
        return 1;
    }
    let b = (n as f64 / (q as f64).sqrt()).ceil() as usize;
    b.max(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::xorshift::XorShift;

    #[test]
    fn permutation_and_bounds() {
        let queries = [(0, 3), (2, 5), (1, 4), (0, 0)];
        let mut order = mo_order(5, &queries);
        order.sort_unstable();
        assert_eq!(order, vec![0, 1, 2, 3]);
        let qs = mo_queries(5, &queries);
        for q in qs {
            assert_eq!((q.l, q.r), queries[q.index]);
        }
    }

    #[test]
    fn can_answer_distinct_count() {
        let mut rng = XorShift::new(777);
        let n = 50;
        let a: Vec<usize> = (0..n).map(|_| rng.next_range(10) as usize).collect();
        let queries: Vec<(usize, usize)> = (0..80)
            .map(|_| {
                let l = rng.next_range((n + 1) as u64) as usize;
                let r = l + rng.next_range((n + 1 - l) as u64) as usize;
                (l, r)
            })
            .collect();
        let mut freq = vec![0usize; 10];
        let mut distinct = 0usize;
        let (mut l, mut r) = (0usize, 0usize);
        let mut ans = vec![0usize; queries.len()];
        for q in mo_queries(n, &queries) {
            while l > q.l {
                l -= 1;
                if freq[a[l]] == 0 {
                    distinct += 1;
                }
                freq[a[l]] += 1;
            }
            while r < q.r {
                if freq[a[r]] == 0 {
                    distinct += 1;
                }
                freq[a[r]] += 1;
                r += 1;
            }
            while l < q.l {
                freq[a[l]] -= 1;
                if freq[a[l]] == 0 {
                    distinct -= 1;
                }
                l += 1;
            }
            while r > q.r {
                r -= 1;
                freq[a[r]] -= 1;
                if freq[a[r]] == 0 {
                    distinct -= 1;
                }
            }
            ans[q.index] = distinct;
        }
        for (i, &(ql, qr)) in queries.iter().enumerate() {
            let mut seen = [false; 10];
            for &x in &a[ql..qr] {
                seen[x] = true;
            }
            assert_eq!(ans[i], seen.iter().filter(|&&b| b).count());
        }
    }
}

