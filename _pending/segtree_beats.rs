// source snippet: key=lib_segtree_beats  prefix=lib_segtree_beats

// ---------- begin segment tree beats ----------
pub trait Beats {
    type Value: Clone;
    fn merge(l: &Self::Value, r: &Self::Value) -> Self::Value;
    fn propagate(p: &mut Self::Value, c: &mut [Self::Value]);
    fn e() -> Self::Value;
}

pub trait Effect {
    type Value;
    type Query;
    fn break_cond(x: &Self::Value, q: &Self::Query) -> bool;
    fn tag_cond(x: &Self::Value, q: &Self::Query) -> bool;
    fn update(x: &mut Self::Value, q: &Self::Query);
}

struct SegmentTreeBeats<R: Beats> {
    size: usize,
    val: Box<[R::Value]>,
    dfs: Vec<(usize, usize, usize)>,
}

impl<R: Beats> SegmentTreeBeats<R> {
    fn new(ini: &[R::Value]) -> Self {
        let size = ini.len().next_power_of_two();
        let val = vec![R::e(); 2 * size];
        let mut val = val.into_boxed_slice();
        val[size..(size + ini.len())].clone_from_slice(ini);
        for i in (1..size).rev() {
            val[i] = R::merge(&val[2 * i], &val[2 * i + 1]);
        }
        SegmentTreeBeats {
            size,
            val,
            dfs: vec![],
        }
    }
    fn update<E>(&mut self, x: usize, y: usize, f: E::Query)
    where
        E: Effect<Value = R::Value>,
    {
        assert!(x <= y && y <= self.size);
        if x >= y {
            return;
        }
        let val = &mut self.val;
        self.dfs.push((1, 0, self.size));
        while let Some((k, l, r)) = self.dfs.pop() {
            if k > 2 * self.size {
                let k = !k;
                val[k] = R::merge(&val[2 * k], &val[2 * k + 1]);
                continue;
            }
            if E::break_cond(&val[k], &f) {
                continue;
            }
            if x <= l && r <= y && E::tag_cond(&val[k], &f) {
                E::update(&mut val[k], &f);
                continue;
            }
            let (a, b) = val.split_at_mut(2 * k);
            R::propagate(&mut a[k], &mut b[..2]);
            self.dfs.push((!k, l, r));
            let m = (l + r) / 2;
            if m < y {
                self.dfs.push((2 * k + 1, m, r));
            }
            if x < m {
                self.dfs.push((2 * k, l, m));
            }
        }
    }
    fn find(&mut self, x: usize, y: usize) -> R::Value {
        assert!(x <= y && y <= self.size);
        let mut res = R::e();
        if x >= y {
            return res;
        }
        let val = &mut self.val;
        self.dfs.push((1, 0, self.size));
        while let Some((k, l, r)) = self.dfs.pop() {
            if x <= l && r <= y {
                res = R::merge(&res, &val[k]);
                continue;
            }
            let (a, b) = val.split_at_mut(2 * k);
            R::propagate(&mut a[k], &mut b[..2]);
            let m = (l + r) / 2;
            if m < y {
                self.dfs.push((2 * k + 1, m, r));
            }
            if x < m {
                self.dfs.push((2 * k, l, m));
            }
        }
        res
    }
}
// ---------- end segment tree beats ----------
// const INF: i64 = 1_000_000_000_000_000_000;

#[derive(Clone)]
struct Value {
    max: i64,
    max_cnt: i64,
    second_max: i64,
    min: i64,
    min_cnt: i64,
    second_min: i64,
    add: i64,
    sum: i64,
    len: i64,
}

impl Value {
    fn one(v: i64) -> Self {
        Value {
            max: v,
            max_cnt: 1,
            second_max: -INF,
            min: v,
            min_cnt: 1,
            second_min: INF,
            add: 0,
            sum: v,
            len: 1,
        }
    }
    fn update_set(&mut self, v: i64) {
        self.max = v;
        self.max_cnt = self.len;
        self.second_max = -INF;
        self.min = v;
        self.min_cnt = self.len;
        self.second_min = INF;
        self.add = 0;
        self.sum = self.len * v;
    }
    fn update_add(&mut self, v: i64) {
        self.max += v;
        self.second_max += v;
        self.min += v;
        self.second_min += v;
        self.add += v;
        self.sum += v * self.len;
    }
}

struct RCCARS;
impl Beats for RCCARS {
    type Value = Value;
    fn merge(l: &Self::Value, r: &Self::Value) -> Self::Value {
        use std::cmp::Ordering::*;
        let (max, max_cnt, second_max) = match l.max.cmp(&r.max) {
            Equal => (l.max, l.max_cnt + r.max_cnt, l.second_max.max(r.second_max)),
            Less => (r.max, r.max_cnt, r.second_max.max(l.max)),
            Greater => (l.max, l.max_cnt, l.second_max.max(r.max)),
        };
        let (min, min_cnt, second_min) = match l.min.cmp(&r.min) {
            Equal => (l.min, l.min_cnt + r.min_cnt, l.second_min.min(r.second_min)),
            Greater => (r.min, r.min_cnt, r.second_min.min(l.min)),
            Less => (l.min, l.min_cnt, l.second_min.min(r.min)),
        };
        let sum = l.sum + r.sum;
        let len = l.len + r.len;
        Value {
            max,
            max_cnt,
            second_max,
            min,
            min_cnt,
            second_min,
            add: 0,
            sum,
            len,
        }
    }
    fn propagate(p: &mut Self::Value, c: &mut [Self::Value]) {
        let add = p.add;
        p.add = 0;
        for c in c.iter_mut() {
            c.update_add(add);
            if p.max < c.max {
                Chmin::update(c, &p.max);
            }
            if p.min > c.min {
                Chmax::update(c, &p.min);
            }
        }
    }
    fn e() -> Self::Value {
        Value {
            max: -INF,
            max_cnt: 0,
            second_max: -2 * INF,
            min: INF,
            min_cnt: 0,
            second_min: 2 * INF,
            add: 0,
            sum: 0,
            len: 0,
        }
    }
}

struct Chmin;
impl Effect for Chmin {
    type Value = Value;
    type Query = i64;
    fn break_cond(x: &Self::Value, q: &Self::Query) -> bool {
        x.max <= *q
    }
    fn tag_cond(x: &Self::Value, q: &Self::Query) -> bool {
        x.second_max < *q && *q <= x.max
    }
    fn update(x: &mut Self::Value, q: &Self::Query) {
        if x.max == x.min {
            x.update_set(*q);
            return;
        }
        if x.max == x.second_min {
            x.second_min = *q;
        }
        x.sum += (*q - x.max) * x.max_cnt;
        x.max = *q;
    }
}

struct Chmax;
impl Effect for Chmax {
    type Value = Value;
    type Query = i64;
    fn break_cond(x: &Self::Value, q: &Self::Query) -> bool {
        x.min >= *q
    }
    fn tag_cond(x: &Self::Value, q: &Self::Query) -> bool {
        x.second_min > *q && *q >= x.min
    }
    fn update(x: &mut Self::Value, q: &Self::Query) {
        if x.max == x.min {
            x.update_set(*q);
            return;
        }
        if x.min == x.second_max {
            x.second_max = *q;
        }
        x.sum += (*q - x.min) * x.min_cnt;
        x.min = *q;
    }
}

struct RangeAdd;
impl Effect for RangeAdd {
    type Value = Value;
    type Query = i64;
    fn break_cond(_: &Self::Value, _: &Self::Query) -> bool {
        false
    }
    fn tag_cond(_: &Self::Value, _: &Self::Query) -> bool {
        true
    }
    fn update(x: &mut Self::Value, q: &Self::Query) {
        x.update_add(*q);
    }
}

// fn run<R: std::io::BufRead>(sc: &mut scanner::Scanner<R>) {
//     let out = std::io::stdout();
//     let mut out = std::io::BufWriter::new(out.lock());
//     let n: usize = sc.next(b' ');
//     let q: usize = sc.next(b'\n');
//     let mut ini = Vec::with_capacity(n);
//     for i in 0..n {
//         let d = if i == n - 1 { b'\n' } else { b' ' };
//         let v: i64 = sc.next(d);
//         ini.push(Value::one(v));
//     }
//     let mut seg = SegmentTreeBeats::<RCCARS>::new(&ini);
//     for _ in 0..q {
//         let op: u8 = sc.next(b' ');
//         let l: usize = sc.next(b' ');
//         let r: usize = sc.next(if op == 3 { b'\n' } else { b' ' });
//         let b: i64 = if op == 3 { 0 } else { sc.next(b'\n') };
//         if op == 0 {
//             seg.update::<Chmin>(l, r, b);
//         } else if op == 1 {
//             seg.update::<Chmax>(l, r, b);
//         } else if op == 2 {
//             seg.update::<RangeAdd>(l, r, b);
//         } else if op == 3 {
//             let ans = seg.find(l, r).sum;
//             writeln!(out, "{}", ans).ok();
//         } else {
//             unreachable!();
//         }
//     }
// }
