// source snippet: key=lib_fht  prefix=lib_fht

fn fht(a: &mut [usize], m: usize, invert: bool) {
    let n = a.len();
    let mut step = 1;
    while step < n {
        for i in (0..n).step_by(step * 2) {
            for j in 0..step {
                let u = a[i + j];
                let v = a[i + j + step];
                a[i + j] = (u + v) % m;
                a[i + j + step] = (u + m - v) % m;
            }
        }
        step *= 2;
    }
    if invert {
        let inv_n = inv(n);//inv of n to modify
        for x in a.iter_mut() {
            *x = *x * inv_n % m;
        }
    }
}
