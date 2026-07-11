// source snippet: key=lib_euler_phi_function  prefix=lib_euler_phi_function

    fn euler_phi(n: usize) -> usize {
        let mut n = n;
        let mut res = n;
        for i in 2..(n as f64).sqrt() as usize + 1 {
            if n % i == 0 {
                while n % i == 0 {
                    n /= i;
                }
                res -= res / i;
            }
        }
        if n > 1 {
            res -= res / n;
        }
        return res;
    }
