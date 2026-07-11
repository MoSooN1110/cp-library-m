// source snippet: key=lib_combination  prefix=lib_combination

pub struct Combination {
    m: usize,
    f_table: Vec<usize>,
}

impl Combination {
    // 0 <= size <= 10^8 is constrained.
    pub fn new(mod_num: usize, table_size: usize) -> Self {
        Self {
            m: mod_num,
            f_table: vec![0; table_size],
        }
    }
    pub fn build(&mut self) {
        let size = self.f_table.len();
        self.f_table = fact_table(size, self.m);
    }
    fn fact_table(len: usize, m: usize) -> Vec<usize> {
        let mut res = vec![1; len + 1];
        for i in 1..len + 1 {
            res[i] = (i as usize * res[i - 1]) % m;
        }
        res
    }

    pub fn p(&mut self, n: usize, k: usize) -> usize {
        let p = MOD as usize;
        if k == 0 {
            return 1;
        }
        if n < k {
            0
        } else {
            let (a1, e1) = mod_fact(n, p, &self.f_table);
            let (a2, e2) = mod_fact(k, p, &self.f_table);
            let (a3, e3) = mod_fact(n - k, p, &self.f_table);
            if e1 > e2 + e3 {
                0
            } else {
                a1 * mod_inverse(a3 % p, p) % p
            }
        }
    }
    pub fn c(&mut self, n: usize, k: usize) -> usize {
        let p = MOD as usize;
        if k == 0 {
            return 1;
        }
        if n < k {
            0
        } else {
            let (a1, e1) = mod_fact(n, p, &self.f_table);
            let (a2, e2) = mod_fact(k, p, &self.f_table);
            let (a3, e3) = mod_fact(n - k, p, &self.f_table);
            if e1 > e2 + e3 {
                0
            } else {
                a1 * mod_inverse(a2 * a3 % p, p) % p
            }
        }
    }
    pub fn h(&mut self, n: usize, k: usize) -> usize {
        return mcom(n + k - 1, k, &self.f_table);
    }

    pub fn factorial(&mut self, n: usize) -> usize {
        return self.p(n, n);
    }

    fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
        if b == 0 {
            (a, 1, 0)
        } else {
            let (gcd, x, y) = extgcd(b, a % b);
            (gcd, y, x - (a / b) * y)
        }
    }
    pub fn mod_inverse(a: usize, m: usize) -> usize {
        let (_, x, _) = extgcd(a as i64, m as i64);
        ((m as i64 + x) as usize % m) % m
    }
    fn mod_fact(&mut self, n: usize, p: usize, fact: &[usize]) -> (usize, usize) {
        if n == 0 {
            (1, 0)
        } else {
            let (a, b) = mod_fact(n / p, p, fact);
            let pow = b + n / p;
            if n / p % 2 != 0 {
                (a * (p - fact[(n % p) as usize]) % p, pow)
            } else {
                (a * fact[(n % p) as usize] % p, pow)
            }
        }
    }
}
