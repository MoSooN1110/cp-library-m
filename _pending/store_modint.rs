// source snippet: key=store_modint  prefix=store_modint

// fp {{{
#[allow(dead_code)]
mod fp {
    use std::{
        fmt::{Debug, Display},
        hash::Hash,
        iter::{once, repeat, successors, FromIterator, Product, Sum},
        marker::PhantomData,
        mem::{swap, take},
        ops::{
            Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
        },
    };
    // TODO: マクロで定義するとバンドラさんに壊されしまいます。
    pub enum M1000000007 {}
    impl Mod for M1000000007 {
        const P: u64 = 1_000_000_007;
    }
    pub type F1000000007 = Fp<M1000000007>;
    pub enum M998244353 {}
    impl Mod for M998244353 {
        const P: u64 = 998_244_353;
    }
    impl Fft for M998244353 {
        const ROOT: u64 = 3;
    }
    pub type F998244353 = Fp<M998244353>;
    pub type Fps998244353 = Fpsp<M998244353>;
    pub enum M1012924417 {}
    impl Mod for M1012924417 {
        const P: u64 = 1_012_924_417;
    }
    impl Fft for M1012924417 {
        const ROOT: u64 = 5;
    }
    pub type F1012924417 = Fp<M1012924417>;
    pub type Fps1012924417 = Fpsp<M1012924417>;
    pub enum M924844033 {}
    impl Mod for M924844033 {
        const P: u64 = 924_844_033;
    }
    impl Fft for M924844033 {
        const ROOT: u64 = 5;
    }
    pub type F924844033 = Fp<M924844033>;
    pub type Fps924844033 = Fpsp<M924844033>;
    pub trait Mod {
        const P: u64; // ……ほう。
    }
    pub trait Fft: Mod {
        const ROOT: u64;
    }
    #[macro_export]
    macro_rules! define_fp {
            ($p:expr) => {
                $crate::fp::define_fp! { $p; enum M; type F }
            };
            ($p:expr, $root:expr) => {
                $crate::fp::define_fp! { $p, $root; enum M; type F; type Fps }
            };
            (
                $p:expr;
                $vism:vis enum $m:ident;
                $visf:vis type $f:ident$(;)?
            ) => {
                #[allow(dead_code)]
                $vism enum $m {}
                impl $crate::fp::Mod for $m {
                    const P: u64 = $p;
                }
                #[allow(dead_code)]
                $visf type $f = $crate::fp::Fp<$m>;
            };
            (
                $p:expr, $root:expr;
                $vism:vis enum $m:ident;
                $visf:vis type $f:ident;
                $visfps:vis type $fps:ident$(;)?
            ) => {
                $crate::fp::define_fp! { $p; $vism enum $m; $visf type $f }
                impl $crate::fp::Fft for $m {
                    const ROOT: u64 = $root;
                }
                #[allow(dead_code)]
                $visfps type $fps = $crate::fp::Fpsp<$m>;
            };
        }
    #[macro_export]
    macro_rules! fp {
        ($num:expr; $den:expr) => {
            $crate::fp::Fp::from($num) / $crate::fp::Fp::from($den)
        };
        ($value:expr) => {
            $crate::fp::Fp::from($value)
        };
    }
    #[macro_export]
    macro_rules! fps {
            () => (
                $crate::fp::Fpsp(Vec::new())
            );
            ($elem:expr; $n:expr) => (
                $crate::fp::Fpsp(vec![$crate::fp::fp!($elem); $n])
            );
            ($($x:expr),+ $(,)?) => (
                $crate::fp::Fpsp(vec![$($crate::fp::fp!($x)),+])
            );
        }
    pub struct Fp<M>(u64, PhantomData<fn() -> M>);
    impl<M: Fft> Fp<M> {
        pub const ROOT: Self = Self(M::ROOT, PhantomData);
    }
    impl<M: Mod> Fp<M> {
        pub const P: u64 = M::P;
        pub fn new(value: u64) -> Self {
            Self(value % Self::P, PhantomData)
        }
        pub fn value(self) -> u64 {
            self.0
        }
        pub fn inv(self) -> Self {
            if self.0 == 0 {
                panic!("Cannot invert `0`.");
            }
            let mut x = Self::P as i64;
            let mut y = self.0 as i64;
            let mut u = 0;
            let mut v = 1;
            while y != 0 {
                let q = x / y;
                x -= y * q;
                u -= v * q;
                swap(&mut x, &mut y);
                swap(&mut u, &mut v);
            }
            debug_assert_eq!(x, 1);
            debug_assert_eq!(v.abs(), Self::P as i64);
            debug_assert!(u.abs() < Self::P as i64);
            if u < 0 {
                u += Self::P as i64;
            }
            Self(u as u64, PhantomData)
        }
        pub fn pow(mut self, mut exp: u64) -> Self {
            let mut res = Self(1, PhantomData);
            if exp != 0 {
                while exp != 1 {
                    if exp % 2 == 1 {
                        res *= self;
                    }
                    exp /= 2;
                    self *= self;
                }
                res *= self;
            }
            res
        }
    }
    impl<M: Mod> Copy for Fp<M> {}
    impl<M: Mod> Clone for Fp<M> {
        fn clone(&self) -> Self {
            Self(self.0, self.1)
        }
    }
    impl<M: Mod> PartialEq for Fp<M> {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }
    impl<M: Mod> Display for Fp<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            Display::fmt(&self.0, f)
        }
    }
    impl<M: Mod> Eq for Fp<M> {}
    impl<M: Mod> Default for Fp<M> {
        fn default() -> Self {
            Self(0, PhantomData)
        }
    }
    impl<M: Mod> Hash for Fp<M> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state)
        }
    }
    impl<M: Mod> Debug for Fp<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            pub fn berlekamp_massey_fp(a: i64, p: i64) -> [i64; 2] {
                let mut u0 = 0_i64;
                let mut v0 = 1_i64;
                let mut w0 = a * u0 + p * v0;
                let mut u1 = 1_i64;
                let mut v1 = 0_i64;
                let mut w1 = a * u1 + p * v1;
                while p <= w0 * w0 {
                    let q = w0 / w1;
                    u0 -= q * u1;
                    v0 -= q * v1;
                    w0 -= q * w1;
                    swap(&mut u0, &mut u1);
                    swap(&mut v0, &mut v1);
                    swap(&mut w0, &mut w1);
                }
                [w0, u0]
            }
            if self.0 == 0 {
                return write!(f, "0");
            }
            let [mut num, mut den] = berlekamp_massey_fp(self.0 as i64, M::P as i64);
            if den < 0 {
                num = -num;
                den = -den;
            }
            if den == 1 {
                write!(f, "{}", num)
            } else {
                write!(f, "{}/{}", num, den)
            }
        }
    }
    macro_rules! impl_from_large_int {
            ($($T:ty), *$(,)?) => {$(
                impl<M: Mod> From<$T> for Fp<M> {
                    fn from(x: $T) -> Self {
                        Self::new(x.rem_euclid(M::P as _) as u64)
                    }
                }
            )*}
        }
    impl_from_large_int! {
        i8, i16, i32, i64,
        u128, usize,
        i128, isize,
    }
    macro_rules! impl_from_small_int {
            ($($T: ty), *$(,)?) => {$(
                impl<M: Mod> From<$T> for Fp<M> {
                    fn from(x: $T) -> Self {
                        Self::new(x as u64)
                    }
                }
            )*}
        }
    impl_from_small_int! {
        u8, u16, u32, u64,
    }
    impl<M: Mod, T: Into<Fp<M>>> AddAssign<T> for Fp<M> {
        fn add_assign(&mut self, rhs: T) {
            self.0 += rhs.into().0;
            if M::P <= self.0 {
                self.0 -= M::P;
            }
        }
    }
    impl<M: Mod, T: Into<Fp<M>>> SubAssign<T> for Fp<M> {
        fn sub_assign(&mut self, rhs: T) {
            let rhs = rhs.into().0;
            if self.0 < rhs {
                self.0 += M::P;
            }
            self.0 -= rhs;
        }
    }
    impl<M: Mod, T: Into<Fp<M>>> MulAssign<T> for Fp<M> {
        fn mul_assign(&mut self, rhs: T) {
            self.0 *= rhs.into().0;
            self.0 %= Self::P;
        }
    }
    #[allow(clippy::suspicious_op_assign_impl)]
    impl<M: Mod, T: Into<Fp<M>>> DivAssign<T> for Fp<M> {
        fn div_assign(&mut self, rhs: T) {
            *self *= rhs.into().inv();
        }
    }
    impl<M: Mod> Neg for Fp<M> {
        type Output = Fp<M>;
        fn neg(self) -> Self::Output {
            if self.0 == 0 {
                self
            } else {
                Self(Self::P - self.0, PhantomData)
            }
        }
    }
    impl<M: Mod> Neg for &Fp<M> {
        type Output = Fp<M>;
        fn neg(self) -> Self::Output {
            -*self
        }
    }
    macro_rules! fp_forward_ops {
            ($(
                $trait:ident,
                $trait_assign:ident,
                $fn:ident,
                $fn_assign:ident,
            )*) => {$(
                impl<M: Mod> $trait_assign<&Fp<M>> for Fp<M> {
                    fn $fn_assign(&mut self, rhs: &Fp<M>) {
                        self.$fn_assign(*rhs);
                    }
                }
                impl<M: Mod, T: Into<Fp<M>>> $trait<T> for Fp<M> {
                    type Output = Fp<M>;
                    fn $fn(mut self, rhs: T) -> Self::Output {
                        self.$fn_assign(rhs.into());
                        self
                    }
                }
                impl<M: Mod> $trait<&Fp<M>> for Fp<M> {
                    type Output = Fp<M>;
                    fn $fn(self, rhs: &Fp<M>) -> Self::Output {
                        self.$fn(*rhs)
                    }
                }
                impl<M: Mod, T: Into<Fp<M>>> $trait<T> for &Fp<M> {
                    type Output = Fp<M>;
                    fn $fn(self, rhs: T) -> Self::Output {
                        (*self).$fn(rhs.into())
                    }
                }
                impl<M: Mod> $trait<&Fp<M>> for &Fp<M> {
                    type Output = Fp<M>;
                    fn $fn(self, rhs: &Fp<M>) -> Self::Output {
                        (*self).$fn(*rhs)
                    }
                }
            )*};
        }
    fp_forward_ops! {
        Add, AddAssign, add, add_assign,
        Sub, SubAssign, sub, sub_assign,
        Mul, MulAssign, mul, mul_assign,
        Div, DivAssign, div, div_assign,
    }
    impl<M: Mod> Sum for Fp<M> {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::new(0), |b, x| b + x)
        }
    }
    impl<M: Mod> Product for Fp<M> {
        fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::new(1), |b, x| b * x)
        }
    }
    impl<'a, M: Mod> Sum<&'a Self> for Fp<M> {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(Self::new(0), |b, x| b + x)
        }
    }
    impl<'a, M: Mod> Product<&'a Self> for Fp<M> {
        fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(Self::new(1), |b, x| b * x)
        }
    }
    pub fn fact_iter<M: Mod>() -> impl Iterator<Item = Fp<M>> {
        (1..).scan(Fp::new(1), |state, x| {
            let ans = *state;
            *state *= x;
            Some(ans)
        })
    }
    #[allow(clippy::missing_panics_doc)]
    pub fn fact_build<M: Mod>(n: usize) -> FactTable<M> {
        FactTable(if n == 0 {
            [Vec::new(), Vec::new()]
        } else {
            let fact = fact_iter::<M>().take(n).collect::<Vec<_>>();
            let mut fact_inv = vec![fact.last().unwrap().inv(); n];
            (1..n).rev().for_each(|i| fact_inv[i - 1] = fact_inv[i] * i);
            [fact, fact_inv]
        })
    }
    #[derive(Clone, Debug, Default, Hash, PartialEq)]
    pub struct FactTable<M: Mod>(pub [Vec<Fp<M>>; 2]);
    impl<M: Mod> FactTable<M> {
        pub fn binom(&self, n: usize, k: usize) -> Fp<M> {
            assert!(n < self.0[0].len());
            assert!(k <= n);
            self.0[0][n] * self.0[1][k] * self.0[1][n - k]
        }
        pub fn binom2(&self, i: usize, j: usize) -> Fp<M> {
            self.binom(i + j, i)
        }
        pub fn binom_inv(&self, n: u64, k: u64) -> Fp<M> {
            let n = n as usize;
            let k = k as usize;
            assert!(n < self.0[0].len());
            assert!(k <= n);
            self.0[1][n] * self.0[0][k] * self.0[0][n - k]
        }
        pub fn binom_or_zero(&self, n: usize, k: isize) -> Fp<M> {
            assert!(n < self.0[0].len() as usize);
            if (0..=n as isize).contains(&k) {
                self.binom(n, k as usize)
            } else {
                Fp::new(0)
            }
        }
    }
    pub fn binom_iter<M: Mod>() -> impl Iterator<Item = Vec<Fp<M>>> {
        successors(Some(vec![Fp::new(1)]), |last| {
            let mut crr = last.clone();
            crr.push(Fp::new(0));
            crr[1..].iter_mut().zip(last).for_each(|(x, &y)| *x += y);
            Some(crr)
        })
    }
    pub fn convolution<M: Fft>(mut a: Vec<Fp<M>>, mut b: Vec<Fp<M>>) -> Vec<Fp<M>> {
        if a.is_empty() || b.is_empty() {
            Vec::new()
        } else {
            let n = a.len() + b.len() - 1;
            a.resize(n.next_power_of_two(), Fp::new(0));
            b.resize(n.next_power_of_two(), Fp::new(0));
            fft(&mut a);
            fft(&mut b);
            let mut c = a.into_iter().zip(b).map(|(x, y)| x * y).collect::<Vec<_>>();
            ifft(&mut c);
            c.truncate(n);
            c
        }
    }
    pub fn anymod_convolution<M: Mod>(a: &[Fp<M>], b: &[Fp<M>]) -> Vec<Fp<M>> {
        type Fp1 = F998244353;
        type Fp2 = F1012924417;
        type Fp3 = F924844033;
        let v1 = convolution(
            a.iter().map(|&x| Fp1::new(x.value())).collect::<Vec<_>>(),
            b.iter().map(|&x| Fp1::new(x.value())).collect::<Vec<_>>(),
        );
        let v2 = convolution(
            a.iter().map(|&x| Fp2::new(x.value())).collect::<Vec<_>>(),
            b.iter().map(|&x| Fp2::new(x.value())).collect::<Vec<_>>(),
        );
        let v3 = convolution(
            a.iter().map(|&x| Fp3::new(x.value())).collect::<Vec<_>>(),
            b.iter().map(|&x| Fp3::new(x.value())).collect::<Vec<_>>(),
        );
        v1.into_iter()
            .zip(v2)
            .zip(v3)
            .map(|((e1, e2), e3)| {
                let x1 = e1;
                let x2 = (e2 - Fp2::new(x1.value())) * Fp2::new(Fp1::P).inv();
                let x3 = ((e3 - Fp3::new(x1.value())) * Fp3::new(Fp1::P).inv()
                    - Fp3::new(x2.value()))
                    * Fp3::new(Fp2::P).inv();
                Fp::new(x1.value())
                    + Fp::new(x2.value()) * Fp::new(Fp1::P)
                    + Fp::new(x3.value()) * Fp::new(Fp1::P) * Fp::new(Fp2::P)
            })
            .collect::<Vec<_>>()
    }
    pub fn ifft<M: Fft>(a: &mut [Fp<M>]) {
        let n = a.len();
        assert!(n.is_power_of_two());
        let root = Fp::ROOT.pow((M::P - 1) / a.len() as u64);
        let mut roots = successors(Some(root.inv()), |x| Some(x * x))
            .take(n.trailing_zeros() as usize + 1)
            .collect::<Vec<_>>();
        roots.reverse();
        let fourth = Fp::ROOT.pow((M::P - 1) / 4).inv();
        let mut quarter = 1_usize;
        if n.trailing_zeros() % 2 == 1 {
            for a in a.chunks_mut(2) {
                let x = a[0];
                let y = a[1];
                a[0] = x + y;
                a[1] = x - y;
            }
            quarter = 2;
        }
        while quarter != n {
            let fft_len = quarter * 4;
            let root = roots[fft_len.trailing_zeros() as usize];
            for a in a.chunks_mut(fft_len) {
                let mut c = Fp::new(1);
                for (((i, j), k), l) in (0..)
                    .zip(quarter..)
                    .zip(quarter * 2..)
                    .zip(quarter * 3..)
                    .take(quarter)
                {
                    let c2 = c * c;
                    let x = a[i] + c2 * a[j];
                    let y = a[i] - c2 * a[j];
                    let z = c * (a[k] + c2 * a[l]);
                    let w = fourth * c * (a[k] - c2 * a[l]);
                    a[i] = x + z;
                    a[j] = y + w;
                    a[k] = x - z;
                    a[l] = y - w;
                    c *= root;
                }
            }
            quarter = fft_len;
        }
        let d = Fp::from(a.len()).inv();
        a.iter_mut().for_each(|x| *x *= d);
    }
    pub fn fft<M: Fft>(a: &mut [Fp<M>]) {
        let n = a.len();
        assert!(n.is_power_of_two());
        let mut root = Fp::ROOT.pow((M::P - 1) / a.len() as u64);
        let fourth = Fp::ROOT.pow((M::P - 1) / 4);
        let mut fft_len = n;
        while 4 <= fft_len {
            let quarter = fft_len / 4;
            for a in a.chunks_mut(fft_len) {
                let mut c = Fp::new(1);
                for (((i, j), k), l) in (0..)
                    .zip(quarter..)
                    .zip(quarter * 2..)
                    .zip(quarter * 3..)
                    .take(quarter)
                {
                    let c2 = c * c;
                    let x = a[i] + a[k];
                    let y = a[j] + a[l];
                    let z = a[i] - a[k];
                    let w = fourth * (a[j] - a[l]);
                    a[i] = x + y;
                    a[j] = c2 * (x - y);
                    a[k] = c * (z + w);
                    a[l] = c2 * c * (z - w);
                    c *= root;
                }
            }
            root *= root;
            root *= root;
            fft_len = quarter;
        }
        if fft_len == 2 {
            for a in a.chunks_mut(2) {
                let x = a[0];
                let y = a[1];
                a[0] = x + y;
                a[1] = x - y;
            }
        }
    }
    pub struct Fpsp<M>(pub Vec<Fp<M>>);
    impl<M: Fft> Fpsp<M> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        pub fn len(&self) -> usize {
            self.0.len()
        }
        pub fn truncated(&self, len: usize) -> Self {
            self.iter().copied().take(len).collect()
        }
        pub fn resized(&self, len: usize) -> Self {
            self.iter()
                .copied()
                .chain(repeat(fp!(0)))
                .take(len)
                .collect()
        }
        pub fn derivative(&self) -> Self {
            self.iter()
                .enumerate()
                .skip(1)
                .map(|(i, &x)| fp!(i) * x)
                .collect()
        }
        pub fn integral(&self) -> Self {
            once(fp!(0))
                .chain(self.iter().enumerate().map(|(i, &x)| x / fp!(i + 1)))
                .collect()
        }
        pub fn inv(&self, precision: usize) -> Self {
            assert!(
                !self.is_empty() && self[0] != fp!(0),
                "Cannot invert an FPS `0`"
            );
            newton_by(precision, self[0].inv(), |g, d| {
                (-&g * self.truncated(d) + 2) * g
            })
        }
        pub fn log(&self, precision: usize) -> Self {
            assert!(
                !self.is_empty() && self[0] == fp!(1),
                "Cannot take a log of an FPS with constant term other than `1`"
            );
            (self.derivative().truncated(precision) * self.inv(precision))
                .integral()
                .resized(precision)
        }
        pub fn exp(&self, precision: usize) -> Self {
            assert!(
                !self.is_empty() && self[0] == fp!(0),
                "Cannot take an exp of an FPS with constant term other than `0`"
            );
            newton_by(precision, fp!(1), |g, d| {
                (self.truncated(d) + 1 - g.log(d)) * g
            })
        }
    }
    pub fn newton_by<M: Fft>(
        precision: usize,
        init: Fp<M>,
        rec: impl Fn(Fpsp<M>, usize) -> Fpsp<M>,
    ) -> Fpsp<M> {
        let mut ans = Fpsp(vec![init]);
        while ans.len() != precision {
            let d = ans.len() * 2;
            ans = rec(ans, d).resized(d.min(precision))
        }
        ans
    }
    // HACK: Deref パターンってラッパーで使っていいんでしたっけ。
    impl<M: Fft> Deref for Fpsp<M> {
        type Target = Vec<Fp<M>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: Fft> DerefMut for Fpsp<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M: Fft> Clone for Fpsp<M> {
        fn clone(&self) -> Self {
            Self(self.to_vec())
        }
    }
    impl<M: Fft, T: Into<Fp<M>>> FromIterator<T> for Fpsp<M> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            Self(iter.into_iter().map(Into::into).collect())
        }
    }
    impl<M: Fft> AddAssign<&Fpsp<M>> for Fpsp<M> {
        fn add_assign(&mut self, rhs: &Fpsp<M>) {
            self.0.iter_mut().zip(&rhs.0).for_each(|(x, &y)| *x += y);
            if self.len() < rhs.len() {
                self.0.extend(rhs.0[self.len()..].iter().copied());
            }
        }
    }
    impl<M: Fft> AddAssign<&Fp<M>> for Fpsp<M> {
        fn add_assign(&mut self, rhs: &Fp<M>) {
            if self.is_empty() {
                self.0.push(*rhs);
            } else {
                self[0] += *rhs;
            }
        }
    }
    impl<M: Fft> SubAssign<&Fpsp<M>> for Fpsp<M> {
        fn sub_assign(&mut self, rhs: &Fpsp<M>) {
            self.0.iter_mut().zip(&rhs.0).for_each(|(x, &y)| *x -= y);
            if self.len() < rhs.len() {
                self.0.extend(rhs.0[self.len()..].iter().map(|&x| -x));
            }
        }
    }
    impl<M: Fft> SubAssign<&Fp<M>> for Fpsp<M> {
        fn sub_assign(&mut self, rhs: &Fp<M>) {
            if self.is_empty() {
                self.0.push(-*rhs);
            } else {
                self[0] -= *rhs;
            }
        }
    }
    impl<M: Fft> Neg for Fpsp<M> {
        type Output = Fpsp<M>;
        fn neg(mut self) -> Self::Output {
            self.0.iter_mut().for_each(|x| *x = -*x);
            self
        }
    }
    impl<M: Fft> Neg for &Fpsp<M> {
        type Output = Fpsp<M>;
        fn neg(self) -> Self::Output {
            self.0.iter().map(|&x| -x).collect()
        }
    }
    macro_rules! fps_forward_ops_borrow {
            ($(
                $trait:ident,
                $trait_assign: ident,
                $fn:ident,
                $fn_assign:ident,
            )*) => {$(
                impl<M: Fft> $trait_assign for Fpsp<M> {
                    fn $fn_assign(&mut self, rhs: Self) {
                        self.$fn_assign(&rhs)
                    }
                }
                impl<M: Fft, T: Into<Fp<M>>> $trait_assign<T> for Fpsp<M> {
                    fn $fn_assign(&mut self, rhs: T) {
                        self.$fn_assign(&rhs.into())
                    }
                }
                impl<M: Fft> $trait for Fpsp<M> {
                    type Output = Fpsp<M>;
                    fn $fn(mut self, rhs: Fpsp<M>) -> Self::Output {
                        self.$fn_assign(rhs);
                        self
                    }
                }
                impl<M: Fft> $trait<&Fpsp<M>> for Fpsp<M> {
                    type Output = Fpsp<M>;
                    fn $fn(mut self, rhs: &Fpsp<M>) -> Self::Output {
                        self.$fn_assign(rhs);
                        self
                    }
                }
                impl<M: Fft> $trait<&Fp<M>> for Fpsp<M> {
                    type Output = Fpsp<M>;
                    fn $fn(mut self, rhs: &Fp<M>) -> Self::Output {
                        self.$fn_assign(rhs);
                        self
                    }
                }
                impl<M: Fft, T: Into<Fp<M>>> $trait<T> for Fpsp<M> {
                    type Output = Fpsp<M>;
                    fn $fn(mut self, rhs: T) -> Self::Output {
                        self.$fn_assign(rhs);
                        self
                    }
                }
            )*};
        }
    fps_forward_ops_borrow! {
        Add, AddAssign, add, add_assign,
        Sub, SubAssign, sub, sub_assign,
    }
    impl<M: Fft> Mul<Fpsp<M>> for Fpsp<M> {
        type Output = Fpsp<M>;
        fn mul(self, rhs: Fpsp<M>) -> Self::Output {
            Fpsp(convolution(self.0, rhs.0))
        }
    }
    impl<M: Fft> MulAssign<&Fp<M>> for Fpsp<M> {
        fn mul_assign(&mut self, rhs: &Fp<M>) {
            self.0.iter_mut().for_each(|x| *x *= *rhs);
        }
    }
    impl<M: Fft> MulAssign<Fpsp<M>> for Fpsp<M> {
        fn mul_assign(&mut self, rhs: Fpsp<M>) {
            *self = take(self).mul(rhs)
        }
    }
    impl<M: Fft, T: Into<Fp<M>>> MulAssign<T> for Fpsp<M> {
        fn mul_assign(&mut self, rhs: T) {
            self.mul_assign(&rhs.into());
        }
    }
    impl<M: Fft> Mul<&Fp<M>> for Fpsp<M> {
        type Output = Fpsp<M>;
        fn mul(mut self, rhs: &Fp<M>) -> Self::Output {
            self.mul_assign(rhs);
            self
        }
    }
    impl<M: Fft, T: Into<Fp<M>>> Mul<T> for Fpsp<M> {
        type Output = Fpsp<M>;
        fn mul(mut self, rhs: T) -> Self::Output {
            self.mul_assign(rhs);
            self
        }
    }
    impl<M: Fft> Debug for Fpsp<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }
    impl<M: Fft> PartialEq for Fpsp<M> {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }
    impl<M: Fft> Eq for Fpsp<M> {}
    impl<M: Fft> Default for Fpsp<M> {
        fn default() -> Self {
            Self(Vec::new())
        }
    }
    impl<M: Fft> Hash for Fpsp<M> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state)
        }
    }
}
// }}}
// template {{{
// mod dining_table {
//     #![allow(unused_imports)]
//     pub use ascii::{AsciiChar, AsciiStr, AsciiString};
//     pub use itertools::Itertools;
//     pub use itertools_num::ItertoolsNum;
//     pub use proconio::marker::Usize1;
//     pub use proconio::{fastout, input};
//     pub use superslice::Ext;

//     #[allow(unused_imports)]
//     #[cfg(feature = "dbg")]
//     pub use dbg::lg;
//     #[cfg(not(feature = "dbg"))]
//     #[allow(unused_macros)]
//     #[macro_export]
//     macro_rules! lg {
//         ($($expr:expr),*) => {};
//     }
// }
