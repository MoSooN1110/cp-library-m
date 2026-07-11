// source snippet: key=lib_segment_dp  prefix=lib_segment_dp

fn segment_dp(l: usize, r: usize, s: &Vec<char>, dp: &mut Vec<Vec<usize>>) -> usize {
    let mut res = 0 as usize;
    if dp[l][r] != INF as usize {
        return dp[l][r];
    }
    if l == r {
        dp[l][r] = 0;
        return 0;
    }
    for i in l + 1..r {
        res = max(res, segment_dp(l, i, s, dp) + segment_dp(i, r, s, dp));
    }
    if s[l] == 'i' && s[r - 1] == 'i' {
        for i in l + 1..r - 1 {
            if s[i] == 'w'
                && segment_dp(l + 1, i, s, dp) == i - l - 1
                && segment_dp(i + 1, r - 1, s, dp) == r - i - 2
            {
                //完全に消せるときにiwiを加えた区間は完全に消せる
                res = max(res, (r - l));
            }
        }
    }
    dp[l][r] = res;

    return res;
}
