// source snippet: key=lib_prime_table  prefix=lib_prime_table

fn prime_table(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    let mut res = vec![0 as usize; 0];

    if n >= 0 {
        is_prime[0] = false;
    }
    if n >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2 as usize;
    while i * i <= n {
        if !is_prime[i] {
            i += 1;
            continue;
        }
        // println!("{:?}", "call");

        let mut j = i + i as usize;
        while j <= n {
            is_prime[j] = false;
            j += i;
        }
        i += 1;
    }
    for i in 0..n {
        if is_prime[i] == true {
            res.push(i);
        }
    }

    return res;
}
