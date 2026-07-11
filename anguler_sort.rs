// source snippet: key=lib_anguler_sort  prefix=lib_anguler_sort

#[derive(PartialEq, PartialOrd, Clone, Copy)]
struct F64Total(f64);
impl Eq for F64Total {}
impl Ord for F64Total {
    fn cmp(&self, other: &F64Total) -> Ordering {
        let F64Total(f1) = *self;
        let F64Total(f2) = *other;
        if f1.is_nan() {
            Less
        } else if f2.is_nan() {
            Greater
        } else {
            if (f1 - f2).is_sign_positive() {
                Greater
            } else {
                Less
            }
        }
    }
}

/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

fn angular_sort(data: &Vec<(f64, f64)>) -> Vec<(f64, f64, f64)> {
    let mut res = vec![];
    let n = data.len();
    let c = (0.0 as f64, 0.0 as f64); //centre
    let mut deg1 = vec![];
    for j in 0..n {
        let p = data[j];
        let mut x = p.1.atan2(p.0);
        if x < 0.0 {
            x += 2.0 * std::f64::consts::PI;
        }
        let deg = F64Total(x / std::f64::consts::PI * 180.0);
        deg1.push((deg, j));
    }
    deg1.sort();
    for i in 0..n {
        res.push(((deg1[i].0).0, data[deg1[i].1].0, data[deg1[i].1].1));
    }
    res
}
