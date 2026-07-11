// source snippet: key=lib_bitbruteforce  prefix=lib_bitbruteforce

    for b in 0..(1 << n) {
        let mut bit = vec![0; 0];
        for j in 0..n {
            bit.push((b >> j) & 1);
        }
        for i in 0..n {
            if bit[i] == 1 {
                //op
            }
        }
    }
