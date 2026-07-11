// source snippet: key=lib_ternaly_search  prefix=lib_ternaly_search

    let f = |x: i64| -> bool { return true };

    let mut l = 0;
    let mut r = INF;

    for i in 0..500 {
        let mid = (l + r) / 2;
        let c1 = (2 * l + r) / 3;
        let c2 = (l + 2 * r) / 3;
        if f(c1) > f(c2) {
            l = c1;
        } else {
            r = c2;
        }
    }
