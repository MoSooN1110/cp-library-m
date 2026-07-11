// source snippet: key=lib_bmbm  prefix=lib_bmbm
// Your description.

const PRIMITIVE_ROOT: usize = 3;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct ModInt(usize);

impl ModInt {
	fn new(n: usize) -> Self {
		ModInt(n % UMOD)
	}
	fn zero() -> Self {
		ModInt(0)
	}
	fn one() -> Self {
		ModInt(1)
	}
	fn pow(self, mut e: u64) -> Self {
		let mut base = self;
		let mut res = ModInt::one();
		while e > 0 {
			if e & 1 == 1 {
				res *= base;
			}
			base *= base;
			e >>= 1;
		}
		res
	}
	fn inv(self) -> Self {
		// Fermat's little theorem: a^(UMOD-2)
		self.pow((UMOD as u64) - 2)
	}
}

impl From<u64> for ModInt {
	fn from(x: u64) -> Self {
		ModInt((x % (UMOD as u64)) as usize)
	}
}

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

impl Add for ModInt {
	type Output = ModInt;
	fn add(self, rhs: ModInt) -> ModInt {
		let sum = self.0 + rhs.0;
		ModInt(if sum >= UMOD { sum - UMOD } else { sum })
	}
}

impl Sub for ModInt {
	type Output = ModInt;
	fn sub(self, rhs: ModInt) -> ModInt {
		ModInt(if self.0 < rhs.0 {
			self.0 + UMOD - rhs.0
		} else {
			self.0 - rhs.0
		})
	}
}

impl Mul for ModInt {
	type Output = ModInt;
	fn mul(self, rhs: ModInt) -> ModInt {
		ModInt((self.0 * rhs.0) % UMOD)
	}
}

impl AddAssign for ModInt {
	fn add_assign(&mut self, rhs: ModInt) {
		*self = *self + rhs;
	}
}
impl SubAssign for ModInt {
	fn sub_assign(&mut self, rhs: ModInt) {
		*self = *self - rhs;
	}
}
impl MulAssign for ModInt {
	fn mul_assign(&mut self, rhs: ModInt) {
		*self = *self * rhs;
	}
}

// Number Theoretic Transform (NTT)
fn bit_reverse(a: &mut [ModInt]) {
	let n = a.len();
	let mut j = 0;
	for i in 1..n {
		let mut bit = n >> 1;
		while j & bit != 0 {
			j ^= bit;
			bit >>= 1;
		}
		j |= bit;
		if i < j {
			a.swap(i, j);
		}
	}
}

fn ntt(a: &mut [ModInt], invert: bool) {
	let n = a.len();
	bit_reverse(a);
	let mut len = 2;
	while len <= n {
		let wlen = ModInt::from(PRIMITIVE_ROOT as u64).pow((UMOD as u64 - 1) / len as u64);
		let wlen = if invert { wlen.inv() } else { wlen };
		for i in (0..n).step_by(len) {
			let mut w = ModInt::one();
			for j in 0..len / 2 {
				let u = a[i + j];
				let v = a[i + j + len / 2] * w;
				a[i + j] = u + v;
				a[i + j + len / 2] = u - v;
				w *= wlen;
			}
		}
		len <<= 1;
	}
	if invert {
		let inv_n = ModInt::from(n as u64).inv();
		for x in a.iter_mut() {
			*x *= inv_n;
		}
	}
}

fn convolution(a: &[ModInt], b: &[ModInt]) -> Vec<ModInt> {
	let mut n = 1;
	while n < a.len() + b.len() - 1 {
		n <<= 1;
	}
	let mut fa = a.to_vec();
	fa.resize(n, ModInt::zero());
	let mut fb = b.to_vec();
	fb.resize(n, ModInt::zero());
	ntt(&mut fa, false);
	ntt(&mut fb, false);
	for i in 0..n {
		fa[i] *= fb[i];
	}
	ntt(&mut fa, true);
	fa.resize(a.len() + b.len() - 1, ModInt::zero());
	fa
}

// Berlekamp–Massey algorithm
fn berlekamp_massey(s: &[ModInt]) -> Vec<ModInt> {
	let n = s.len();
	let mut c = vec![ModInt::one()];
	let mut b = vec![ModInt::one()];
	let mut l = 0;
	let mut m = 1;
	let mut bb = ModInt::one();
	for i in 0..n {
		let mut d = ModInt::zero();
		for j in 0..=l {
			d += c[j] * s[i - j];
		}
		if d == ModInt::zero() {
			m += 1;
		} else if 2 * l <= i {
			let t = c.clone();
			let coef = d * bb.inv();
			c.resize((b.len() + m).max(c.len()), ModInt::zero());
			for j in 0..b.len() {
				c[j + m] -= coef * b[j];
			}
			l = i + 1 - l;
			b = t;
			bb = d;
			m = 1;
		} else {
			let coef = d * bb.inv();
			c.resize((b.len() + m).max(c.len()), ModInt::zero());
			for j in 0..b.len() {
				c[j + m] -= coef * b[j];
			}
			m += 1;
		}
	}
	c.remove(0);
	for x in c.iter_mut() {
		*x = ModInt::zero() - *x;
	}
	c
}

// Bostan–Mori to compute [z^N] num(z)/den(z)
fn bostan_mori(mut num: Vec<ModInt>, mut den: Vec<ModInt>, mut n: i64) -> ModInt {
	while n > 0 {
		let mut den_neg = den.clone();
		for (i, x) in den_neg.iter_mut().enumerate() {
			if i % 2 == 1 {
				*x = ModInt::zero() - *x;
			}
		}
		let f2 = convolution(&num, &den_neg);
		let g2 = convolution(&den, &den_neg);
		let num2: Vec<ModInt> = if n % 2 == 0 {
			f2.iter().step_by(2).cloned().collect()
		} else {
			f2.iter().skip(1).step_by(2).cloned().collect()
		};
		let den2: Vec<ModInt> = g2.iter().step_by(2).cloned().collect();
		num = num2;
		den = den2;
		n /= 2;
	}
	num[0] * den[0].inv()
}

// Compute k-th term (0-based) of linearly recurrent sequence with initial a and recurrence c
fn linear_recurrence(a: &[ModInt], c: &[ModInt], k: i64) -> ModInt {
	let n = c.len();
	if n == 0 {
		return ModInt::zero();
	}
	// D(z) = 1 - sum_{i=0..n) c[i] z^{i+1}
	let mut dnm = vec![ModInt::one(); 1 + n];
	for i in 0..n {
		dnm[i + 1] = ModInt::zero() - c[i];
	}
	let mut a_vec = a.to_vec();
	a_vec.resize(n, ModInt::zero());
	let num = convolution(&dnm, &a_vec)[..n].to_vec();
	bostan_mori(num, dnm, k)
}

fn useage() {
	let (k, l, r) = readuuu();
	let k = k as i64;
	let l = l as i64;

	// Precompute sequence up to N
	let N = 500;
	let mut a = vec![ModInt::zero(); N + 1];
	a[0] = ModInt::one();
	for n in 0..N {
		let term = ModInt::from(k as u64) * a[n]
			+ ModInt::from(n as u64).pow(k as u64)
			+ ModInt::from(k as u64).pow(n as u64);
		a[n + 1] = term;
	}
	// Prefix sums b
	let mut b = vec![ModInt::zero(); N + 2];
	for i in 0..=N {
		b[i + 1] = b[i] + a[i];
	}
	// Find minimal linear recurrence for b
	let c = berlekamp_massey(&b);
	let res_r = linear_recurrence(&b, &c, (r + 1) as i64);
	let res_l = linear_recurrence(&b, &c, l as i64);
	let ans = res_r - res_l;
	println!("{}", ans.0);
}
