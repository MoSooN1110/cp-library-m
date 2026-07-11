// source snippet: key=tech_rectangle_union  prefix=tech_rectangle_union

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
    type Monoid = (i64, usize);
    type OperatorMonoid = (i64);
    fn m0() -> Self::Monoid {
        (INF, 0)
    }
    fn om0() -> Self::OperatorMonoid {
        0
    }
    fn f(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
        if x.0 < y.0 {
            x
        } else if x.0 > y.0 {
            y
        } else {
            (x.0, x.1 + y.1)
        }
    }
    fn g(x: Self::Monoid, y: Self::OperatorMonoid, weight: usize) -> Self::Monoid {
        (x.0 + y, x.1)
    }
    fn h(x: Self::OperatorMonoid, y: Self::OperatorMonoid) -> Self::OperatorMonoid {
        x + y
    }
}

fn get_rectangle_union_area(
    rectangle_data: &Vec<((usize, usize), (usize, usize))>,
    n: usize,
    max_width: usize,
    max_height: usize,
) -> usize {
    //座圧版はライブラリチェッカーを見ること
    let mut seg: SEGLazy<RUQ> = SEGLazy::new(max_height + 1, (0, 1));
    let mut plane_search = BTreeMap::new();
    enum Operation {
        Add,
        Remove,
    }
    for i in 0..max_height + 1 {
        seg.update(i, i + 1, 0);
    }
    // seg.update(0, max_height + 1, 0);
    // seg.query(0, max_height + 1);
    for i in 0..n {
        if !plane_search.contains_key(&rectangle_data[i].0 .0) {
            plane_search.insert(rectangle_data[i].0 .0, vec![]);
        }
        if !plane_search.contains_key(&rectangle_data[i].1 .0) {
            plane_search.insert(rectangle_data[i].0 .1, vec![]);
        }
        plane_search
            .get_mut(&rectangle_data[i].0 .0)
            .unwrap()
            .push((
                ((rectangle_data[i].1).0, rectangle_data[i].1 .1),
                Operation::Add,
            ));
        plane_search
            .get_mut(&rectangle_data[i].0 .1)
            .unwrap()
            .push((
                ((rectangle_data[i].1).0, rectangle_data[i].1 .1),
                Operation::Remove,
            ));
        // plane_search.insert(
        //     rectangle_data[i].0 .0,
        //     (
        //         ((rectangle_data[i].1).0, rectangle_data[i].1 .1),
        //         Operation::Add,
        //     ),
        // );
        // plane_search.insert(
        //     rectangle_data[i].1 .0,
        //     (
        //         ((rectangle_data[i].0).0, rectangle_data[i].0 .1),
        //         Operation::Remove,
        //     ),
        // );
    }
    let mut res = 0;
    // dbg!(seg.query(0, max_height + 1));
    for w in 0..max_width {
        if let Some(operations) = plane_search.get(&w) {
            for (pos, op) in operations {
                // dbg!(pos);
                match op {
                    Operation::Add => {
                        seg.update(pos.0, pos.1, 1);
                        // d!(("add", (pos.0, pos.1)));
                    }
                    Operation::Remove => {
                        seg.update(pos.0, pos.1, -1);
                        // d!(("remove", (pos.0, pos.1)));
                    }
                }
            }
        }
        // dbg!(seg.query(0, max_height + 1));
        if seg.query(0, max_height + 1).0 == 0 {
            res += max_height + 1 - seg.query(0, max_height + 1).1;
        }
    }

    res
}
