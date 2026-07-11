// source snippet: key=lib_grid_congruence  prefix=lib_grid_congruence

fn grid_congruence(n: usize, s: Vec<Vec<char>>, t: Vec<Vec<char>>) ->bool; {
    let n: usize = read();
    let h = n;
    let w = n;
    let mut vv: Vec<Vec<char>> = vec![vec!['0' as char; (0) as usize]; (h) as usize];
    for i in 0..h {
        let s: String = read();
        let vvv = s.chars().collect();
        vv[i] = vvv;
    }
    let s = vv.clone();
    let mut st = BTreeSet::new();
    let mut sh = UINF;
    let mut sw = UINF;
    let mut cnt1 = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                st.insert((i as i64, j as i64));
                sh = min(sh, i);
                sw = min(sw, j);
                cnt1 += 1;
            }
        }
    }
    let mut cnt2 = 0;

    for i in 0..h {
        let s: String = read();
        let vvv = s.chars().collect();
        vv[i] = vvv;
    }
    let t = vv.clone();

    let mut t1 = vec![vec!['.'; (w) as usize]; (h) as usize];
    let mut t2 = vec![vec!['.'; (w) as usize]; (h) as usize];
    let mut t3 = vec![vec!['.'; (w) as usize]; (h) as usize];
    let mut t4 = vec![vec!['.'; (w) as usize]; (h) as usize];
    for i in 0..n {
        for j in 0..n {
            t1[i][j] = t[i][j];
            t2[j][n - 1 - i] = t[i][j];
            t3[n - 1 - i][n - 1 - j] = t[i][j];
            t4[n - 1 - j][i] = t[i][j];
        }
    }
    let mut flg = false;

    //
    let t = t1;
    let mut x = 0;
    let mut th = UINF;
    let mut tw = UINF;
    for i in 0..n {
        for j in 0..n {
            if t[i][j] == '#' {
                th = min(th, i);
                tw = min(tw, j);
                cnt2 += 1;
            }
        }
    }
    let oh = sh as i64 - th as i64;
    let ow = sw as i64 - tw as i64;
    let mut fflg = true;
    for i in 0..n {
        for j in 0..n {
            if t[i][j] == '#' {
                // println!("{:?}", (i, j));
                let ssh = i as i64 + oh;
                let ssw = j as i64 + ow;
                if !st.contains(&(ssh, ssw)) {
                    fflg = false;
                }
            }
        }
    }
    if cnt1 != cnt2 {
        
        return false;
    }
    if fflg {
        flg = true;
    }
    // println!("{:?}", (oh, ow));
    // println!("{:?}", st.clone());
    //
    let t = t2;
    let mut x = 0;
    let mut th = UINF;
    let mut tw = UINF;
    for i in 0..n {
        for j in 0..n {
            if t[i][j] == '#' {
                th = min(th, i);
                tw = min(tw, j);
            }
        }
    }
    let oh = sh as i64 - th as i64;
    let ow = sw as i64 - tw as i64;
    let mut fflg = true;
    for i in 0..n {
        for j in 0..n {
            if t[i][j] == '#' {
                let ssh = i as i64 + oh;
                let ssw = j as i64 + ow;
                if !st.contains(&(ssh, ssw)) {
                    fflg = false;
                }
            }
        }
    }
    if fflg {
        flg = true;
    }
    //
    let t = t3;
    let mut x = 0;
    let mut th = UINF;
    let mut tw = UINF;
    for i in 0..n {
        for j in 0..n {
            if t[i][j] == '#' {
                th = min(th, i);
                tw = min(tw, j);
            }
        }
    }
    let oh = sh as i64 - th as i64;
    let ow = sw as i64 - tw as i64;
    let mut fflg = true;
    for i in 0..n {
        for j in 0..n {
            if t[i][j] == '#' {
                let ssh = i as i64 + oh;
                let ssw = j as i64 + ow;
                if !st.contains(&(ssh, ssw)) {
                    fflg = false;
                }
            }
        }
    }
    if fflg {
        flg = true;
    }

    //
    let t = t4;
    let mut x = 0;
    let mut th = UINF;
    let mut tw = UINF;
    for i in 0..n {
        for j in 0..n {
            if t[i][j] == '#' {
                th = min(th, i);
                tw = min(tw, j);
            }
        }
    }
    let oh = sh as i64 - th as i64;
    let ow = sw as i64 - tw as i64;
    let mut fflg = true;
    for i in 0..n {
        for j in 0..n {
            if t[i][j] == '#' {
                let ssh = i as i64 + oh;
                let ssw = j as i64 + ow;
                if !st.contains(&(ssh, ssw)) {
                    fflg = false;
                }
            }
        }
    }
    if fflg {
        flg = true;
    }
    if flg {
        return true;
    } else {
        return false;
    }
    return;
}
