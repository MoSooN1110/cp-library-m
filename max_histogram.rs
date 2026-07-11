// source snippet: key=lib_max_histogram  prefix=lib_max_histogram

fn max_histogram(height: &Vec<usize>, width: &Vec<usize>) -> usize {
    let mut height = height.clone();
    height.push(0);
    let mut width = width.clone();
    width.push(0);
    let mut res = 0;
    assert!(height.len() == width.len());
    let n = height.len();
    let mut stack: Vec<(usize, usize)> = vec![];
    let mut pos = 0;
    for i in 0..n {
        let mut npos = pos;
        while !stack.is_empty() && stack.last().unwrap().1 >= height[i] {
            let (p, h) = stack.pop().unwrap();
            res = max(res, h * (pos - p));
            npos = p;
        }

        stack.push((npos, height[i]));
        pos += width[i];
    }
    res
}
