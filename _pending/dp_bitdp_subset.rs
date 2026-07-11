// source snippet: key=lib_dp_bitdp_subset  prefix=lib_dp_bitdp_subset

    let mut dp = vec![vec![INF; (1 << n) as usize]; (k + 1) as usize];
    dp[0][0] = 0;
    for i in 1..=k {
        for j in 0..(1 << n) {
            let mut s = j;
            while s != 0 {
                dp[i][j] = min(dp[i][j], max(dp[i - 1][j - s], cost[s]));
                s = (s - 1) & j;
            }
        }
    }

    println!("{:?}", dp[k][(1 << n) - 1]);
