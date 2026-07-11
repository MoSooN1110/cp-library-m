// source snippet: key=mlib_segment_sum_2d  prefix=mlib_segment_sum_2d

struct SegmentSum2D<T> {
    arr: Vec<Vec<T>>,
}
impl<T: Copy + Eq + Add<Output = T> + Sub<Output = T> + Default> SegmentSum2D<T> {
    fn new(arr: Vec<Vec<T>>) -> Self {
        let mut carr = arr.clone();
        carr.insert(0, vec![T::default(); carr[0].len()]);
        for i in 0..carr.len() {
            carr[i].insert(0, T::default());
        }
        Self { arr: carr }
    }
    fn build(&mut self) {
        for i in 1..self.arr.len() {
            for j in 1..self.arr[i].len() {
                self.arr[i][j] = self.arr[i][j] + self.arr[i - 1][j] + self.arr[i][j - 1]
                    - self.arr[i - 1][j - 1];
            }
        }
    }
    fn qerry(&self, i1: usize, i2: usize, j1: usize, j2: usize) -> T {
        // [i1, i2) * [j1, j2)
        assert!(i1 <= i2 && i2 < self.arr.len());
        assert!(j1 <= j2 && j2 < self.arr[0].len());
        self.arr[i2][j2] - self.arr[i1][j2] - self.arr[i2][j1] + self.arr[i1][j1]
    }
}
