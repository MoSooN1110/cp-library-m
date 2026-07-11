// source snippet: key=partition_dp  prefix=partition_dp
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
/// dp[i][j] = j th partition number of i
pub fn partition_dp(n: usize, m: usize, p: u64) -> Vec<Vec<u64>> {
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..m + 1 {
        dp[0][i] = 1;
    }
    for i in 1..n + 1 {
        for j in 1..m + 1 {
            if i >= j {
                dp[i][j] = (dp[i - j][j] + dp[i][j - 1]) % p;
            } else {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }
    dp
}
