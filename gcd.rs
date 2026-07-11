// source snippet: key=lib_gcd  prefix=lib_gcd

#[allow(dead_code)]
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    if (a < b) {
        let t = a;
        a = b;
        b = t;
    }
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
