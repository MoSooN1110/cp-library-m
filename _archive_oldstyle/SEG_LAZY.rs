// source snippet: key=SEG_LAZY  prefix=SEG_LAZY
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
/// Lazy Segment Tree
pub struct SEG<T: SEGimpl> {
    n: usize,
    buf: Vec<T::Elem>,
    zero: T::Elem,
    phantom: std::marker::PhantomData<T>,
}
impl<T: SEGimpl> SEG<T> {
    #[allow(dead_code)]
    pub fn new(n: usize, zero: T::Elem) -> SEG<T> {
        let n = (1..).map(|i| 1 << i).find(|&x| x >= n).unwrap();
        SEG {
            n,
            buf: vec![zero.clone(); 2 * n],
            zero,
            phantom: std::marker::PhantomData,
        }
    }
    #[allow(dead_code)]
    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if r - l > 1 {
            let (l, r) = self.buf.split_at_mut(2 * k + 1);
            let (c1, c2) = r.split_at_mut(1);
            T::eval(&mut l[k], Some((&mut c1[0], &mut c2[0])));
        } else {
            T::eval(&mut self.buf[k], None);
        }
    }
    #[allow(clippy::many_single_char_names)]
    #[allow(dead_code)]
    pub fn update(&mut self, i: usize, x: T::Elem) {
        let mut k = i + self.n - 1;
        self.buf[k] = x;
        self.eval(k, i, i + 1);
        while k > 0 {
            k = (k - 1) / 2;
            let (l, r) = self.buf.split_at_mut(2 * k + 1);
            let (c1, c2) = r.split_at_mut(1);
            T::reduce(&mut l[k], &c1[0], &c2[0]);
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, i: usize) -> Option<T::R> {
        self.query(i, i + 1)
    }
    #[allow(dead_code)]
    #[allow(clippy::many_single_char_names)]
    fn r(&mut self, x: &T::A, a: usize, b: usize, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            T::range(x, &mut self.buf[k], l, r);
            self.eval(k, l, r);
            return;
        }
        self.r(x, a, b, 2 * k + 1, l, (l + r) / 2);
        self.r(x, a, b, 2 * k + 2, (l + r) / 2, r);
        let (l, r) = self.buf.split_at_mut(2 * k + 1);
        let (c1, c2) = r.split_at_mut(1);
        T::reduce(&mut l[k], &c1[0], &c2[0]);
    }
    #[allow(dead_code)]
    pub fn range_add(&mut self, x: &T::A, a: usize, b: usize) {
        let n = self.n;
        self.r(x, a, b, 0, 0, n);
    }
    #[allow(dead_code)]
    pub fn add(&mut self, x: &T::A, i: usize) {
        self.range_add(x, i, i + 1);
    }
    #[allow(dead_code)]
    #[allow(clippy::many_single_char_names)]
    fn q(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> Option<T::Elem> {
        self.eval(k, l, r);
        if r <= a || b <= l {
            return None;
        }
        if a <= l && r <= b {
            Some(self.buf[k].clone())
        } else {
            let vl = self.q(a, b, k * 2 + 1, l, (l + r) / 2);
            let vr = self.q(a, b, k * 2 + 2, (l + r) / 2, r);
            match (vl, vr) {
                (Some(l), Some(r)) => {
                    let mut res = self.zero.clone();
                    T::reduce(&mut res, &l, &r);
                    Some(res)
                }
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                _ => None,
            }
        }
    }
    #[allow(dead_code)]
    pub fn query(&mut self, a: usize, b: usize) -> Option<T::R> {
        let n = self.n;
        self.q(a, b, 0, 0, n).map(T::to_result)
    }
}
pub trait SEGimpl {
    type Elem: Clone;
    type A;
    type R;
    fn eval(parent: &mut Self::Elem, children: Option<(&mut Self::Elem, &mut Self::Elem)>);
    fn range(x: &Self::A, elem: &mut Self::Elem, l: usize, r: usize);
    fn reduce(parent: &mut Self::Elem, c1: &Self::Elem, c2: &Self::Elem);
    fn to_result(elem: Self::Elem) -> Self::R;
}
