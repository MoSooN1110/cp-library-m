// source snippet: key=lis  prefix=lis
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

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
#[derive(PartialEq, Eq, Clone, Debug)]
enum Inf<T> {
    Val(T),
    Inf,
}
impl<T: Ord> Inf<T> {
    #[allow(dead_code)]
    fn val(self) -> Option<T> {
        match self {
            Inf::Val(v) => Some(v),
            _ => None,
        }
    }
}
impl<T: PartialOrd> PartialOrd for Inf<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Inf::Inf, &Inf::Inf) => Some(Ordering::Equal),
            (&Inf::Inf, &Inf::Val(_)) => Some(Ordering::Greater),
            (&Inf::Val(_), &Inf::Inf) => Some(Ordering::Less),
            (&Inf::Val(ref a), &Inf::Val(ref b)) => a.partial_cmp(b),
        }
    }
}
impl<T: Ord> Ord for Inf<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&Inf::Inf, &Inf::Inf) => Ordering::Equal,
            (&Inf::Inf, &Inf::Val(_)) => Ordering::Greater,
            (&Inf::Val(_), &Inf::Inf) => Ordering::Less,
            (&Inf::Val(ref a), &Inf::Val(ref b)) => a.cmp(b),
        }
    }
}
#[allow(dead_code)]
/// Calculate length of Longest Increasing Subsequence. O(N log N)
pub fn lis<T: Ord>(seq: &[T]) -> usize {
    let mut dp: Vec<Inf<&T>> = vec![Inf::Inf; seq.len() + 1];
    for x in seq.iter() {
        let i = dp.lower_bound(&Inf::Val(x));
        dp[i] = Inf::Val(x);
    }
    dp.lower_bound(&Inf::Inf)
}
