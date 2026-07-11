// source snippet: key=lib_run_length_encoding  prefix=lib_run_length_encoding

    let mut data = vec![(0, 0 as usize); 0];
    data.push((v[0], 1));
    let n = v.len();
    for i in 1..n {
        if data.last().unwrap().0 == v[i] {
            // *(data.last().unwrap()).1
            let nn = data.len();
            data[nn - 1].1 += 1;
        } else {
            data.push((v[i], 1));
        }
    }
