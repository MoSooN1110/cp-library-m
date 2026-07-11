/// {0,1,...,k-1}^n を辞書順に全列挙する Iterator
///
/// 例:
/// for v in product_k_n(3, 4) {
///     // v は長さ4、各要素 0..3
/// }
fn product_k_n(k: usize, n: usize) -> ProductKN {
    ProductKN {
        k,
        n,
        cur: vec![0; n],
        first: true,
        finished: false,
    }
}

struct ProductKN {
    k: usize,
    n: usize,
    cur: Vec<usize>,
    first: bool,
    finished: bool,
}

impl Iterator for ProductKN {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if self.k == 0 {
            self.finished = true;
            return None;
        }

        if self.first {
            self.first = false;
            return Some(self.cur.clone());
        }

        for i in (0..self.n).rev() {
            self.cur[i] += 1;
            if self.cur[i] < self.k {
                return Some(self.cur.clone());
            }
            self.cur[i] = 0;
        }

        self.finished = true;
        None
    }
}
fn main() {
    let (n, k) = readuu();
    let mut data = vec![];
    for _ in 0..n {
        data.push(read_vec::<usize>());
    }

    for v in product_k_n(k, n) {
        // v は長さn、各要素 0..k-1
        // data[v[0]], data[v[1]], ..., data[v[n-1]] を使う
    }
}
