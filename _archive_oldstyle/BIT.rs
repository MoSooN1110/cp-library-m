// source snippet: key=BIT  prefix=BIT
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
