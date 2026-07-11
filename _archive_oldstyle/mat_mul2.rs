// source snippet: key=mat_mul2  prefix=mat_mul2
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
