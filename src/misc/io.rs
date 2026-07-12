//! 高速入力スキャナ（stdin 一括読み・空白区切りトークン）。マクロ不使用で expander 安全。
//!
//! 出力側は `let mut out = std::io::BufWriter::new(std::io::stdout().lock());` と
//! `writeln!(out, ...)` の定型で十分（本モジュールでは提供しない）。
//!
//! ```
//! use cplib::misc::io::*;
//! let mut sc = Scanner::from_str("3 hello\n1 2 3\n");
//! let n: usize = sc.read();
//! let s: String = sc.read();
//! let v: Vec<i64> = sc.vec(n);
//! assert_eq!((n, s.as_str(), v), (3, "hello", vec![1, 2, 3]));
//! ```

pub struct Scanner {
    buf: String,
    pos: usize,
}

impl Scanner {
    /// stdin を EOF まで一括で読み込む（本番用）。
    pub fn new() -> Self {
        let mut buf = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin().lock(), &mut buf).unwrap();
        Self { buf, pos: 0 }
    }

    /// 文字列から作る（テスト用）。
    pub fn from_str(s: &str) -> Self {
        Self { buf: s.to_string(), pos: 0 }
    }

    fn token(&mut self) -> &str {
        let bytes = self.buf.as_bytes();
        while self.pos < bytes.len() && bytes[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        let start = self.pos;
        while self.pos < bytes.len() && !bytes[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        assert!(start < self.pos, "Scanner: no more tokens");
        &self.buf[start..self.pos]
    }

    /// 次のトークンを型 T として読む。
    pub fn read<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.token().parse().unwrap()
    }

    /// 1-indexed の整数を読んで 0-indexed にする。
    pub fn usize1(&mut self) -> usize {
        self.read::<usize>() - 1
    }

    /// n 個のトークンを Vec として読む。
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T>
    where
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }

    /// n 個の (T, U) 組を読む。
    pub fn vec2<T: std::str::FromStr, U: std::str::FromStr>(&mut self, n: usize) -> Vec<(T, U)>
    where
        T::Err: std::fmt::Debug,
        U::Err: std::fmt::Debug,
    {
        (0..n).map(|_| (self.read(), self.read())).collect()
    }

    /// 次のトークンをバイト列として読む（文字列問題用）。
    pub fn bytes(&mut self) -> Vec<u8> {
        self.token().bytes().collect()
    }

    /// 次のトークンを char の Vec として読む。
    pub fn chars(&mut self) -> Vec<char> {
        self.token().chars().collect()
    }

    /// 残りトークンが存在するか。
    pub fn has_next(&mut self) -> bool {
        let bytes = self.buf.as_bytes();
        while self.pos < bytes.len() && bytes[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        self.pos < bytes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_types() {
        let mut sc = Scanner::from_str("  42 -7 3.5 abc\nxyz  ");
        assert_eq!(sc.read::<usize>(), 42);
        assert_eq!(sc.read::<i64>(), -7);
        assert!((sc.read::<f64>() - 3.5).abs() < 1e-9);
        assert_eq!(sc.bytes(), b"abc".to_vec());
        assert_eq!(sc.chars(), vec!['x', 'y', 'z']);
        assert!(!sc.has_next());
    }

    #[test]
    fn test_vec_and_usize1() {
        let mut sc = Scanner::from_str("3\n1 2\n3 4\n5 6\n1");
        let n: usize = sc.read();
        let ps: Vec<(i64, i64)> = sc.vec2(n);
        assert_eq!(ps, vec![(1, 2), (3, 4), (5, 6)]);
        assert_eq!(sc.usize1(), 0);
    }

    #[test]
    #[should_panic]
    fn test_empty_panics() {
        let mut sc = Scanner::from_str("   ");
        let _: i64 = sc.read();
    }
}
