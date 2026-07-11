// source snippet: key=RangeAddSum  prefix=RangeAddSum
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
struct RangeAddSum();
impl SEGimpl for RangeAddSum {
    type Elem = (u64, u64);
    type A = u64;
    type R = u64;
    fn eval(parent: &mut Self::Elem, children: Option<(&mut Self::Elem, &mut Self::Elem)>) {
        let x = parent.1;
        parent.0 += x;
        parent.1 = 0;
        if let Some((c1, c2)) = children {
            c1.1 += x / 2;
            c2.1 += x / 2;
        }
    }
    fn range(x: &Self::A, elem: &mut Self::Elem, l: usize, r: usize) {
        elem.1 += (r - l) as u64 * x;
    }
    fn reduce(parent: &mut Self::Elem, c1: &Self::Elem, c2: &Self::Elem) {
        parent.0 = c1.0 + c2.0;
    }
    fn to_result(elem: Self::Elem) -> Self::R {
        elem.0
    }
}
