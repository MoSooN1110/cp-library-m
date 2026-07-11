// source snippet: key=lib_seg_lazy  prefix=lib_seg_lazy

trait SEGLazyImpl {
    type Monoid: Copy;
    type OperatorMonoid: Copy + PartialEq;
    fn m0() -> Self::Monoid;
    fn om0() -> Self::OperatorMonoid;
    fn f(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid;
    fn g(x: Self::Monoid, y: Self::OperatorMonoid, weight: usize) -> Self::Monoid;
    fn h(x: Self::OperatorMonoid, y: Self::OperatorMonoid) -> Self::OperatorMonoid;
}

struct SEGLazy<T: SEGLazyImpl> {
    n: usize,
    data: Vec<T::Monoid>,
    lazy: Vec<T::OperatorMonoid>,
    weight: Vec<usize>,
}

impl<T: SEGLazyImpl> SEGLazy<T> {
    pub fn new(n: usize, init: T::Monoid) -> SEGLazy<T> {
        let weights = vec![1; n];
        Self::with_weight(n, init, weights)
    }
    pub fn with_weight(n: usize, init: T::Monoid, weights: Vec<usize>) -> Self {
        let mut m = 1;
        while m < n {
            m *= 2;
        }
        SEGLazy {
            n: m,
            data: vec![init; m * 2],
            lazy: vec![T::om0(); m * 2],
            weight: Self::mk_weight(&weights),
        }
    }
    fn mk_weight(xs: &[usize]) -> Vec<usize> {
        let n = xs.len();
        let mut m = 1;
        while m < n {
            m *= 2;
        }
        let mut res = vec![0; 2 * m];
        for i in 0..n {
            res[m + i] = xs[i];
        }
        for k in (1..m).rev() {
            let l = 2 * k;
            let r = 2 * k + 1;
            res[k] = res[l] + res[r];
        }
        res
    }
    fn propagate(&mut self, k: usize) {
        let weight = self.weight[k];
        if self.lazy[k] != T::om0() {
            if k < self.n {
                self.lazy[2 * k + 0] = T::h(self.lazy[2 * k + 0], self.lazy[k]);
                self.lazy[2 * k + 1] = T::h(self.lazy[2 * k + 1], self.lazy[k]);
            }
            self.data[k] = T::g(self.data[k], self.lazy[k], weight);
            self.lazy[k] = T::om0();
        }
    }
    fn do_update(
        &mut self,
        a: usize,
        b: usize,
        x: T::OperatorMonoid,
        k: usize,
        l: usize,
        r: usize,
    ) -> T::Monoid {
        self.propagate(k);
        if r <= a || b <= l {
            self.data[k]
        } else if a <= l && r <= b {
            self.lazy[k] = T::h(self.lazy[k], x);
            self.propagate(k);
            self.data[k]
        } else {
            self.data[k] = T::f(
                self.do_update(a, b, x, 2 * k + 0, l, (l + r) >> 1),
                self.do_update(a, b, x, 2 * k + 1, (l + r) >> 1, r),
            );
            self.data[k]
        }
    }
    #[doc = "[l,r)"]
    pub fn update(&mut self, l: usize, r: usize, x: T::OperatorMonoid) -> T::Monoid {
        let n = self.n;
        self.do_update(l, r, x, 1, 0, n)
    }
    fn do_query(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T::Monoid {
        self.propagate(k);
        if r <= a || b <= l {
            T::m0()
        } else if a <= l && r <= b {
            self.data[k]
        } else {
            T::f(
                self.do_query(a, b, 2 * k + 0, l, (l + r) >> 1),
                self.do_query(a, b, 2 * k + 1, (l + r) >> 1, r),
            )
        }
    }
    #[doc = "[l,r)"]
    pub fn query(&mut self, l: usize, r: usize) -> T::Monoid {
        let n = self.n;
        self.do_query(l, r, 1, 0, n)
    }
}

struct RUQ;
impl SEGLazyImpl for RUQ {
    type Monoid = i64;
    type OperatorMonoid = i64;
    fn m0() -> Self::Monoid {
        0
    }
    fn om0() -> Self::OperatorMonoid {
        0
    }
    fn f(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
        std::cmp::max(x, y)
    }
    fn g(x: Self::Monoid, y: Self::OperatorMonoid, _: usize) -> Self::Monoid {
        y
    }
    fn h(x: Self::OperatorMonoid, y: Self::OperatorMonoid) -> Self::OperatorMonoid {
        y
    }
}
// #[test]
// fn test_MAX_RUQ() {
//     let mut seg: SEGLazy<MAX_RUQ> = SEGLazy::new(10, MAX_RUQ::m0());
//     assert_eq!(seg.query(0, 3), 0);
//     seg.update(0, 2, 10); // [10,10,0,...]
//     assert_eq!(seg.query(0, 3), 10);
//     assert_eq!(seg.query(2, 3), 0);
//     seg.update(1, 5, 20);
//     assert_eq!(seg.query(0, 3), 20);
//     assert_eq!(seg.query(0, 1), 10);
//     seg.update(0, 1, 5);
//     assert_eq!(seg.query(0, 1), 5);
// }
