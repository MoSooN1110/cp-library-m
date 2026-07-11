// source snippet: key=lib_wavelet_matrix4  prefix=lib_wavelet_matrix4
// Your description.

/* ========== ここからライブラリ ========== */
mod bitvec {
	/// **Succinct BitVector**
	/// - `access`, `rank1` を O(1) で提供
	pub struct BitVector {
		bits: Vec<u64>,
		prefix: Vec<usize>, // block ごとの 1 の累積
		len: usize,
	}
	impl BitVector {
		pub fn new(raw: Vec<u8>) -> Self {
			let len = raw.len();
			let blocks = (len + 63) / 64;
			let mut bits = vec![0u64; blocks];
			for (i, &b) in raw.iter().enumerate() {
				if b != 0 {
					bits[i >> 6] |= 1 << (i & 63);
				}
			}
			// 64bit ごとの累積 1 数
			let mut prefix = Vec::with_capacity(blocks + 1);
			prefix.push(0);
			let mut sum = 0usize;
			for &blk in &bits {
				sum += blk.count_ones() as usize;
				prefix.push(sum);
			}
			Self { bits, prefix, len }
		}
		#[inline]
		pub fn access(&self, idx: usize) -> bool {
			((self.bits[idx >> 6] >> (idx & 63)) & 1) != 0
		}
		#[inline]
		pub fn rank1(&self, idx: usize) -> usize {
			// 区間 [0, idx) に含まれる 1 の個数
			let blk = idx >> 6;
			let off = idx & 63;
			let mut cnt = self.prefix[blk];
			if off != 0 {
				cnt += (self.bits[blk] & ((1u64 << off) - 1)).count_ones() as usize;
			}
			cnt
		}
	}
}

mod wavelet_matrix {
	//! Wavelet Matrix 実装
	//!
	//! - 値域：`u64`（必要なら `u32` でも OK）
	//! - 高さ = ⌈log₂(max+1)⌉
	//! - 主要操作をすべて **O(log σ)** で提供

	use super::bitvec::BitVector;

	pub struct WaveletMatrix {
		levels: Vec<BitVector>, // MSB → LSB
		zeros: Vec<usize>,      // 各レベルで 0 側に移動した要素数
		height: usize,
		size: usize,
	}

	impl WaveletMatrix {
		/// O(N log σ) で構築
		pub fn new(arr: &[u64]) -> Self {
			let size = arr.len();
			let max = *arr.iter().max().unwrap_or(&0);
			let height = if max == 0 {
				1
			} else {
				64 - max.leading_zeros() as usize
			};

			let mut levels = Vec::with_capacity(height);
			let mut zeros = Vec::with_capacity(height);

			// 現在の並びを更新しつつ上位ビットから構築
			let mut cur: Vec<u64> = arr.to_vec();
			for bit in (0..height).rev() {
				// MSB → LSB
				let mut raw = vec![0u8; size];
				let mut zs = Vec::with_capacity(size);
				let mut os = Vec::with_capacity(size);

				for (i, &v) in cur.iter().enumerate() {
					if (v >> bit) & 1 == 1 {
						raw[i] = 1;
						os.push(v);
					} else {
						zs.push(v);
					}
				}
				zeros.push(zs.len());
				zs.extend_from_slice(&os);
				cur = zs;
				levels.push(BitVector::new(raw));
			}
			// levels と zeros は MSB → LSB の順序なので reverse は不要

			Self {
				levels,
				zeros,
				height,
				size,
			}
		}

		#[inline]
		fn rank1(&self, lvl: usize, idx: usize) -> usize {
			self.levels[lvl].rank1(idx)
		}

		// ----------------------------------------------------------
		// ★ クエリ実装
		// ----------------------------------------------------------

		/// access(i): 元配列 A[i] を取得
		pub fn access(&self, mut idx: usize) -> u64 {
			let mut val = 0u64;
			for lvl in 0..self.height {
				val <<= 1;
				let bit = self.levels[lvl].access(idx);
				if bit {
					val |= 1;
					idx = self.zeros[lvl] + self.rank1(lvl, idx);
				} else {
					idx = idx - self.rank1(lvl, idx);
				}
			}
			val
		}

		/// freq(l, r, x): 区間 [l, r) における値 x の出現数
		pub fn freq(&self, mut l: usize, mut r: usize, x: u64) -> usize {
			if l >= r {
				return 0;
			}
			for lvl in 0..self.height {
				let bit = ((x >> (self.height - lvl - 1)) & 1) as usize;
				let rl = self.rank1(lvl, l);
				let rr = self.rank1(lvl, r);
				if bit == 0 {
					// 0 側へ
					l = l - rl;
					r = r - rr;
				} else {
					// 1 側へ
					l = self.zeros[lvl] + rl;
					r = self.zeros[lvl] + rr;
				}
			}
			r - l
		}

		/// kth_smallest(l, r, k): [l,r) で k 番目 (0-based) に小さい値
		pub fn kth_smallest(&self, mut l: usize, mut r: usize, mut k: usize) -> Option<u64> {
			if l >= r || k >= r - l {
				return None;
			}
			let mut val = 0u64;
			for lvl in 0..self.height {
				let zeros_l = l - self.rank1(lvl, l);
				let zeros_r = r - self.rank1(lvl, r);
				let zc = zeros_r - zeros_l; // 0 側の個数
				val <<= 1;
				if k < zc {
					// 0 側
					l = zeros_l;
					r = zeros_r;
				} else {
					// 1 側
					val |= 1;
					k -= zc;
					l = self.zeros[lvl] + self.rank1(lvl, l);
					r = self.zeros[lvl] + self.rank1(lvl, r);
				}
			}
			Some(val)
		}

		/// kth_largest(l, r, k): [l,r) で k 番目 (0-based) に大きい値
		pub fn kth_largest(&self, l: usize, r: usize, k: usize) -> Option<u64> {
			if l >= r {
				return None;
			}
			let len = r - l;
			if k >= len {
				return None;
			}
			// 大きい順 k → 小さい順 (len-k-1)
			self.kth_smallest(l, r, len - k - 1)
		}

		/// less_than(l, r, x): [l,r) で x 未満の要素数
		pub fn less_than(&self, mut l: usize, mut r: usize, x: u64) -> usize {
			let mut cnt = 0usize;
			for lvl in 0..self.height {
				let bit = ((x >> (self.height - lvl - 1)) & 1) as usize;
				let rl = self.rank1(lvl, l);
				let rr = self.rank1(lvl, r);
				let zeros_l = l - rl;
				let zeros_r = r - rr;
				if bit == 1 {
					// 0 側はすべて x 未満
					cnt += zeros_r - zeros_l;
					l = self.zeros[lvl] + rl;
					r = self.zeros[lvl] + rr;
				} else {
					l = zeros_l;
					r = zeros_r;
				}
			}
			cnt
		}

		/// range_freq(l, r, low, high): [l,r) において値が [low, high) に含まれる要素数
		pub fn range_freq(&self, l: usize, r: usize, low: u64, high: u64) -> usize {
			if low >= high || l >= r {
				return 0;
			}
			self.less_than(l, r, high) - self.less_than(l, r, low)
		}

		/// quantile(l, r, k): kth_smallest の別名（統計でいう分位点）
		pub fn quantile(&self, l: usize, r: usize, k: usize) -> Option<u64> {
			self.kth_smallest(l, r, k)
		}
	}
}
