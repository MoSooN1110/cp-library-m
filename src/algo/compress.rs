//! 座標圧縮。値を 0..k の連番に対応づける。
//!
//! ```
//! use cplib::algo::compress::*;
//! let c = Compress::new(vec![40, 10, 40, 30]);
//! assert_eq!(c.len(), 3);            // {10,30,40}
//! assert_eq!(c.index(&40), 2);
//! assert_eq!(c.value(0), 10);
//! ```

pub struct Compress<T> {
    sorted: Vec<T>,
}

impl<T: Ord + Clone> Compress<T> {
    pub fn new(mut vals: Vec<T>) -> Self {
        vals.sort();
        vals.dedup();
        Compress { sorted: vals }
    }
    /// 異なる値の個数
    pub fn len(&self) -> usize {
        self.sorted.len()
    }
    pub fn is_empty(&self) -> bool {
        self.sorted.is_empty()
    }
    /// x の圧縮後インデックス（x は登録済みであること）
    pub fn index(&self, x: &T) -> usize {
        self.sorted.partition_point(|v| v < x)
    }
    /// x 未満の値の個数（x が未登録でも可）
    pub fn count_less(&self, x: &T) -> usize {
        self.sorted.partition_point(|v| v < x)
    }
    /// インデックス i の元の値
    pub fn value(&self, i: usize) -> T {
        self.sorted[i].clone()
    }
    /// 配列全体を圧縮後インデックス列に変換
    pub fn transform(&self, a: &[T]) -> Vec<usize> {
        a.iter().map(|x| self.index(x)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let c = Compress::new(vec![100, 5, 100, 5, 42, -3]);
        assert_eq!(c.len(), 4); // -3,5,42,100
        assert_eq!(c.index(&-3), 0);
        assert_eq!(c.index(&100), 3);
        assert_eq!(c.value(2), 42);
        assert_eq!(c.count_less(&50), 3);
        assert_eq!(c.transform(&[100, -3, 42]), vec![3, 0, 2]);
    }
}
