// source snippet: key=lib_mod_inv  prefix=lib_mod_inv

pub fn mod_pow(x: usize, n: usize, m: usize) -> usize {
    let mut res = 1;
    let mut x = x % m;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    res
}

fn mod_inv(x: usize, m: usize) -> usize {
    return mod_pow(x, m - 2, m);
}
