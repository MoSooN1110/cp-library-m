// source snippet: key=lib_f64total  prefix=lib_f64total

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
