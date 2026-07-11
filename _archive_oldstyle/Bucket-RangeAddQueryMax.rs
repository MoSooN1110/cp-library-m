// source snippet: key=Bucket-RangeAddQueryMax  prefix=Bucket-RangeAddQueryMax
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
struct RangeAddQueryMax();
impl BucketImpl for RangeAddQueryMax {
    type Elem = u64;
    type Parent = (u64, u64);
    type A = u64;
    type R = u64;
    fn reduce_parent(p: &mut Self::Parent, e: &Self::Elem) {
        p.0 = max(p.1 + e, p.0);
    }
    fn add(p: &mut Self::Parent, e: &mut Self::Elem, v: &Self::A) {
        *e += v;
        p.0 = max(p.0, *e + p.1);
    }
    fn add_parent(p: &mut Self::Parent, d: &Self::A) {
        p.0 += d;
        p.1 += d;
    }
    fn parent_to_result(p: &Self::Parent) -> Self::R {
        p.0
    }
    fn elem_to_result(e: &Self::Elem, p: &Self::Parent) -> Self::R {
        e + p.1
    }
    fn reduce_result(a: &mut Self::R, b: &Self::R) {
        *a = max(*a, *b);
    }
}
