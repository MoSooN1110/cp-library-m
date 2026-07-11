// source snippet: key=closest_pair  prefix=closest_pair
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

pub fn closest_pair(ps: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    fn d(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
    }
    fn rec(x_sort: &[(f64, f64)], y_sort: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
        if x_sort.len() <= 3 {
            let mut min_d = std::f64::MAX;
            let mut pair = ((0.0, 0.0), (0.0, 0.0));
            for (i, &p1) in x_sort.iter().enumerate() {
                for (j, &p2) in x_sort.iter().enumerate() {
                    if i != j {
                        let dist = d(p1, p2);
                        if dist < min_d {
                            min_d = dist;
                            pair = (p1, p2);
                        }
                    }
                }
            }
            return pair;
        }
        let mid = x_sort.len() / 2;
        let pivot = x_sort[mid].0;
        let q_x = &x_sort[..mid];
        let r_x = &x_sort[mid..];
        let mut q_y = Vec::with_capacity(mid);
        let mut r_y = Vec::with_capacity(x_sort.len() - mid);
        for &(x, y) in y_sort {
            if x < pivot {
                q_y.push((x, y));
            } else {
                r_y.push((x, y));
            }
        }
        let pair1 = rec(q_x, &q_y);
        let pair2 = rec(r_x, &r_y);
        let w = d(pair1.0, pair1.1).min(d(pair2.0, pair2.1));
        let s: Vec<(f64, f64)> = y_sort
            .iter()
            .filter(|&&(x, _)| (pivot - x).abs() <= w)
            .cloned()
            .collect();
        let mut min_d = w;
        let mut pair = if d(pair1.0, pair1.1) < d(pair2.0, pair2.1) {
            pair1
        } else {
            pair2
        };
        for (i, &p1) in s.iter().enumerate() {
            for &p2 in s[i + 1..].iter().take(15) {
                let dist = d(p1, p2);
                if dist < min_d {
                    min_d = dist;
                    pair = (p1, p2);
                }
            }
        }
        pair
    }
    let mut x_sort = ps.to_vec();
    let mut y_sort = ps.to_vec();
    x_sort.sort_by_key(|p| Total(p.0));
    y_sort.sort_by_key(|p| Total(p.1));
    rec(&x_sort, &y_sort)
}
