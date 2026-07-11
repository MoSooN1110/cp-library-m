// source snippet: key=SEG  prefix=SEG
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
pub trait Monoid {
    type T: Clone;
    fn id() -> Self::T;
    fn op(a: &Self::T, b: &Self::T) -> Self::T;
}
#[allow(dead_code)]
pub enum SUM {}
impl Monoid for SUM {
    type T = u64;
    fn id() -> Self::T {
        0
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        *a + *b
    }
}
#[allow(dead_code)]
/// Segment Tree
pub struct SEG<M: Monoid> {
    n: usize,
    buf: Vec<M::T>,
}
impl<M: Monoid> SEG<M> {
    #[allow(dead_code)]
    pub fn new(n: usize) -> SEG<M> {
        SEG {
            n,
            buf: vec![M::id(); 2 * n],
        }
    }
    #[allow(dead_code)]
    pub fn update(&mut self, k: usize, a: M::T) {
        let mut k = k + self.n;
        self.buf[k] = a;
        while k > 0 {
            k >>= 1;
            self.buf[k] = M::op(&self.buf[k << 1], &self.buf[(k << 1) | 1]);
        }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, k: usize, a: &M::T) {
        let mut k = k + self.n;
        self.buf[k] = M::op(&self.buf[k], a);
        while k > 0 {
            k >>= 1;
            self.buf[k] = M::op(&self.buf[k << 1], &self.buf[(k << 1) | 1]);
        }
    }
    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> M::T {
        self.query(i, i + 1)
    }
    #[allow(dead_code)]
    pub fn query_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> M::T {
        let l = match range.start_bound() {
            std::ops::Bound::Excluded(&x) => {
                assert!(x > 0);
                x - 1
            }
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Excluded(&x) => x,
            std::ops::Bound::Included(&x) => (x + 1),
            std::ops::Bound::Unbounded => self.n,
        };
        self.query(l, r)
    }
    #[allow(dead_code)]
    pub fn query(&self, l: usize, r: usize) -> M::T {
        let mut vl = M::id();
        let mut vr = M::id();
        let mut l = l + self.n;
        let mut r = r + self.n;
        while l < r {
            if l & 1 == 1 {
                vl = M::op(&vl, &self.buf[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr = M::op(&self.buf[r], &vr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&vl, &vr)
    }
}
