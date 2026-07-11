// source snippet: key=convex_hull  prefix=convex_hull
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
fn convex_hull(vs: &[Vector2D]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..vs.len()).collect();
    idx.sort_by_key(|&i| Total((vs[i].0, vs[i].1)));
    let mut res = Vec::new();
    for &i in &idx {
        while res.len() > 1
            && Vector2D::det(
                vs[res[res.len() - 1]] - vs[res[res.len() - 2]],
                vs[i] - vs[res[res.len() - 1]],
            ) <= 0.0
        {
            res.pop();
        }
        res.push(i);
    }
    let t = res.len();
    for &i in idx.iter().rev().skip(1) {
        while res.len() > t
            && (vs[res[res.len() - 1]] - vs[res[res.len() - 2]]).det(vs[i] - vs[res[res.len() - 1]])
                <= 0.0
        {
            res.pop();
        }
        res.push(i);
    }
    res.pop();
    res
}
