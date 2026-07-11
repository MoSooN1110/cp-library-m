// source snippet: key=pow_mat  prefix=pow_mat
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

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
