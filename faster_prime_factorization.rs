// source snippet: key=lib_faster_prime_factorization  prefix=lib_faster_prime_factorization

fn prime_factorization(x: usize) -> BTreeMap<usize, usize> {
    let mut res: BTreeMap<usize, usize> = BTreeMap::new();
    let mut xx = x;
    let mut p: usize = 2;
    while p * p <= xx {
        while xx % p == 0 {
            // println!("{:?}", p);
            let t = res.get_mut(&p);
            if t.is_none() {
                res.insert(p, 1);
            } else {
                *t.unwrap() += 1;
            }
            xx /= p;
        }
        // println!("{:?} {:?}", p, res);
        p += 1;
    }

    if xx != 1 {
        let t = res.get_mut(&xx);
        if t.is_none() {
            res.insert(xx, 1);
        } else {
            *t.unwrap() += 1;
        }
    }
    res
}

pub struct Montgomery {
    m: usize,
    pow_r: usize,
    mp: usize,
    mask: usize,
    r2: usize,
}

impl Montgomery {
    pub fn new(m: usize, pow_r: usize) -> Self {
        fn extended_gcd(a: i128, b: i128) -> (i128, i128) {
            if (a, b) == (1, 0) {
                (1, 0)
            } else {
                let (x, y) = extended_gcd(b, a % b);
                (y, x - (a / b) * y)
            }
        }
        let mp = {
            let (_, b) = extended_gcd(1i128 << pow_r, m as i128);
            if b <= 0 {
                (-b) as usize
            } else {
                (-b + (1 << pow_r)) as usize
            }
        };
        let mask = std::usize::MAX;
        let r2 =
            (((1u128 << pow_r) % m as u128) * ((1u128 << pow_r) % m as u128) % m as u128) as usize;
        Montgomery {
            m,
            pow_r,
            mp,
            mask,
            r2,
        }
    }

    /// - Returns:
    ///     - t * R^{-1} mod N
    fn mr(&self, t: u128) -> usize {
        let temp = {
            let mask = self.mask as u128;
            let mp = self.mp as u128;
            let m = self.m as u128;
            let pow_r = self.pow_r as u128;
            ((t + ((t & mask) * mp & mask) * m) >> pow_r) as usize
        };

        if temp >= self.m {
            temp - self.m
        } else {
            temp
        }
    }

    /// - Returns:
    ///     - a + b mod N
    pub fn add(&self, a: usize, b: usize) -> usize {
        (a + b) % self.m
    }

    /// - Returns:
    ///     - a * b mod N
    pub fn mul(&self, a: usize, b: usize) -> usize {
        self.mr(self.mr(a as u128 * b as u128) as u128 * self.r2 as u128)
    }
}

/// - Returns:
///     - GCD(a, b)
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

/// - Returns:
///     - if n is prime number:  
///         * true  
///     - else:  
///         * false  
///
/// - Note:
///     - Algorithm:
///         - Miller-Rabin
pub fn is_prime_large(n: usize) -> bool {
    if n == 0 || n == 1 || (n > 2 && n % 2 == 0) {
        return false;
    }

    if n == 2 {
        return true;
    }

    /// - Returns:
    ///     - $a^{n}$ modulo $m$
    pub fn mod_pow(a: usize, mut n: usize, mont: &Montgomery) -> usize {
        let mut res = 1;
        let mut x = a;
        while n > 0 {
            if n % 2 == 1 {
                res = mont.mul(res, x);
            }
            x = mont.mul(x, x);
            n /= 2;
        }

        res
    }

    let s = (n - 1).trailing_zeros();
    let d = (n - 1) / (1 << s);
    let mont = Montgomery::new(n, 64);

    let f = |mut a| {
        a %= n;
        if a == 0 {
            return true;
        }
        let mut ad = mod_pow(a, d, &mont);
        if ad == 1 || ad == n - 1 {
            return true;
        }

        for _ in 0..s {
            ad = mont.mul(ad, ad);
            if ad == n - 1 {
                return true;
            }
        }

        false
    };

    const A: [usize; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    A.iter().all(|x| f(*x))
}

pub fn factorize_sub(n: usize, res: &mut Vec<usize>) {
    if n == 1 {
        return;
    }

    if is_prime_large(n) {
        res.push(n);
        return;
    }

    let n2 = (n as f64).powf(1.0 / 8.0) as usize;

    // find divisor of n
    let d = if n % 2 == 0 {
        2
    } else {
        (|| {
            let mont = Montgomery::new(n, 64);
            for c in 1234567891.. {
                let f = |a, mont: &Montgomery| mont.add(mont.mul(a, a), c);

                let mut a = vec![2, f(2, &mont)];
                let mut i1 = 0;
                let mut i2 = 1;
                loop {
                    let mut q = 1;
                    for _ in 0..n2 {
                        a.push(f(a[i2], &mont));
                        a.push(f(a[i2 + 1], &mont));
                        i1 += 1;
                        i2 += 2;
                        q = mont.mul(q, std::cmp::max(a[i1], a[i2]) - std::cmp::min(a[i1], a[i2]));
                    }
                    let g = gcd(q, n);
                    if 1 < g && g < n {
                        return g;
                    }
                    if g == n {
                        break;
                    }
                    a.push(f(a[i2], &mont));
                    a.push(f(a[i2 + 1], &mont));
                    i1 += 1;
                    i2 += 2;
                }

                let mut a = vec![2, f(2, &mont)];
                let mut i1 = 0;
                let mut i2 = 1;
                loop {
                    let g = gcd(std::cmp::max(a[i1], a[i2]) - std::cmp::min(a[i1], a[i2]), n);
                    if 1 < g && g < n {
                        return g;
                    }
                    if g == n {
                        break;
                    }
                    a.push(f(a[i2], &mont));
                    a.push(f(a[i2 + 1], &mont));
                    i1 += 1;
                    i2 += 2;
                }
            }
            unreachable!()
        })()
    };

    factorize_sub(d, res);
    factorize_sub(n / d, res);
}

/// - Returns:
///     - result of integer factorization of n
/// - Note:
///     - Algorithm:
///         - Pollard's rho algorithm
pub fn factorize(n: usize) -> Vec<usize> {
    assert!(n != 0);
    let mut res = vec![];

    factorize_sub(n, &mut res);

    res.sort();
    res
}
