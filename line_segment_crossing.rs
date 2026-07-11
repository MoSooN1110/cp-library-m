// source snippet: key=lib_line_segment_crossing  prefix=lib_line_segment_crossing

fn line_segment_crossing(a1: (f64, f64), a2: (f64, f64), b1: (f64, f64), b2: (f64, f64)) -> bool {
    let mut s = (a1.0 - a2.0) * (b1.1 - a1.1) - (a1.1 - a2.1) * (b1.0 - a1.0);
    let mut t = (a1.0 - a2.0) * (b2.1 - a1.1) - (a1.1 - a2.1) * (b2.0 - a1.0);
    if (s * t > 0.0) {
        return false;
    }
    let mut s = (b1.0 - b2.0) * (a1.1 - b1.1) - (b1.1 - b2.1) * (a1.0 - b1.0);
    let mut t = (b1.0 - b2.0) * (a2.1 - b1.1) - (b1.1 - b2.1) * (a2.0 - b1.0);
    if (s * t > 0.0) {
        return false;
    }
    return true;
}
