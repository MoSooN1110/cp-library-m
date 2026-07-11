// source snippet: key=lib_combtable_dp  prefix=lib_combtable_dp

    let mut comb = vec![vec![0.0; (2010) as usize]; (2010) as usize];
    comb[0][0] = 1.0;
    for i in 0..2005 {
        for j in 0..=i {
            comb[i + 1][j] += comb[i][j] / 2.0;
            comb[i + 1][j + 1] += comb[i][j] / 2.0;
        }
    }
