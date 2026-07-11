// source snippet: key=lib_osak_prime_factorization  prefix=lib_osak_prime_factorization

fn prime_table_min(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    let mut res = vec![0 as usize; n + 1];

    for i in 0..=n {
        res[i] = i;
    }

    let mut i = 2 as usize;
    while i * i <= n {
        if !is_prime[i] {
            i += 1;
            continue;
        }
        // println!("{:?}", "call");
        // is_prime[i] = i;

        let mut j = i + i as usize;
        while j <= n {
            is_prime[j] = false;
            res[j] = min(res[j], i);
            j += i;
        }
        i += 1;
    }

    return res;
}

fn prime_factorization_osak(x: usize, table: &Vec<usize>) -> BTreeMap<usize, usize> {
    let mut res: BTreeMap<usize, usize> = BTreeMap::new();
    let mut xx = x;
    // let mut p: usize = 2;
    while xx > 1 {
        let mut p = table[xx];
        let t = res.get_mut(&p);
        if t.is_none() {
            res.insert(p, 1);
        } else {
            *t.unwrap() += 1;
        }
        xx /= table[xx];

        // println!("{:?} {:?}", p, res);
        // p += 1;
    }

    if xx != 1 {
        let t = res.get_mut(&xx);
        if t.is_none() {
            res.insert(xx, 1);
        } else {
            *t.unwrap() += 1;
        }
    }
    res
}
