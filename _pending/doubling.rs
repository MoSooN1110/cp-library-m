// source snippet: key=lib_doubling  prefix=lib_doubling

pub struct Doubling {
    n: usize,
    log: usize,
    table: Vec<Vec<i64>>,
}

impl Doubling {
    fn new(n: usize, max_loop: usize) -> Doubling {
        Doubling {
            n,
            log: ((max_loop as f64).log2().floor()) as usize + 1,
            table: vec![vec![-1; n]; (max_loop as f64).log2().floor() as usize + 2],
        }
    }
    fn set_next(&mut self, i: usize, x: usize) {
        self.table[0][i] = x as i64;
        return;
    }
    fn build(&mut self) {
        for k in 0..self.log {
            for i in 0..self.table[k].len() {
                if self.table[k][i] == -1 {
                    self.table[k + 1][i] = -1;
                } else {
                    self.table[k + 1][i] = self.table[k][self.table[k][i] as usize];
                }
            }
        }
        return;
    }

    fn query(&mut self, mut k: usize, t: usize) -> usize {
        for i in (0..self.log).rev() {
            if (t >> i) & 1 == 1 {
                k = self.table[i][k] as usize;
            }
        }
        return k;
    }
}
