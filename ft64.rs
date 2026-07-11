// source snippet: key=lib_ft64  prefix=lib_ft64

#[derive(PartialEq, PartialOrd, Clone, Copy)]
struct FT64(f64);

impl Eq for FT64 {}
impl Ord for FT64 {
    fn cmp(&self, other: &FT64) -> Ordering {
        let FT64(f1) = *self;
        let FT64(f2) = *other;
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
