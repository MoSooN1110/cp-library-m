// source snippet: key=lib_lis  prefix=lib_lis

    let mut dp = vec![0; 1];
    for x in v2 {
        if dp[dp.len() - 1] < x {
            dp.push(x);
        } else {
            match dp.binary_search(&x) {
                Err(k) => dp[k] = x,
                _ => (),
            }
        }
    }
