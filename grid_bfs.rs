// source snippet: key=lib_grid_bfs  prefix=lib_grid_bfs

fn grid_01bfs(mat: &Vec<Vec<usize>>, start: (usize, usize)) -> Vec<Vec<usize>> {
    let h = mat.len();
    let w = mat[0].len();
    let mut res = vec![vec![INF as usize; (w) as usize]; (h) as usize];
    let mut q = VecDeque::new();
    q.push_back(start);
    res[start.0][start.1] = 0;
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        for i in -1..=1 {
            for j in -1..=1 {
                if (i * j as i32).abs() == 1 || (i == 0 && j == 0) {
                    continue;
                }
                if v.0 == 0 && i == -1 {
                    continue;
                }

                if v.0 == h - 1 && i == 1 {
                    continue;
                }

                if v.1 == 0 && j == -1 {
                    continue;
                }

                if v.1 == w - 1 && j == 1 {
                    continue;
                }
                let nv = ((v.0 as i32 + i) as usize, (v.1 as i32 + j) as usize);
                let mut d = 1;
                if mat[nv.0][nv.1] == 1 {
                    d = 1;
                } else {
                    d = 0;
                }
                if (res[nv.0][nv.1] > res[v.0][v.1] + d) {
                    res[nv.0][nv.1] = res[v.0][v.1] + d;
                    if d == 0 {
                        q.push_front(nv);
                    } else {
                        q.push_back(nv);
                    }
                }
            }
        }
    }

    return res;
}

fn grid_bfs(mat: &Vec<Vec<usize>>, start: (usize, usize)) -> Vec<Vec<usize>> {
    let h = mat.len();
    let w = mat[0].len();
    let mut res = vec![vec![INF as usize; (w) as usize]; (h) as usize];
    let mut q = VecDeque::new();
    q.push_back(start);
    res[start.0][start.1] = 0;
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        for i in -1..=1 {
            for j in -1..=1 {
                if (i * j as i32).abs() == 1 || (i == 0 && j == 0) {
                    continue;
                }
                if v.0 == 0 && i == -1 {
                    continue;
                }

                if v.0 == h - 1 && i == 1 {
                    continue;
                }

                if v.1 == 0 && j == -1 {
                    continue;
                }

                if v.1 == w - 1 && j == 1 {
                    continue;
                }
                let nv = ((v.0 as i32 + i) as usize, (v.1 as i32 + j) as usize);
                let mut d = 1;
                if mat[nv.0][nv.1] == 1 {
                    continue;
                } else {
                    d = 1;
                }
                if res[nv.0][nv.1] > res[v.0][v.1] + d {
                    res[nv.0][nv.1] = res[v.0][v.1] + d;
                    if d == 0 {
                        q.push_front(nv);
                    } else {
                        q.push_back(nv);
                    }
                }
            }
        }
    }

    return res;
}
