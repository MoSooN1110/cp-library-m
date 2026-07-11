//! 整数論の基本: gcd/lcm、拡張ユークリッド、一般 mod 逆元、中国剰余定理(CRT)。
//!
//! ```
//! use cplib::math::number::*;
//! assert_eq!(gcd(12, 18), 6);
//! assert_eq!(mod_inv(3, 7), Some(5));   // 3*5=15≡1 (mod 7)
//! // x≡2(mod3), x≡3(mod5) → x≡8(mod15)
//! assert_eq!(crt(&[2, 3], &[3, 5]), Some((8, 15)));
//! ```

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

/// 拡張ユークリッド: `(g, x, y)` で `a*x + b*y = g = gcd(|a|,|b|)`。
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a.abs(), if a < 0 { -1 } else { 1 }, 0)
    } else {
        let (g, x, y) = ext_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

/// 一般法 m での逆元（gcd(a,m)=1 のとき Some）。
pub fn mod_inv(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = ext_gcd(a.rem_euclid(m), m);
    if g != 1 {
        None
    } else {
        Some(x.rem_euclid(m))
    }
}

/// 中国剰余定理: `x ≡ r[i] (mod m[i])` を満たす `(rem, lcm)` を返す。解なしは None。
/// m[i] は互いに素でなくてよい。
pub fn crt(r: &[i64], m: &[i64]) -> Option<(i64, i64)> {
    assert_eq!(r.len(), m.len());
    let (mut r0, mut m0) = (0i64, 1i64);
    for i in 0..r.len() {
        let (mut ri, mut mi) = (r[i].rem_euclid(m[i]), m[i]);
        if m0 < mi {
            std::mem::swap(&mut r0, &mut ri);
            std::mem::swap(&mut m0, &mut mi);
        }
        if m0 % mi == 0 {
            if r0 % mi != ri {
                return None;
            }
            continue;
        }
        let (g, im, _) = ext_gcd(m0, mi);
        let u1 = mi / g;
        if (ri - r0) % g != 0 {
            return None;
        }
        let x = ((ri - r0) / g).rem_euclid(u1) * im.rem_euclid(u1) % u1;
        r0 += x * m0;
        m0 *= u1;
        r0 = r0.rem_euclid(m0);
    }
    Some((r0, m0))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gcd_lcm() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(gcd(0, 5), 5);
    }
    #[test]
    fn extgcd_prop() {
        for a in -20i64..=20 {
            for b in -20i64..=20 {
                let (g, x, y) = ext_gcd(a, b);
                assert_eq!(a * x + b * y, g);
                if a != 0 || b != 0 {
                    assert!(g > 0);
                }
            }
        }
    }
    #[test]
    fn inv() {
        for m in 2..50i64 {
            for a in 1..m {
                match mod_inv(a, m) {
                    Some(x) => assert_eq!((a * x).rem_euclid(m), 1),
                    None => assert_ne!(gcd(a as u64, m as u64), 1),
                }
            }
        }
    }
    #[test]
    fn crt_check() {
        assert_eq!(crt(&[2, 3], &[3, 5]), Some((8, 15)));
        assert_eq!(crt(&[1, 2, 3], &[2, 3, 5]), Some((23, 30)));
        // 矛盾: x≡0(mod2), x≡1(mod4)
        assert_eq!(crt(&[0, 1], &[2, 4]), None);
        // 総当たり検証
        for a in 0..12i64 {
            for b in 0..12i64 {
                let (m1, m2) = (6, 8);
                let res = crt(&[a, b], &[m1, m2]);
                let mut expect = None;
                for x in 0..(m1 * m2 / gcd(m1 as u64, m2 as u64) as i64) {
                    if x % m1 == a % m1 && x % m2 == b % m2 {
                        expect = Some(x);
                        break;
                    }
                }
                match (res, expect) {
                    (Some((r, _)), Some(e)) => assert_eq!(r, e),
                    (None, None) => {}
                    _ => panic!("mismatch a={a} b={b} res={res:?} expect={expect:?}"),
                }
            }
        }
    }
}
