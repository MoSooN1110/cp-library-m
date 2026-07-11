// source snippet: key=lib_binary_search  prefix=lib_binary_search

    let f = |x: i64| -> bool {
        return true;
    };

    let mut ok = 0;
    let mut ng = INF;

    while (ng - ok).abs() > 1 {
        let mid = (ok + ng) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
