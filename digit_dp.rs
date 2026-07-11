// source snippet: key=lib_digit_dp  prefix=lib_digit_dp

fn digit_dp(s: String) -> usize {
    let mut v = s.as_bytes().to_vec();
    for i in 0..v.len() {
        v[i] -= '0' as u8;
    }
    let n = v.len();
    let mut dp = vec![vec![vec![0 as usize; 10]; (2) as usize]; (v.len() + 1) as usize];
    dp[0][0][0] = 1;
    for i in 0..n {
        for j in 0..2 {
            let mut ld = 0;
            if j == 1 {
                ld = 9;
            } else {
                ld = v[i];
            }
            for d_f in 0..10 {
                for d_t in 0..=ld {
                    dp[i + 1][(j == 1 || (d_t < v[i])) as i64 as usize][d_t as usize] +=
                        dp[i][j as usize][d_f];
                }
            }
        }
    }
    return 1;
}
