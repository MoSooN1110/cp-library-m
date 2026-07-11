// source snippet: key=lib_lcs  prefix=lib_lcs

fn lcs(v1: &Vec<char>, v2: &Vec<char>) -> Vec<char> {
    let mut n = v1.len();
    let mut m = v2.len();
    let mut dp = vec![vec![0 as usize; (v2.len() + 1) as usize]; (v1.len() + 1) as usize];
    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j + 1] = max(dp[i][j + 1], dp[i + 1][j + 1]);
            dp[i + 1][j + 1] = max(dp[i + 1][j], dp[i + 1][j + 1]);
            if v1[i] == v2[j] {
                dp[i + 1][j + 1] = max(dp[i][j] + 1, dp[i + 1][j + 1]);
            }
        }
    }
    let mut val = dp[n][m];
    let mut r = vec![];
    let mut pos = m;
    for i in (1..=n).rev() {
        for j in (1..=pos).rev() {
            // d!((i, j));
            if dp[i][j] == val
                && (dp[i][j] == dp[i - 1][j - 1] + 1)
                && dp[i][j] != dp[i][j - 1]
                && dp[i][j] != dp[i - 1][j]
            {
                val -= 1;
                // println!("{:?}", (i, j, val));
                pos = j - 1;
                r.push(v2[j - 1]);
                break;
            }
        }
    }
    // d!(dp);
    if val != 0 {
        assert!(false, "lcs function has error");
    }
    r.reverse();
    return r;
}
