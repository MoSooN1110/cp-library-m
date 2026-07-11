// source snippet: key=lib_vector_lcm  prefix=lib_vector_lcm

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

pub fn vector_lcm(vec: &Vec<usize>) -> usize {
    let mut res = 1;
    let mut alcm: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..vec.len() {
        let pf = prime_factorization(vec[i]);
        // println!("{:?}", vec[i]);
        // println!("{:?}", pf);
        for i in pf {
            let t = alcm.get_mut(&i.0);
            if t.is_none() {
                alcm.insert(i.0, i.1);
            } else {
                let val = *t.unwrap();
                *alcm.get_mut(&i.0).unwrap() = max(val, i.1);
            }
        }
    }
    for i in alcm {
        for j in 0..i.1 {
            res *= i.0;
        }
        // println!("{:?}", i);
    }

    res
}
