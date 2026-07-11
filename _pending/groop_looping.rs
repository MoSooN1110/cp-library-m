// source snippet: key=lib_groop_looping  prefix=lib_groop_looping

fn gorup_looping(s: usize, num: usize, vec: &Vec<usize>) -> usize {
    let mut res = 0;
    let k = num;
    let n = vec.len();
    let mut x = s;
    let mut c = vec![0; n];
    let mut cnt = 0;
    while true {
        if c[x] == 2 {
            break;
        }
        c[x] += 1;
        x = vec[x];
        cnt += 1;
        if cnt == k {
            return x;
        }
    }

    let mut c2 = 0;
    let mut c1 = 0;
    for i in 0..n {
        if c[i] == 2 {
            c2 += 1;
        }
        if c[i] == 1 {
            c1 += 1;
        }
    }
    // let su1 = su - su2;
    let nn = (k - c1) % c2;
    // p!(res);
    for i in 0..(nn % c2) {
        c[x] += 1;
        x = vec[x];
    }
    // d!(c);
    res = x;
    res
}
