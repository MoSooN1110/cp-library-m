// source snippet: key=lib_wavelet_matrix3  prefix=lib_wavelet_matrix3

/// This module is bundled automatically.
/// See <https://rsk0315.github.io/library-rs/nekolib/> for documentation.
pub mod nekolib {
    pub mod ds {
        pub mod rs_dict {
            use super::super::traits::count;
            use super::super::traits::find_nth;
            use super::super::utils::buf_range;
            use buf_range::bounds_within;
            use count::Count;
            use find_nth::FindNth;
            use std::fmt::Debug;
            use std::ops::{Range, RangeBounds};
            const WORD_SIZE: usize = 64;
            const WORD_SIZE_2: usize = WORD_SIZE * WORD_SIZE;
            #[derive(Clone, Debug)]
            pub struct RsDict {
                len: usize,
                buf: Vec<u64>,
                rank: Vec<usize>,
                sel0: Vec<SelectPreprocess>,
                sel1: Vec<SelectPreprocess>,
            }
            #[derive(Clone, Debug)]
            enum SelectPreprocess {
                Sparse(Vec<usize>),
                Dense(Range<usize>),
            }
            use SelectPreprocess::{Dense, Sparse};
            impl From<Vec<bool>> for RsDict {
                fn from(buf: Vec<bool>) -> Self {
                    let len = buf.len();
                    let buf = Self::compress_vec_bool(buf);
                    let rank = Self::preprocess_rank(&buf);
                    let sel0 = Self::preprocess_select(&buf, len, 0);
                    let sel1 = Self::preprocess_select(&buf, len, 1);
                    Self {
                        len,
                        buf,
                        rank,
                        sel0,
                        sel1,
                    }
                }
            }
            impl RsDict {
                fn compress_vec_bool(buf: Vec<bool>) -> Vec<u64> {
                    if buf.is_empty() {
                        return vec![];
                    }
                    let n = buf.len();
                    let nc = 1 + (n - 1) / WORD_SIZE;
                    let mut res = vec![0; nc + 1];
                    for i in 0..n {
                        if buf[i] {
                            res[i / WORD_SIZE] |= 1_u64 << (i % WORD_SIZE);
                        }
                    }
                    res
                }
                fn preprocess_rank(buf: &[u64]) -> Vec<usize> {
                    let n = buf.len();
                    let mut res = vec![0; n];
                    for i in 1..n {
                        res[i] = res[i - 1] + buf[i - 1].count_ones() as usize;
                    }
                    res
                }
                fn preprocess_select(buf: &[u64], n: usize, x: u64) -> Vec<SelectPreprocess> {
                    let mut sel = vec![];
                    let mut tmp = vec![];
                    let mut last = 0;
                    for i in 0..n {
                        if buf[i / WORD_SIZE] >> (i % WORD_SIZE) & 1 != x {
                            continue;
                        }
                        if tmp.len() == WORD_SIZE {
                            let len = i - last;
                            if len < WORD_SIZE_2 {
                                sel.push(Dense(last..i));
                            } else {
                                sel.push(Sparse(tmp));
                            }
                            tmp = vec![];
                            last = i;
                        }
                        tmp.push(i);
                    }
                    if !tmp.is_empty() {
                        sel.push(Sparse(tmp));
                    }
                    sel
                }
                pub fn rank(&self, end: usize, x: u64) -> usize {
                    let il = end / WORD_SIZE;
                    let is = end % WORD_SIZE;
                    let rank1 =
                        self.rank[il] + (self.buf[il] & !(!0_u64 << is)).count_ones() as usize;
                    let rank = if x == 0 { end - rank1 } else { rank1 };
                    rank
                }
                pub fn select(&self, x: u64, k: usize) -> Option<usize> {
                    if self.rank(self.len, x) < k {
                        None
                    } else if k == 0 {
                        Some(0)
                    } else {
                        Some(self.find_nth_internal(x, k - 1) + 1)
                    }
                }
            }
            impl Count<u64> for RsDict {
                fn count(&self, r: impl RangeBounds<usize>, x: u64) -> usize {
                    let Range { start, end } = bounds_within(r, self.len);
                    if start > 0 {
                        self.rank(end, x) - self.rank(start, x)
                    } else {
                        self.rank(end, x)
                    }
                }
            }
            impl FindNth<u64> for RsDict {
                fn find_nth(&self, r: impl RangeBounds<usize>, x: u64, n: usize) -> Option<usize> {
                    let Range { start, end } = bounds_within(r, self.len);
                    if self.count(start..end, x) <= n {
                        None
                    } else {
                        let offset = self.rank(start, x);
                        Some(self.find_nth_internal(x, offset + n))
                    }
                }
            }
            impl RsDict {
                fn find_nth_internal(&self, x: u64, n: usize) -> usize {
                    if self.rank(self.len, x) < n {
                        panic!("the number of {}s is less than {}", x, n);
                    }
                    let sel = if x == 0 { &self.sel0 } else { &self.sel1 };
                    let il = n / WORD_SIZE;
                    let is = n % WORD_SIZE;
                    eprintln!("{:?}", sel[il]);
                    match &sel[il] {
                        Sparse(dir) => dir[is],
                        Dense(range) => {
                            let mut lo = range.start / WORD_SIZE;
                            let mut hi = 1 + (range.end - 1) / WORD_SIZE;
                            while hi - lo > 1 {
                                let mid = lo + (hi - lo) / 2;
                                let rank = self.rank_rough(mid, x);
                                *(if rank <= n { &mut lo } else { &mut hi }) = mid;
                            }
                            let rank_frac = n - self.rank_rough(lo, x);
                            lo * WORD_SIZE + Self::find_nth_small(self.buf[lo], x, rank_frac)
                        }
                    }
                }
                fn rank_rough(&self, n: usize, x: u64) -> usize {
                    let rank1 = self.rank[n];
                    let rank = if x == 0 { n * WORD_SIZE - rank1 } else { rank1 };
                    rank
                }
                fn find_nth_small(word: u64, x: u64, n: usize) -> usize {
                    let mut word = if x == 0 { !word } else { word };
                    let mut n = n as u32;
                    let mut res = 0;
                    for &mid in &[32, 16, 8, 4, 2, 1] {
                        let count = (word & !(!0 << mid)).count_ones();
                        if count <= n {
                            n -= count;
                            word >>= mid;
                            res += mid;
                        }
                    }
                    res
                }
            }
        }
        pub use rs_dict::RsDict;
        pub mod wavelet_matrix {
            use super::super::traits::count;
            use super::super::traits::find_nth;
            use super::super::traits::quantile;
            use super::super::utils::buf_range;
            use super::rs_dict;
            use buf_range::bounds_within;
            use count::{Count, Count3way, Count3wayResult};
            use find_nth::FindNth;
            use quantile::Quantile;
            use rs_dict::RsDict;
            use std::ops::{
                Bound::{Excluded, Included, Unbounded},
                Range, RangeBounds,
            };
            pub struct WaveletMatrix {
                len: usize,
                bitlen: usize,
                buf: Vec<RsDict>,
                zeros: Vec<usize>,
            }
            impl From<Vec<u128>> for WaveletMatrix {
                fn from(orig: Vec<u128>) -> Self {
                    if orig.is_empty() {
                        return Self {
                            len: 0,
                            bitlen: 0,
                            buf: vec![],
                            zeros: vec![],
                        };
                    }
                    let len = orig.len();
                    let mut whole = orig.clone();
                    let &max = orig.iter().max().unwrap();
                    let bitlen = if max >= 1 << 127 {
                        128
                    } else {
                        (max + 1).next_power_of_two().trailing_zeros() as usize
                    };
                    let mut zeros = vec![0; bitlen];
                    let mut buf = vec![];
                    for i in (0..bitlen).rev() {
                        let mut zero = vec![];
                        let mut one = vec![];
                        let mut vb = vec![false; len];
                        for j in 0..len {
                            (if whole[j] >> i & 1 == 0 {
                                &mut zero
                            } else {
                                &mut one
                            })
                            .push(whole[j]);
                            vb[j] = whole[j] >> i & 1 != 0;
                        }
                        zeros[i] = zero.len();
                        buf.push(vb.into());
                        whole = zero;
                        whole.append(&mut one);
                    }
                    buf.reverse();
                    Self {
                        len,
                        bitlen,
                        buf,
                        zeros,
                    }
                }
            }
            impl<R: RangeBounds<u128>> Count<R> for WaveletMatrix {
                fn count(&self, range: impl RangeBounds<usize>, value: R) -> usize {
                    self.count_3way(range, value).eq()
                }
            }
            impl<R: RangeBounds<u128>> Count3way<R> for WaveletMatrix {
                fn count_3way(&self, range: impl RangeBounds<usize>, value: R) -> Count3wayResult {
                    let Range { start, end } = bounds_within(range, self.len);
                    let len = end - start;
                    let lt = match value.start_bound() {
                        Included(&x) => self.count_3way_internal(start..end, x).0,
                        Excluded(&std::u128::MAX) => len,
                        Excluded(&x) => self.count_3way_internal(start..end, x + 1).0,
                        Unbounded => 0,
                    };
                    let gt = match value.end_bound() {
                        Included(&x) => self.count_3way_internal(start..end, x).1,
                        Excluded(&0) => len,
                        Excluded(&x) => self.count_3way_internal(start..end, x - 1).1,
                        Unbounded => 0,
                    };
                    let eq = len - (lt + gt);
                    Count3wayResult::new(lt, eq, gt)
                }
            }
            impl WaveletMatrix {
                fn count_3way_internal(
                    &self,
                    Range { mut start, mut end }: Range<usize>,
                    value: u128,
                ) -> (usize, usize) {
                    if start == end {
                        return (0, 0);
                    }
                    let mut lt = 0;
                    let mut gt = 0;
                    for i in (0..self.bitlen).rev() {
                        let tmp = end - start;
                        if value >> i & 1 == 0 {
                            start = self.buf[i].rank(start, 0);
                            end = self.buf[i].rank(end, 0);
                        } else {
                            start = self.zeros[i] + self.buf[i].rank(start, 1);
                            end = self.zeros[i] + self.buf[i].rank(end, 1);
                        }
                        *(if value >> i & 1 == 0 {
                            &mut gt
                        } else {
                            &mut lt
                        }) += tmp - (end - start);
                    }
                    (lt, gt)
                }
            }
            impl Quantile for WaveletMatrix {
                type Output = u128;
                fn quantile(&self, range: impl RangeBounds<usize>, mut n: usize) -> Option<u128> {
                    let Range { mut start, mut end } = bounds_within(range, self.len);
                    if end - start <= n {
                        return None;
                    }
                    let mut res = 0;
                    for i in (0..self.bitlen).rev() {
                        let z = self.buf[i].count(start..end, 0);
                        if n < z {
                            start = self.buf[i].rank(start, 0);
                            end = self.buf[i].rank(end, 0);
                        } else {
                            res |= 1_u128 << i;
                            start = self.zeros[i] + self.buf[i].rank(start, 1);
                            end = self.zeros[i] + self.buf[i].rank(end, 1);
                            n -= z;
                        }
                    }
                    Some(res)
                }
            }
            impl WaveletMatrix {
                pub fn xored_quantile(
                    &self,
                    range: impl RangeBounds<usize>,
                    mut n: usize,
                    x: u128,
                ) -> Option<u128> {
                    let Range { mut start, mut end } = bounds_within(range, self.len);
                    if end - start <= n {
                        return None;
                    }
                    let mut res = 0;
                    for i in (0..self.bitlen).rev() {
                        let z = self.buf[i].count(start..end, 0);
                        if x >> i & 1 == 0 {
                            if n < z {
                                start = self.buf[i].rank(start, 0);
                                end = self.buf[i].rank(end, 0);
                            } else {
                                res |= 1_u128 << i;
                                start = self.zeros[i] + self.buf[i].rank(start, 1);
                                end = self.zeros[i] + self.buf[i].rank(end, 1);
                                n -= z;
                            }
                        } else {
                            let z = (end - start) - z;
                            if n < z {
                                start = self.zeros[i] + self.buf[i].rank(start, 1);
                                end = self.zeros[i] + self.buf[i].rank(end, 1);
                            } else {
                                res |= 1_u128 << i;
                                start = self.buf[i].rank(start, 0);
                                end = self.buf[i].rank(end, 0);
                                n -= z;
                            }
                        }
                    }
                    Some(res)
                }
            }
            impl FindNth<u128> for WaveletMatrix {
                fn find_nth(
                    &self,
                    range: impl RangeBounds<usize>,
                    value: u128,
                    n: usize,
                ) -> Option<usize> {
                    let start = bounds_within(range, self.len).start;
                    let (lt, gt) = self.count_3way_internal(0..start, value);
                    let offset = start - (lt + gt);
                    Some(self.select(value, n + offset + 1)? - 1)
                }
            }
            impl WaveletMatrix {
                pub fn rank(&self, end: usize, value: u128) -> usize {
                    self.count(0..end, value..=value)
                }
                pub fn select(&self, value: u128, mut n: usize) -> Option<usize> {
                    if n == 0 {
                        return Some(0);
                    }
                    let (lt, gt) = self.count_3way_internal(0..self.len, value);
                    let count = self.len - (lt + gt);
                    if count < n {
                        return None;
                    }
                    let si = self.start_pos(value);
                    let value0 = (value & 1) as u64;
                    n += self.buf[0].rank(si, value0);
                    n = self.buf[0].select(value0, n).unwrap();
                    for i in 1..self.bitlen {
                        if value >> i & 1 == 0 {
                            n = self.buf[i].select(0, n).unwrap();
                        } else {
                            n -= self.zeros[i];
                            n = self.buf[i].select(1, n).unwrap();
                        }
                    }
                    Some(n)
                }
                fn start_pos(&self, value: u128) -> usize {
                    let mut start = 0;
                    let mut end = 0;
                    for i in (1..self.bitlen).rev() {
                        if value >> i & 1 == 0 {
                            start = self.buf[i].rank(start, 0);
                            end = self.buf[i].rank(end, 0);
                        } else {
                            start = self.zeros[i] + self.buf[i].rank(start, 1);
                            end = self.zeros[i] + self.buf[i].rank(end, 1);
                        }
                    }
                    start
                }
            }
        }
        pub use wavelet_matrix::WaveletMatrix;
    }
    pub mod traits {
        pub mod count {
            use std::fmt::Debug;
            use std::ops::RangeBounds;
            pub trait Count<I> {
                fn count(&self, range: impl RangeBounds<usize>, value: I) -> usize;
            }
            pub trait Count3way<I> {
                fn count_3way(&self, range: impl RangeBounds<usize>, value: I) -> Count3wayResult;
            }
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub struct Count3wayResult {
                lt: usize,
                eq: usize,
                gt: usize,
            }
            impl Count3wayResult {
                pub fn new(lt: usize, eq: usize, gt: usize) -> Self {
                    Self { lt, eq, gt }
                }
                pub fn lt(&self) -> usize {
                    self.lt
                }
                pub fn eq(&self) -> usize {
                    self.eq
                }
                pub fn gt(&self) -> usize {
                    self.gt
                }
                pub fn le(&self) -> usize {
                    self.lt + self.eq
                }
                pub fn ge(&self) -> usize {
                    self.gt + self.eq
                }
                pub fn ne(&self) -> usize {
                    self.lt + self.gt
                }
            }
        }
        pub use count::{Count, Count3way};
        pub mod find_nth {
            use std::ops::RangeBounds;
            pub trait FindNth<I> {
                fn find_nth(
                    &self,
                    range: impl RangeBounds<usize>,
                    value: I,
                    n: usize,
                ) -> Option<usize>;
            }
        }
        pub use find_nth::FindNth;
        pub mod quantile {
            use std::ops::RangeBounds;
            pub trait Quantile {
                type Output;
                fn quantile(
                    &self,
                    range: impl RangeBounds<usize>,
                    n: usize,
                ) -> Option<Self::Output>;
            }
        }
        pub use quantile::Quantile;
    }
    pub mod utils {
        pub mod buf_range {
            use std::ops::Bound::{Excluded, Included, Unbounded};
            use std::ops::{Range, RangeBounds};
            pub fn bounds_within<R: RangeBounds<usize>>(r: R, len: usize) -> Range<usize> {
                let e_ex = match r.end_bound() {
                    Included(&e) => e + 1,
                    Excluded(&e) => e,
                    Unbounded => len,
                }
                .min(len);
                let s_in = match r.start_bound() {
                    Included(&s) => s,
                    Excluded(&s) => s + 1,
                    Unbounded => 0,
                }
                .min(e_ex);
                s_in..e_ex
            }
        }
        pub use buf_range::bounds_within;
    }
}
