//! NTT 畳み込み（mod 998244353）。既存の `Mint` をそのまま使う ACL 風 butterfly。
//!
//! MOD は NTT-friendly（998244353 = 119·2^23 + 1、原始根 3）である必要がある。
//! 長さの合計は 2^23 まで。`ntt_inplace` / `intt_inplace` も公開（FPS 用）。
//!
//! ```
//! use cplib::math::convolution::*;
//! use cplib::math::modint::*;
//! let a: Vec<Mint> = [1i64, 2, 3].iter().map(|&x| Mint::new(x)).collect();
//! let b: Vec<Mint> = [4i64, 5, 6].iter().map(|&x| Mint::new(x)).collect();
//! let c: Vec<u64> = convolution(&a, &b).iter().map(|x| x.val()).collect();
//! assert_eq!(c, vec![4, 13, 28, 27, 18]);
//! assert_eq!(convolution_i64(&[1, 2], &[1, 1]), vec![1, 3, 2]);
//! ```

use crate::math::modint::{Mint, MOD};

const PRIMITIVE_ROOT: u64 = 3; // 998244353 の原始根

struct NttTables {
    rate2: Vec<Mint>,
    irate2: Vec<Mint>,
    rate3: Vec<Mint>,
    irate3: Vec<Mint>,
    imag: Mint,
    iimag: Mint,
    rank2: usize,
}

fn ntt_tables() -> &'static NttTables {
    use std::sync::OnceLock;
    static ONCE: OnceLock<NttTables> = OnceLock::new();
    ONCE.get_or_init(|| {
        assert_eq!(MOD, 998_244_353, "NTT requires the NTT-friendly modulus");
        let rank2 = (MOD - 1).trailing_zeros() as usize; // 23
        let root = Mint::raw(PRIMITIVE_ROOT).pow((MOD - 1) >> rank2);
        let iroot = root.inv();
        // 1 の原始 4 乗根
        let imag = root.pow(1u64 << (rank2 - 2));
        let iimag = imag.inv();

        let mut rate2 = vec![Mint::raw(0); rank2 - 1];
        let mut irate2 = vec![Mint::raw(0); rank2 - 1];
        {
            let mut prod = Mint::raw(1);
            let mut iprod = Mint::raw(1);
            for i in 0..rank2 - 1 {
                let e = 1u64 << (rank2 - i - 2);
                rate2[i] = root.pow(e) * prod;
                irate2[i] = iroot.pow(e) * iprod;
                prod *= iroot.pow(e);
                iprod *= root.pow(e);
            }
        }
        let mut rate3 = vec![Mint::raw(0); rank2 - 2];
        let mut irate3 = vec![Mint::raw(0); rank2 - 2];
        {
            let mut prod = Mint::raw(1);
            let mut iprod = Mint::raw(1);
            for i in 0..rank2 - 2 {
                let e = 1u64 << (rank2 - i - 3);
                rate3[i] = root.pow(e) * prod;
                irate3[i] = iroot.pow(e) * iprod;
                prod *= iroot.pow(e);
                iprod *= root.pow(e);
            }
        }
        NttTables {
            rate2,
            irate2,
            rate3,
            irate3,
            imag,
            iimag,
            rank2,
        }
    })
}

fn butterfly(a: &mut [Mint]) {
    let n = a.len();
    if n <= 1 {
        return;
    }
    debug_assert!(n.is_power_of_two());
    let h = n.trailing_zeros() as usize;
    let tab = ntt_tables();
    assert!(h <= tab.rank2, "length exceeds NTT capability");

    let mut len = 0usize;
    while len < h {
        if h - len == 1 {
            // radix-2
            let p = 1usize << (h - len - 1);
            let mut rot = Mint::raw(1);
            for s in 0..1usize << len {
                let offset = s << (h - len);
                for i in 0..p {
                    let l = a[offset + i];
                    let r = a[offset + i + p] * rot;
                    a[offset + i] = l + r;
                    a[offset + i + p] = l - r;
                }
                if s + 1 != 1usize << len {
                    rot *= tab.rate2[(!s).trailing_zeros() as usize];
                }
            }
            len += 1;
        } else {
            // radix-4
            let p = 1usize << (h - len - 2);
            let mut rot = Mint::raw(1);
            for s in 0..1usize << len {
                let rot2 = rot * rot;
                let rot3 = rot2 * rot;
                let offset = s << (h - len);
                for i in 0..p {
                    let a0 = a[offset + i];
                    let a1 = a[offset + i + p] * rot;
                    let a2 = a[offset + i + 2 * p] * rot2;
                    let a3 = a[offset + i + 3 * p] * rot3;
                    let a1na3 = a1 - a3;
                    let a1pa3 = a1 + a3;
                    let a0pa2 = a0 + a2;
                    let a0ma2 = a0 - a2;
                    a[offset + i] = a0pa2 + a1pa3;
                    a[offset + i + p] = a0pa2 - a1pa3;
                    a[offset + i + 2 * p] = a0ma2 + a1na3 * tab.imag;
                    a[offset + i + 3 * p] = a0ma2 - a1na3 * tab.imag;
                }
                if s + 1 != 1usize << len {
                    rot *= tab.rate3[(!s).trailing_zeros() as usize];
                }
            }
            len += 2;
        }
    }
}

fn butterfly_inv(a: &mut [Mint]) {
    let n = a.len();
    if n <= 1 {
        return;
    }
    debug_assert!(n.is_power_of_two());
    let h = n.trailing_zeros() as usize;
    let tab = ntt_tables();
    assert!(h <= tab.rank2, "length exceeds NTT capability");

    let mut len = h;
    while len > 0 {
        if len == 1 {
            let p = 1usize << (h - len);
            let mut irot = Mint::raw(1);
            for s in 0..1usize << (len - 1) {
                let offset = s << (h - len + 1);
                for i in 0..p {
                    let l = a[offset + i];
                    let r = a[offset + i + p];
                    a[offset + i] = l + r;
                    a[offset + i + p] = (l - r) * irot;
                }
                if s + 1 != 1usize << (len - 1) {
                    irot *= tab.irate2[(!s).trailing_zeros() as usize];
                }
            }
            len -= 1;
        } else {
            let p = 1usize << (h - len);
            let mut irot = Mint::raw(1);
            for s in 0..1usize << (len - 2) {
                let irot2 = irot * irot;
                let irot3 = irot2 * irot;
                let offset = s << (h - len + 2);
                for i in 0..p {
                    let y0 = a[offset + i];
                    let y1 = a[offset + i + p];
                    let y2 = a[offset + i + 2 * p];
                    let y3 = a[offset + i + 3 * p];
                    let t0 = y0 + y1;
                    let t1 = y0 - y1;
                    let t2 = y2 + y3;
                    let t3 = (y2 - y3) * tab.iimag;
                    a[offset + i] = t0 + t2;
                    a[offset + i + p] = (t1 + t3) * irot;
                    a[offset + i + 2 * p] = (t0 - t2) * irot2;
                    a[offset + i + 3 * p] = (t1 - t3) * irot3;
                }
                if s + 1 != 1usize << (len - 2) {
                    irot *= tab.irate3[(!s).trailing_zeros() as usize];
                }
            }
            len -= 2;
        }
    }
}

/// 数論変換（in-place）。長さは 2 冪。
pub fn ntt_inplace(a: &mut [Mint]) {
    butterfly(a);
}

/// 逆数論変換（in-place、1/n 倍まで行う）。長さは 2 冪。
pub fn intt_inplace(a: &mut [Mint]) {
    butterfly_inv(a);
    let inv_n = Mint::from(a.len()).inv();
    for x in a.iter_mut() {
        *x *= inv_n;
    }
}

/// 畳み込み c[k] = Σ_{i+j=k} a[i]·b[j]。長さ |a|+|b|-1。O(n log n)。
pub fn convolution(a: &[Mint], b: &[Mint]) -> Vec<Mint> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let need = a.len() + b.len() - 1;
    if a.len().min(b.len()) <= 40 {
        let mut res = vec![Mint::raw(0); need];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                res[i + j] += x * y;
            }
        }
        return res;
    }
    let n = need.next_power_of_two();
    let mut fa = vec![Mint::raw(0); n];
    let mut fb = vec![Mint::raw(0); n];
    fa[..a.len()].copy_from_slice(a);
    fb[..b.len()].copy_from_slice(b);
    butterfly(&mut fa);
    butterfly(&mut fb);
    for i in 0..n {
        fa[i] *= fb[i];
    }
    butterfly_inv(&mut fa);
    fa.truncate(need);
    let inv_n = Mint::from(n).inv();
    for x in fa.iter_mut() {
        *x *= inv_n;
    }
    fa
}

/// i64 入出力の畳み込み（内部で mod 998244353）。結果は [0, MOD)。
pub fn convolution_i64(a: &[i64], b: &[i64]) -> Vec<i64> {
    let a: Vec<Mint> = a.iter().map(|&x| Mint::new(x)).collect();
    let b: Vec<Mint> = b.iter().map(|&x| Mint::new(x)).collect();
    convolution(&a, &b).iter().map(|x| x.val() as i64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(a: &[Mint], b: &[Mint]) -> Vec<Mint> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut res = vec![Mint::raw(0); a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                res[i + j] += x * y;
            }
        }
        res
    }

    #[test]
    fn known_and_edge() {
        let a: Vec<Mint> = [1i64, 2, 3].iter().map(|&x| Mint::new(x)).collect();
        let b: Vec<Mint> = [4i64, 5, 6].iter().map(|&x| Mint::new(x)).collect();
        let c: Vec<u64> = convolution(&a, &b).iter().map(|x| x.val()).collect();
        assert_eq!(c, vec![4, 13, 28, 27, 18]);
        assert!(convolution(&[], &a).is_empty());
        assert!(convolution(&a, &[]).is_empty());
        assert_eq!(
            convolution(&[Mint::new(7)], &[Mint::new(6)]),
            vec![Mint::new(42)]
        );
    }

    #[test]
    fn ntt_roundtrip() {
        let mut x: u64 = 424242;
        let mut rng = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for lg in 0..8 {
            let n = 1usize << lg;
            let orig: Vec<Mint> = (0..n).map(|_| Mint::raw(rng() % MOD)).collect();
            let mut a = orig.clone();
            ntt_inplace(&mut a);
            intt_inplace(&mut a);
            assert_eq!(a, orig, "roundtrip n={n}");
        }
    }

    #[test]
    fn random_vs_naive() {
        let mut x: u64 = 998244353;
        let mut rng = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..30 {
            // 小（ナイーブ経路）から大（NTT 経路、非 2 冪長含む）まで
            let n = (rng() % 150) as usize;
            let m = (rng() % 150) as usize;
            let a: Vec<Mint> = (0..n).map(|_| Mint::raw(rng() % MOD)).collect();
            let b: Vec<Mint> = (0..m).map(|_| Mint::raw(rng() % MOD)).collect();
            assert_eq!(convolution(&a, &b), naive(&a, &b), "n={n} m={m}");
        }
        // NTT 経路を確実に通す大きめサイズ
        let a: Vec<Mint> = (0..300).map(|_| Mint::raw(rng() % MOD)).collect();
        let b: Vec<Mint> = (0..123).map(|_| Mint::raw(rng() % MOD)).collect();
        assert_eq!(convolution(&a, &b), naive(&a, &b));
    }

    #[test]
    fn i64_wrapper() {
        assert_eq!(convolution_i64(&[1, 2], &[1, 1]), vec![1, 3, 2]);
        // 負数は mod に正規化される
        let c = convolution_i64(&[-1], &[1]);
        assert_eq!(c, vec![(MOD - 1) as i64]);
    }
}
