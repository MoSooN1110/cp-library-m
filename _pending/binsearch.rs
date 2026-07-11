// source snippet: key=lib_binsearch  prefix=lib_binsearch

fn binsearch(ok: i64, ng: i64, f: &dyn Fn(i64) -> bool) -> i64 {
    let mut ok = ok;
    let mut ng = ng;
    while (ng - ok).abs() > 1 {
        let mid = (ok + ng) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    return ok;
}
