// source snippet: key=lib_ext_gcd_lde  prefix=lib_ext_gcd_lde

struct LDE {
    a: i64,
    b: i64,
    c: i64,
    m: i64,
    x: i64,
    y: i64,
    check: bool,
}

impl LDE {
    // 初期化
    fn new(a_input: i64, b_input: i64, c_input: i64) -> LDE {
        let mut a = a_input;
        let mut b = b_input;
        let c = c_input;
        let mut x = 0i64;
        let mut y = 0i64;
        let m = 0i64;
        let mut check = true;
        let g = gcd(a, b);
        if c % g != 0 {
            check = false;
        } else {
            // ax + by = g の特殊解を求める
            let mut x0 = 0i64;
            let mut y0 = 0i64;
            extgcd(a.abs(), b.abs(), &mut x0, &mut y0);
            if a < 0 {
                x0 = -x0;
            }
            if b < 0 {
                y0 = -y0;
            }
            // ax + by = c の特殊解を求める
            x = x0 * (c / g);
            y = y0 * (c / g);
            // 一般解を求めるために
            a /= g;
            b /= g;
        }
        LDE {
            a,
            b,
            c,
            m,
            x,
            y,
            check,
        }
    }

    // パラメータ m の更新（書き換え）
    fn m_update(&mut self, m_new: i64) {
        self.x += (m_new - self.m) * self.b;
        self.y -= (m_new - self.m) * self.a;
        self.m = m_new;
    }
}

// ユークリッドの互除法による最大公約数の計算
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

// 拡張ユークリッドの互除法
fn extgcd(a: i64, b: i64, x0: &mut i64, y0: &mut i64) -> i64 {
    if b == 0 {
        *x0 = 1;
        *y0 = 0;
        a
    } else {
        let d = extgcd(b, a % b, y0, x0);
        *y0 -= (a / b) * (*x0);
        d
    }
}
// このRustコードは、PythonのクラスLDEを再現しており、二元一次不定方程式 ax + by = c (a≠0かつb≠0) を解くことができます。初期化すると、一般解 x = x0 + m*b、y = y0 - m*a が得られます（m=0で初期化）。
