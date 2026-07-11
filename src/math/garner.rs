//! Garner のアルゴリズム（互いに素な法の連立合同を、任意の法で復元）。
//! `x ≡ r[i] (mod m[i])` を満たす最小非負 x を `target_mod` で割った値を返す。
//!
//! ```
//! use cplib::math::garner::*;
//! // x≡2(mod3), x≡3(mod5) → x=8。1000 で割った値も 8。
//! assert_eq!(garner(&[2, 3], &[3, 5], 1_000_000_007), 8);
//! ```
use crate::math::number::mod_inv;

/// `m[i]` は pairwise coprime を仮定。返り値は `x mod target_mod`。
pub fn garner(r: &[u64], m: &[u64], target_mod: u64) -> u64 {
    assert_eq!(r.len(), m.len());
    // 末尾に target_mod を追加して同じ手続きで x mod target を得る
    let mut mm: Vec<u64> = m.to_vec();
    mm.push(target_mod);
    let n = r.len();
    // coeffs[k] = prod_{j<k} m[j] mod mm[k], constants[k] = 現在の x mod mm[k]
    let mut coeffs = vec![1u64; n + 1];
    let mut constants = vec![0u64; n + 1];
    for i in 0..n {
        let mi = m[i];
        // t = (r[i] - constants[i]) / coeffs[i]  (mod mi)
        let inv = mod_inv(coeffs[i] as i64, mi as i64).expect("moduli must be coprime") as u64;
        let t = ((r[i] + mi - constants[i] % mi) % mi) * inv % mi;
        for j in (i + 1)..=n {
            constants[j] = (constants[j] + t % mm[j] * (coeffs[j] % mm[j])) % mm[j];
            coeffs[j] = coeffs[j] % mm[j] * (mi % mm[j]) % mm[j];
        }
    }
    constants[n] % target_mod
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(garner(&[2, 3], &[3, 5], 1_000_000_007), 8);
        assert_eq!(garner(&[1, 2, 3], &[2, 3, 5], 1_000_000_007), 23);
        // target で割った値になっているか
        assert_eq!(garner(&[1, 2, 3], &[2, 3, 5], 7), 23 % 7);
    }
    #[test]
    fn brute_check() {
        // 小さい法でナイーブ照合
        let ms = [3u64, 4, 5]; // pairwise coprime
        let prod: u64 = ms.iter().product();
        for x in 0..prod {
            let r: Vec<u64> = ms.iter().map(|&m| x % m).collect();
            let got = garner(&r, &ms, 1_000_000);
            assert_eq!(got, x % 1_000_000);
        }
    }
}
