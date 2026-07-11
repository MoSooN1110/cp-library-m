// source snippet: key=is_intersected  prefix=is_intersected
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

/// Is line a-b and line c-d intersected ?
pub fn is_intersected(a: Vector2D, b: Vector2D, c: Vector2D, d: Vector2D) -> bool {
    let ta = (c.0 - d.0) * (a.1 - c.1) + (c.1 - d.1) * (c.0 - a.0);
    let tb = (c.0 - d.0) * (b.1 - c.1) + (c.1 - d.1) * (c.0 - b.0);
    let tc = (a.0 - b.0) * (c.1 - a.1) + (a.1 - b.1) * (a.0 - c.0);
    let td = (a.0 - b.0) * (d.1 - a.1) + (a.1 - b.1) * (a.0 - d.0);
    tc * td <= 0.0 && ta * tb <= 0.0
}
