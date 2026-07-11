// source snippet: key=lib_matrix_multiplication  prefix=lib_matrix_multiplication

    let mut vv = vec![vec![0 as i64; (n) as usize]; (n) as usize];
    for i in 0..n {
        for j in 0..n {
            vv[i][j] += 1;
        }
    }
    let mut m = Matrix { v: vv };
    let m = m.pow(n as u64, MOD);
    let mut a = vec![vec![0 as i64; (1) as usize]; (n) as usize];
    a[0][0] = 1;
    let aa = Matrix { v: a };
    println!("{:?}", m.v.clone());
    let p = m * aa;
    println!("{:?}", p.v[0][0]);
