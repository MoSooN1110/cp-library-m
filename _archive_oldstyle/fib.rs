// source snippet: key=fib  prefix=fib
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
/// Matrix mul
fn mat_mul2(x: &[u64; 4], y: &[u64; 4], p: u64) -> [u64; 4] {
    [
        (x[0] * y[0] + x[1] * y[2]) % p,
        (x[0] * y[1] + x[1] * y[3]) % p,
        (x[2] * y[0] + x[3] * y[2]) % p,
        (x[2] * y[1] + x[3] * y[3]) % p,
    ]
}
#[allow(dead_code)]
/// Matrix pow
fn pow_mat(m: &[u64; 4], n: u64, p: u64) -> [u64; 4] {
    let mut m = *m;
    let mut res = [1, 0, 1, 0];
    for i in 0.. {
        if n >> i == 0 {
            break;
        }
        if n >> i & 1 == 1 {
            res = mat_mul2(&res, &m, p);
        }
        m = mat_mul2(&m, &m, p);
    }
    res
}
#[allow(dead_code)]
/// Fast fibonacci calculation
fn fib(i: u64, p: u64) -> u64 {
    let m = pow_mat(&[1, 1, 1, 0], i, p);
    m[1]
}
