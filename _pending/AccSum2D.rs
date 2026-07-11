// source snippet: key=lib_AccSum2D  prefix=lib_AccSum2D

struct CumSum2 {
    base: Vec<Vec<i64>>,
    dp: Vec<Vec<i64>>,
}

impl CumSum2 {
    fn new(n: usize, m: usize) -> CumSum2 {
        CumSum2 {
            base: vec![vec![0; m]; n],
            dp: vec![],
        }
    }
    #[doc = "i~j"]
    fn add(&mut self, i: usize, j: usize, x: i64) {
        self.base[i][j] += x;
    }
    #[doc = "i~j"]
    fn set(&mut self, i: usize, j: usize, x: i64) {
        self.base[i][j] = x;
    }
    fn build(&mut self) {
        let n = self.base.len();
        let m = self.base[0].len();
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                dp[i + 1][j + 1] = self.base[i][j];
            }
        }
        for i in 1..n + 1 {
            for j in 1..m + 1 {
                dp[i][j] += dp[i - 1][j] + dp[i][j - 1] - dp[i - 1][j - 1];
            }
        }
        self.dp = dp;
    }
    #[doc = "[i0,i1)~[j0,j1)"]
    fn query(&self, i0: usize, i1_: usize, j0: usize, j1_: usize) -> i64 {
        self.dp[i1_][j1_] - (self.dp[i0][j1_] + self.dp[i1_][j0] - self.dp[i0][j0])
    }
}
