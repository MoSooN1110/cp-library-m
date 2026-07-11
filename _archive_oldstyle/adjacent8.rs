// source snippet: key=adjacent8  prefix=adjacent8
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
fn adjacent8(x: usize, y: usize, sx: usize, sy: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ]
    .iter()
    .filter_map(move |&(dx, dy)| {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && nx < sx as isize && ny >= 0 && ny < sy as isize {
            Some((nx as usize, ny as usize))
        } else {
            None
        }
    })
}
