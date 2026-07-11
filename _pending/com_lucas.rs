// source snippet: key=lib_com_lucas  prefix=lib_com_lucas

fn com_lucas(n: usize, k: usize, p: usize, dp: &Vec<Vec<usize>>) -> usize {
    if k == 0 || n == k {
        return 1;
    }
    let mut res = 1;
    let mut nn = n;
    let mut kk = k;
    while nn > 0 {
        let a = nn % p;
        let b = kk % p;
        nn /= p;
        kk /= p;
        res *= dp[a][b];
        res %= p;
        // println!("{:?}", r);
    }
    res %= p;
    res
}

fn com_lucas_dp_gen(n: usize, p: usize) -> Vec<Vec<usize>> {
    let mut vv = vec![vec![0 as usize; (n + 1) as usize]; (n + 1) as usize];
    vv[0][0] = 1;
    for i in 1..n {
        vv[i][0] = 1;
        for j in (1..=i).rev() {
            vv[i][j] = (vv[i - 1][j - 1] + vv[i - 1][j]) % p;
        }
    }
    vv
}

#[test]
fn test_com_lucas() {
    let dp = com_lucas_dp_gen(100, 3);
    assert_eq!(com_lucas(7, 2, 2, &dp), 1);
    assert_eq!(com_lucas(4, 2, 3, &dp), 0);
}
