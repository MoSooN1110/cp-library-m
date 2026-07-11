// source snippet: key=lib_inversion_number  prefix=lib_inversion_number

#[allow(dead_code)]
pub trait Monoid {
    type T: Clone;
    fn id() -> Self::T;
    fn op(a: &Self::T, b: &Self::T) -> Self::T;
}
#[allow(dead_code)]
pub enum SUM {}
impl Monoid for SUM {
    type T = usize;
    fn id() -> Self::T {
        0
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        *a + *b
    }
}
#[allow(dead_code)]
/// Generic Binary Indexed Tree
pub struct BIT<M: Monoid> {
    buf: Vec<M::T>,
}
impl<M: Monoid> BIT<M> {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![M::id(); n + 1],
        }
    }
    #[allow(dead_code)]
    pub fn sum(&self, i: usize) -> M::T {
        let mut i = i;
        let mut s = M::id();
        while i > 0 {
            s = M::op(&s, &self.buf[i]);
            i &= i - 1;
        }
        s
    }
    #[allow(dead_code)]
    pub fn add(&mut self, i: usize, x: &M::T) {
        let mut i = i as i64;
        while i < self.buf.len() as i64 {
            let t = &mut self.buf[i as usize];
            *t = M::op(&t, x);
            i += i & -i;
        }
    }
}

// 0 ~ n -1
fn inversion_number(v: &Vec<usize>) -> usize {
    let mut res = 0;
    let n = v.len();
    let mut bit: BIT<SUM> = BIT::new(n);
    let mut res = 0 as usize;
    for i in 0..n {
        res += bit.sum((n - v[i]));
        bit.add(n - v[i], &1);
    }
    res
}
