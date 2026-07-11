// source snippet: key=lib_fps  prefix=lib_fps

const MODU: usize = 998244353; // 119 * (1 << 23) + 1
const RANK: usize = 23;
const PRIMITIVE_ROOT: usize = 3;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ModInt {
    value: usize,
}

impl ModInt {
    pub fn new(value: usize) -> ModInt {
        ModInt {
            value: value % MODU,
        }
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn inverse(&self) -> ModInt {
        // (a, b) -> (x, y) s.t. a * x + b * y = gcd(a, b)
        fn extended_gcd(a: isize, b: isize) -> (isize, isize) {
            if (a, b) == (1, 0) {
                (1, 0)
            } else {
                let (x, y) = extended_gcd(b, a % b);
                (y, x - (a / b) * y)
            }
        }

        let (x, _y) = extended_gcd(self.value() as isize, MODU as isize);
        ModInt::new((MODU as isize + x) as usize)
    }

    // a^n
    pub fn pow(&self, mut n: usize) -> ModInt {
        let mut res = ModInt::new(1);
        let mut x = *self;
        while n > 0 {
            if n % 2 == 1 {
                res = res * x;
            }
            x = x * x;
            n /= 2;
        }

        res
    }
}

impl ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, other: Self) -> Self {
        ModInt::new(self.value + other.value)
    }
}

impl ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, other: Self) -> Self {
        ModInt::new(MODU as usize + self.value - other.value)
    }
}

impl ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, other: Self) -> Self {
        ModInt::new(self.value * other.value)
    }
}

impl ops::Div for ModInt {
    type Output = ModInt;
    fn div(self, other: Self) -> Self {
        self * other.inverse()
    }
}

impl ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl ops::DivAssign for ModInt {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl fmt::Display for ModInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
#[macro_export]
macro_rules! m {
    ($x:expr) => {
        ModInt::new((MODU as isize + $x as isize) as usize)
    };
}

#[macro_export]
macro_rules! fps {
    ( $( $x:expr ),* ) => {
        FPS::new(vec![$(ModInt::new((MODU as isize + $x as isize) as usize)),*])
    };
    ( $x:expr ; n:expr ) => {
        FPS::new(vec![ModInt::new((MODU as isize + $x as isize) as usize); n])
    };
}
pub struct FftCache {
    rate: Vec<ModInt>,
    irate: Vec<ModInt>,
}

impl FftCache {
    pub fn new() -> Self {
        let mut root = vec![ModInt::new(0); RANK + 1];
        let mut iroot = vec![ModInt::new(0); RANK + 1];
        let mut rate = vec![ModInt::new(0); RANK - 1];
        let mut irate = vec![ModInt::new(0); RANK - 1];

        root[RANK] = ModInt::new(PRIMITIVE_ROOT).pow((MODU - 1) >> RANK);
        iroot[RANK] = root[RANK].inverse();
        for i in (0..RANK).rev() {
            root[i] = root[i + 1] * root[i + 1];
            iroot[i] = iroot[i + 1] * iroot[i + 1];
        }

        {
            let mut prod = ModInt::new(1);
            let mut iprod = ModInt::new(1);
            for i in 0..RANK - 1 {
                rate[i] = root[i + 2] * prod;
                irate[i] = iroot[i + 2] * iprod;
                prod *= iroot[i + 2];
                iprod *= root[i + 2];
            }
        }

        FftCache { rate, irate }
    }
}

pub fn conv(a: &Vec<ModInt>, b: &Vec<ModInt>, cache: &FftCache) -> Vec<ModInt> {
    let ntt = |a: &mut Vec<ModInt>| {
        let n = a.len();
        let h = n.trailing_zeros();

        for len in 0..h {
            let p = 1 << (h - len - 1);
            let mut rot = ModInt::new(1);
            for (s, offset) in (0..1 << len).map(|s| s << (h - len)).enumerate() {
                let (al, ar) = a[offset..offset + 2 * p].split_at_mut(p);
                for (al, ar) in al.iter_mut().zip(ar.iter_mut()) {
                    let l = *al;
                    let r = *ar * rot;
                    *al = l + r;
                    *ar = l - r;
                }
                rot *= cache.rate[(!s).trailing_zeros() as usize];
            }
        }
    };

    let intt = |a: &mut Vec<ModInt>| {
        let n = a.len();
        let h = n.trailing_zeros();

        for len in (1..=h).rev() {
            let p = 1 << (h - len);
            let mut irot = ModInt::new(1);
            for (s, offset) in (0..1 << (len - 1)).map(|s| s << (h - len + 1)).enumerate() {
                let (al, ar) = a[offset..offset + 2 * p].split_at_mut(p);
                for (al, ar) in al.iter_mut().zip(ar.iter_mut()) {
                    let l = *al;
                    let r = *ar;
                    *al = l + r;
                    *ar = (l - r) * irot;
                }
                irot *= cache.irate[(!s).trailing_zeros() as usize];
            }
        }
    };

    if a.len() <= 2 {
        let mut res = vec![ModInt::new(0); a.len() + b.len() - 1];
        for i in 0..a.len() {
            for j in 0..b.len() {
                res[i + j] += a[i] * b[j];
            }
        }
        return res;
    } else if b.len() <= 2 || a.len() + b.len() <= 60 {
        let mut res = vec![ModInt::new(0); a.len() + b.len() - 1];
        for j in 0..b.len() {
            for i in 0..a.len() {
                res[i + j] += a[i] * b[j];
            }
        }
        return res;
    }

    let s = a.len() + b.len() - 1;
    let t = s.next_power_of_two();

    let (mut a, mut b) = (a.to_owned(), b.to_owned());
    a.resize(t, ModInt::new(0));
    ntt(&mut a);
    b.resize(t, ModInt::new(0));
    ntt(&mut b);
    a.iter_mut().zip(b.iter()).for_each(|(x, y)| *x = *x * *y);
    intt(&mut a);
    a.resize(s, ModInt::new(0));
    let t_inv = ModInt::new(t).inverse();
    a.iter_mut().for_each(|x| *x = *x * t_inv);
    a
}

#[derive(Clone, PartialEq, Eq)]
pub struct FPS {
    coefficient: Vec<ModInt>,
}

impl FPS {
    pub fn new(coefficient: Vec<ModInt>) -> Self {
        FPS { coefficient }
    }

    pub fn get(&self, deg: usize) -> Option<&ModInt> {
        self.coefficient.get(deg)
    }

    pub fn len(&self) -> usize {
        self.coefficient.len()
    }

    pub fn shrinked(&self, deg: usize) -> Self {
        FPS::new(self.coefficient.iter().copied().take(deg + 1).collect())
    }

    pub fn inverse(&self, deg: usize) -> Self {
        fn newton_by(dim: usize, init: ModInt, rec: impl Fn(FPS, usize) -> FPS) -> FPS {
            let mut res = FPS::new(vec![init]);
            while res.len() != dim {
                let d = res.coefficient.len() * 2;
                res = rec(res, d).shrinked(std::cmp::min(d, dim) - 1);
            }

            res
        }
        let mut fps = self.clone();

        if fps.len() < deg + 1 {
            fps.coefficient.resize(deg + 1, ModInt::new(0));
        }

        let rec = |g: FPS, d| g.clone() * (FPS::new(vec![ModInt::new(2)]) - g * fps.shrinked(d));

        newton_by(deg + 1, fps.coefficient[0].inverse(), rec)
    }

    pub fn derivative(&self) -> Self {
        FPS::new(
            self.coefficient
                .iter()
                .enumerate()
                .skip(1)
                .map(|(i, &x)| ModInt::new(i) * x)
                .collect(),
        )
    }

    pub fn integral(&self) -> Self {
        FPS::new(
            vec![ModInt::new(0)]
                .into_iter()
                .chain(
                    self.coefficient
                        .iter()
                        .enumerate()
                        .map(|(i, &x)| x / ModInt::new(i + 1)),
                )
                .collect(),
        )
    }

    pub fn log(&self, deg: usize) -> Self {
        assert_eq!(
            *self.coefficient.get(0).unwrap_or(&ModInt::new(0)),
            ModInt::new(1)
        );
        (self.derivative().shrinked(deg) * self.inverse(deg))
            .integral()
            .shrinked(deg)
    }

    pub fn exp(&self, deg: usize) -> Self {
        assert_eq!(
            *self.coefficient.get(0).unwrap_or(&ModInt::new(0)),
            ModInt::new(0)
        );

        fn newton_by(precision: usize, init: ModInt, rec: impl Fn(FPS, usize) -> FPS) -> FPS {
            let mut res = FPS::new(vec![init]);
            while res.len() != precision {
                let d = res.coefficient.len() * 2;
                res = rec(res, d).shrinked(std::cmp::min(d, precision) - 1);
            }
            res
        }

        let rec = |g: FPS, d| {
            ((self.shrinked(d - 1) + FPS::new(vec![ModInt::new(1)]) - g.log(d - 1)) * g)
                .shrinked(d - 1)
        };

        newton_by(deg + 1, ModInt::new(1), rec)
    }

    // If:
    //     f = x^{k}mg ([x^{0}]g = 1)
    // Then:
    //     f^{n} = x^{kn}m^{n}exp(nlog(g))
    pub fn pow(&self, n: usize, max_deg: usize) -> FPS {
        if n == 0 {
            return FPS::new(vec![ModInt::new(1)]);
        }

        if self.coefficient.iter().all(|k| *k == ModInt::new(0)) {
            return FPS::new(vec![ModInt::new(0)]);
        }

        let mut k = 0;
        while let Some(&x) = self.get(k) {
            if x == ModInt::new(0) {
                k += 1;
            } else {
                break;
            }
        }

        if k as u128 * n as u128 >= max_deg as u128 + 1 {
            return FPS::new(vec![ModInt::new(0)]);
        }

        let m = *self.get(k).unwrap();
        let m_inv = m.inverse();
        let g = FPS::new(
            self.coefficient
                .iter()
                .cloned()
                .skip(k)
                .map(|x| x * m_inv)
                .collect(),
        );

        let mut res = g.log(max_deg + 10 - k * n);
        res *= FPS::new(vec![ModInt::new(n)]);
        res = res.exp(max_deg);
        res *= FPS::new(vec![m.pow(n)]);
        FPS::new(
            iter::repeat(ModInt::new(0))
                .take(k * n)
                .chain(res.coefficient.into_iter().take(max_deg + 1 - k * n))
                .collect(),
        )
    }

    /// Returns:
    ///     f(exp(x))
    pub fn composition_exp(&self, max_deg: usize) -> FPS {
        let mut ab = vec![];
        for i in 0..self.coefficient.len() {
            ab.push((self.coefficient[i], ModInt::new(i)));
        }

        sum_of_exp_bx(ab, max_deg)
    }
}

impl ops::Add for FPS {
    type Output = FPS;
    fn add(self, other: Self) -> Self {
        let len = std::cmp::max(self.len(), other.len());
        let mut res = self.coefficient.clone();
        res.resize(len, ModInt::new(0));
        res.iter_mut()
            .zip(other.coefficient.iter())
            .for_each(|(x, y)| *x += *y);
        FPS::new(res)
    }
}

impl ops::Sub for FPS {
    type Output = FPS;
    fn sub(self, other: Self) -> Self {
        let len = std::cmp::max(self.len(), other.len());
        let mut res = self.coefficient.clone();
        res.resize(len, ModInt::new(0));
        res.iter_mut()
            .zip(other.coefficient.iter())
            .for_each(|(x, y)| *x -= *y);
        FPS::new(res)
    }
}

impl ops::Mul for FPS {
    type Output = FPS;
    fn mul(self, other: Self) -> Self {
        let cache = FftCache::new();
        let coefficient = conv(&self.coefficient, &other.coefficient, &cache);
        FPS::new(coefficient)
    }
}

impl ops::AddAssign for FPS {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl ops::SubAssign for FPS {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl ops::MulAssign for FPS {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl fmt::Display for FPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.coefficient.len() {
            let _ = write!(f, "{}x^{} ", self.coefficient[i], i);
            if i + 1 != self.coefficient.len() {
                let _ = write!(f, "+ ");
            }
        }
        write!(f, "")
    }
}

impl Ord for FPS {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for FPS {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.len()).cmp(&other.len()).reverse())
    }
}

pub fn product_of_polynomial_sequence(v: Vec<FPS>, max_deg: usize) -> FPS {
    let mut que = v.into_iter().collect::<VecDeque<_>>();
    que.push_back(FPS::new(vec![ModInt::new(1)]));

    while que.len() >= 2 {
        let fps1 = que.pop_front().unwrap();
        let fps2 = que.pop_front().unwrap();
        que.push_back(fps1 * fps2.shrinked(max_deg));
    }

    que[0].clone()
}

pub fn division_of_polynomials(mut f: FPS, g: FPS) -> (FPS, FPS) {
    if f.len() < g.len() {
        return (FPS::new(vec![]), f);
    }

    let mut rf = f.clone();
    let mut rg = g.clone();
    rf.coefficient.reverse();
    rg.coefficient.reverse();
    let deg = rf.len() - g.len() + 1;
    rf.coefficient.resize(deg, ModInt::new(0));
    rg.coefficient.resize(deg, ModInt::new(0));
    rg = rg.inverse(deg);
    let mut q = rf * rg;
    q.coefficient.resize(deg, ModInt::new(0));
    q.coefficient.reverse();
    let mut h = q.clone() * g;
    h.coefficient.resize(f.len(), ModInt::new(0));
    f -= h;
    while f.len() > 0 && f.coefficient[f.len() - 1] == ModInt::new(0) {
        f.coefficient.pop();
    }
    (q, f)
}

pub fn multipoint_evaluation(f: FPS, p: Vec<ModInt>) -> Vec<ModInt> {
    let m = p.len();

    let mut g = vec![FPS::new(vec![ModInt::new(1)]); 2 * m.next_power_of_two()];
    for i in 0..m {
        g[i + m.next_power_of_two()] = FPS::new(vec![ModInt::new(UMOD - 1) * p[i], ModInt::new(1)]);
    }
    for i in (1..m.next_power_of_two()).rev() {
        g[i] = g[2 * i].clone() * g[2 * i + 1].clone();
    }

    let mut h = vec![FPS::new(vec![ModInt::new(1)]); 2 * m.next_power_of_two()];
    h[1] = division_of_polynomials(f.clone(), g[1].clone()).1;

    for i in 2..2 * m.next_power_of_two() {
        h[i] = division_of_polynomials(h[i / 2].clone(), g[i].clone()).1;
    }

    h[m.next_power_of_two()..]
        .iter()
        .take(m)
        .map(|x| *x.coefficient.get(0).unwrap_or(&ModInt::new(0)))
        .collect()
}

pub fn polynomial_interpolation(x: Vec<ModInt>, y: Vec<ModInt>) -> FPS {
    assert_eq!(x.len(), y.len());
    let n = x.len();

    let l = product_of_polynomial_sequence(
        x.iter()
            .cloned()
            .map(|x| FPS::new(vec![ModInt::new(UMOD - 1) * x, ModInt::new(1)]))
            .collect(),
        x.len() - 1,
    );
    let dl = FPS::new(
        (1..l.len())
            .map(|i| l.coefficient[i] * ModInt::new(i))
            .collect(),
    );

    let mut g = vec![fps!(1); 2 * n.next_power_of_two()];
    for i in 0..n {
        g[i + n.next_power_of_two()] = fps! {-1 * x[i].value() as isize, 1};
    }
    for i in (1..n.next_power_of_two()).rev() {
        g[i] = g[2 * i].clone() * g[2 * i + 1].clone();
    }

    let w = multipoint_evaluation(dl, x.clone());

    let mut a = y
        .iter()
        .zip(w.iter())
        .map(|(x, y)| *x / *y)
        .collect::<Vec<_>>();
    a.resize(n.next_power_of_two(), ModInt::new(0));

    let mut flr = vec![fps! {0}; 2 * n.next_power_of_two()];
    for i in 0..n {
        flr[n.next_power_of_two() + i] = fps!(a[i].value());
    }
    for i in (1..n.next_power_of_two()).rev() {
        flr[i] =
            flr[2 * i].clone() * g[2 * i + 1].clone() + flr[2 * i + 1].clone() * g[2 * i].clone();
    }
    flr[1].coefficient.resize(n, ModInt::new(0));

    flr[1].clone()
}

/// fracs := \{(a_{i}, b_{i}}_{i}
/// Returns:
///     - fracs := \{(a_{i}, b_{i}}_{i}
///     \sum_{i} \frac{a_{i}}{b_{i}}
fn sum_of_rationals(fracs: Vec<(FPS, FPS)>) -> (FPS, FPS) {
    let mut fracs = fracs.into_iter().collect::<VecDeque<_>>();
    while fracs.len() > 1 {
        let a = fracs.pop_front().unwrap();
        let b = fracs.pop_front().unwrap();
        fracs.push_back((a.0 * b.1.clone() + a.1.clone() * b.0, a.1 * b.1));
    }
    fracs[0].clone()
}

/// Returns:
///     - ab := \{(a_{i}, b_{i}}_{i}
///     \sum_{i} a_{i} exp(b_{i}x)
pub fn sum_of_exp_bx(ab: Vec<(ModInt, ModInt)>, max_deg: usize) -> FPS {
    let mut fracs = vec![];
    for (a, b) in ab {
        fracs.push((fps! {a.value()}, fps! {1, -1 * b.value() as isize}));
    }
    let (mut f, mut g) = sum_of_rationals(fracs);
    g = g.shrinked(max_deg + 1);
    f = f * g.inverse(max_deg + 1);
    f = f.shrinked(max_deg + 1);
    let mut fact_inv = (1..=max_deg)
        .fold(ModInt::new(1), |x, y| {
            let yy = y as isize;
            x * m!(yy)
        })
        .inverse();
    for i in (1..=max_deg).rev() {
        f.coefficient[i] *= fact_inv;
        fact_inv *= m!(i);
    }
    f
}

// FPSマクロ呼んだ時 マクロの$がスニペットカーソルに変換されるので注意 $xと打てば良い
