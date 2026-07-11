// source snippet: key=lib_prime_count  prefix=lib_prime_count

//https://yukicoder.me/submissions/677252
fn prime_pi_fast(n: usize) -> usize {
    if n <= 3 {
        return n.saturating_sub(1);
    }
    let v = floor_sqrt(n);
    let mut smalls: Vec<_> = (0..=v).map(|i| (i + 1) / 2).collect();
    let mut s = (v + 1) / 2;
    let mut roughs: Vec<_> = (0..s).map(|i| 2 * i + 1).collect();
    let mut larges: Vec<_> = (0..s).map(|i| (n / (2 * i + 1) + 1) / 2).collect();
    let mut skip = vec![false; v + 1];

    let mut pc = 0;
    for p in (3..=v).step_by(2) {
        if skip[p] {
            continue;
        }
        let q = p * p;
        pc += 1;
        if q * q > n {
            break;
        }
        skip[p] = true;
        for i in (q..=v).step_by(2 * p) {
            skip[i] = true;
        }
        let mut ns = 0;
        for k in 0..s {
            let i = roughs[k];
            if skip[i] {
                continue;
            }
            let d = i * p;
            let x = if d <= v {
                larges[smalls[d] - pc]
            } else {
                smalls[n / d]
            };
            larges[ns] = larges[k] + pc - x;
            roughs[ns] = i;
            ns += 1;
        }
        s = ns;
        let mut i = v;
        for j in (p..=v / p).rev() {
            let c = smalls[j] - pc;
            let e = j * p;
            while i >= e {
                smalls[i] -= c;
                i -= 1;
            }
        }
    }

    let roughs = roughs;
    let pc = pc;

    let mut res: usize =
        larges[0] + (s + 2 * (pc - 1)) * (s - 1) / 2 - larges[1..s].iter().sum::<usize>();

    for l in 1..s {
        let q = roughs[l];
        let m = n / q;
        let e = smalls[m / q] - pc;
        if e <= l {
            break;
        }
        let t: usize = roughs[l + 1..=e].iter().map(|&r| smalls[m / r]).sum();
        res += t - (e - l) * (pc + l - 1);
    }
    res
}

fn floor_sqrt(n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    let mut lo = 1;
    let mut hi = n;
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        match mid.overflowing_mul(mid) {
            (x, false) if x <= n => lo = mid,
            _ => hi = mid,
        }
    }
    lo
}
