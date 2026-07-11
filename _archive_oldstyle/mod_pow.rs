// source snippet: key=mod_pow  prefix=mod_pow
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

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
