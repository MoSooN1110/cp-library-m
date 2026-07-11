// source snippet: key=lib_min_linear_mod  prefix=lib_min_linear_mod
// Your description.

fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
	while b != 0 {
		let t = a % b;
		a = b;
		b = t;
	}
	a.abs()
}

#[inline]
fn floor_div_nonneg(a: i128, b: i128) -> i128 {
	// a,b >= 0 前提
	a / b
}

#[inline]
fn ceil_div_nonneg(a: i128, b: i128) -> i128 {
	// a,b >= 0 前提
	if a == 0 {
		0
	} else {
		(a - 1) / b + 1
	}
}

// C++ min_of_linear_segments 相当：
// a,b,mod は 0<=a,b<mod, mod>0, さらに g=gcd(a,mod) で割った後の値（= “本体”）を渡す。
// 返り値： (X, DX)
//  - 等差数列の境界 X = [x0=0, x1, x2, ...] （昇順）
//  - 各区間の公差 DX = [q0, q1, ...]（X[i]..X[i+1] の区間の公差）
//   このとき、prefix-min を更新する x は
//   区間ごとに { X[i] + q_i, X[i] + 2*q_i, ..., X[i+1] } の和集合（最後はちょうど X[i+1] に一致）
fn min_of_linear_segments_core(mut a: i128, mut b: i128, mut modu: i128) -> (Vec<i128>, Vec<i128>) {
	debug_assert!(0 <= a && a < modu);
	debug_assert!(0 <= b && b < modu);
	// ここに渡す前に gcd で割ってある前提（= 本体だけ）
	let mut x_list: Vec<i128> = vec![0];
	let mut dx_list: Vec<i128> = vec![];

	// Stern–Brocot 風の 2 つの分数区間を保つ
	// p/q <= (mod-a)/mod <= r/s
	let mut p: i128 = 0;
	let mut q: i128 = 1;
	let mut r: i128 = 1;
	let mut s: i128 = 1;

	// det_l = mod-a, det_r = a
	let mut det_l = modu - a;
	let mut det_r = a;

	// 現在の最小値（“本体”側） y と、その最小が達成される最後の x（縮約世界）
	let mut x: i128 = 0;
	let mut y: i128 = b;

	// y が 0 になるまで（0 になれば以降は更新なし）
	while y > 0 {
		// ---- upd r/s （右側の分数を進める）----
		let mut k = floor_div_nonneg(det_r, det_l);
		det_r %= det_l;
		if det_r == 0 {
			k -= 1;
			det_r = det_l;
		}
		r += k * p;
		s += k * q;

		// ---- 内側ループ：左側を det_r ずつ削りつつ、更新セグメントを吐き出す ----
		loop {
			// k = max(0, ceil((det_l - y)/det_r))
			let need = if det_l >= y { det_l - y } else { 0 };
			let k = ceil_div_nonneg(need, det_r).max(0);
			if det_l - k * det_r <= 0 {
				break;
			}

			det_l -= k * det_r;
			p += k * r;
			q += k * s;

			// この時点で p/q <= a/mod
			// (a*q - p*mod) = det_l を y から引けるだけ引く
			let t = floor_div_nonneg(y, det_l); // ここで t ≥ 1 になる
			y -= t * det_l;
			x += q * t;

			x_list.push(x);
			dx_list.push(q);
		}

		// ---- det_l を det_r で 1 回割る（右へ 1 ステップ）----
		k = floor_div_nonneg(det_l, det_r);
		det_l -= k * det_r;
		p += k * r;
		q += k * s;

		// 安全側のチェック（負になることはない）
		debug_assert!(p >= 0 && q >= 0 && r >= 0 && s >= 0);
	}
	(x_list, dx_list)
}

// ーーーーー ここから公開 API ーーーーー

// min of (a*x + b) % m on x in [0..=n]
pub fn min_linear_mod(mut n: i128, mut m: i128, mut a: i128, mut b: i128) -> i128 {
	debug_assert!(n >= 0 && m > 0);

	// 正規化（0..m-1）
	a = a.rem_euclid(m);
	b = b.rem_euclid(m);

	// gcd で分解して“本体”へ
	let g = gcd_i128(a, m);
	let m1 = m / g;
	let a1 = a / g;
	// 重要：b は floor(b/g) と b%g に分ける
	let b_floor = b.div_euclid(g);
	let b_rem = b - b_floor * g; // = b % g （0..g-1）

	// 早期終了：b_floor==0 なら“本体”最小は 0、よって答えは b_rem
	if b_floor == 0 {
		return b_rem;
	}

	// “本体”側のセグメント列をつくる
	let (xs, dxs) = min_of_linear_segments_core(a1, b_floor, m1);

	// [0..=n] に含まれる「prefix-min を更新する最後の x*」を探す。
	// セグメント i（0-based）は [xs[i], xs[i+1]] に対して
	// S_i = { xs[i] + dxs[i], xs[i] + 2*dxs[i], ..., xs[i+1] } が更新点集合。
	// これを超えない最大の要素を拾う。
	let mut best_x_opt: Option<i128> = None;
	for i in 0..dxs.len() {
		let left = xs[i];
		let right = xs[i + 1];
		let step = dxs[i];

		let first = left + step;
		if n < first {
			break;
		}

		let upto = right.min(n);
		let t = (upto - first) / step; // >=0
		let x_here = first + t * step;

		best_x_opt = Some(x_here);
		if n < right {
			break;
		} // 途中で止まる
	}

	// “本体”側の最小剰余
	let y_min_core = if let Some(x_star) = best_x_opt {
		// x* が取れたので “本体”での値を直接計算
		(a1 * x_star + b_floor) % m1
	} else {
		// 更新点が 1 つも区間に入らない ⇒ x=0 が最小
		b_floor % m1
	};

	// 元スケールに戻す
	b_rem + g * y_min_core
}

// max of (a*x + b) % m on x in [0..=n]
pub fn max_linear_mod(n: i128, m: i128, a: i128, b: i128) -> i128 {
	debug_assert!(n >= 0 && m > 0);

	// 区間長が m 以上なら全剰余を取り得るので最大は m-1
	if n + 1 >= m {
		return m - 1;
	}

	// 最大 = (m-1) - 最小{ (-a*x + (m-1-b)) % m }
	let aa = (-a).rem_euclid(m);
	let bb = (m - 1 - b).rem_euclid(m);
	let mn = min_linear_mod(n, m, aa, bb);
	(m - 1 - mn)
}

fn solve_coprime(a: i128, b: i128, l: i128, r: i128) -> Option<i128> {
	let inv = modinv(a, b);
	let n = r - l;
	let aa = inv.rem_euclid(b);
	let bb = (l.rem_euclid(b) * inv).rem_euclid(b);

	let s_max = max_linear_mod(n, b, aa, bb);
	let lhs = a * s_max;
	if lhs > l + (b - 1) {
		return None;
	}
	let t_max = (r - a * s_max) / b;
	Some(s_max + t_max)
}
