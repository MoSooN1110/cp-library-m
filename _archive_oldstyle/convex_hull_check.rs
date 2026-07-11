// source snippet: key=convex_hull_check  prefix=convex_hull_check
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
/// A check function for convex hull trick
pub fn convex_hull_check((a1, b1): (i64, i64), (a2, b2): (i64, i64), (a3, b3): (i64, i64)) -> bool {
    (a2 as f64 - a1 as f64) * (b3 as f64 - b2 as f64)
        >= (b2 as f64 - b1 as f64) * (a3 as f64 - a2 as f64)
}
