// source snippet: key=lib_histgram_max_area  prefix=lib_histgram_max_area

struct Rect {
    h: usize,
    p: usize,
}

fn max_area_in_histgram(hist: &[usize]) -> usize {
    let mut max_v = 0;

    let mut h = vec![];
    for &x in hist {
        h.push(x);
    }
    h.push(0); // sentinel

    let mut stack = vec![];

    for i in 0..h.len() {
        let cur_h = h[i];
        if stack.is_empty() {
            stack.push(Rect { h: cur_h, p: i });
        } else if stack.last().unwrap().h <= cur_h {
            stack.push(Rect { h: cur_h, p: i });
        } else if stack.last().unwrap().h > cur_h {
            let mut new_i = i;
            while !stack.is_empty() && stack.last().unwrap().h > cur_h {
                let rect = stack.pop().unwrap();
                new_i = rect.p;
                max_v = max(max_v, (i - new_i) * rect.h);
            }
            stack.push(Rect { h: cur_h, p: new_i })
        }
    }

    max_v
}
