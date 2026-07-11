// source snippet: key=lib_vector_compression  prefix=lib_vector_compression

fn vector_compression(vec: &mut Vec<i64>) -> Vec<i64> {
    let n = vec.len();
    let mut res = vec![0; n];
    let mut bt = BTreeMap::new();
    for i in 0..n {
        *bt.entry(vec[i]).or_insert(1) += 1;
    }
    let mut bt2 = BTreeMap::new();
    let mut cnt = 0;
    for i in bt {
        bt2.insert(i.0, cnt);
        cnt += 1;
    }
    for i in 0..n {
        // let mut xx: usize = vec[i];
        res[i] = *bt2.get(&(vec[i])).unwrap();
    }

    return res;
}
