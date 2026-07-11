// source snippet: key=lib_prime_factoriaztion  prefix=lib_prime_factoriaztion

fn prime_factorization(x: usize) -> BTreeMap<usize, usize> {
    let mut res: BTreeMap<usize, usize> = BTreeMap::new();
    let mut xx = x;
    let mut p: usize = 2;
    while p * p <= xx {
        while xx % p == 0 {
            // println!("{:?}", p);
            let t = res.get_mut(&p);
            if t.is_none() {
                res.insert(p, 1);
            } else {
                *t.unwrap() += 1;
            }
            xx /= p;
        }
        // println!("{:?} {:?}", p, res);
        p += 1;
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
