// source snippet: key=lib_backward_analysis_of_game_graph  prefix=lib_backward_analysis_of_game_graph

fn one_char_encoding(x: char) -> usize {
    let mut res = 0;
    if x >= 'a' {
        res = x as usize - 'a' as usize;
    } else if x >= 'A' {
        res = x as usize - 'A' as usize + 26;
    }
    res
}
fn three_char_encoding(a: char, b: char, c: char) -> usize {
    let mut res = 0;
    res += one_char_encoding(a) * 52 * 52;
    res += one_char_encoding(b) * 52;
    res += one_char_encoding(c);
    res
}

fn backward_analysis_on_game_graph() {
    //test https://atcoder.jp/contests/abc209/tasks/abc209_e
    let mut graph = vec![vec![(0 as usize, 0 as usize); (0) as usize]; (202020) as usize];
    let mut rgraph = vec![vec![(0 as usize, 0 as usize); (0) as usize]; (202020) as usize];

    let mut state = vec![UINF; 202020];
    let mut cnt = vec![0; 202020];
    let n: usize = read();
    let mut data = vec![];
    let mut st = VecDeque::new();
    for i in 0..n {
        let s: String = read();
        let mut v: Vec<char> = s.chars().collect();
        let s = v.len();
        let fv = three_char_encoding(v[0], v[1], v[2]);
        let tv = three_char_encoding(v[(s - 3)], v[s - 2], v[s - 1]);
        data.push(tv);
        cnt[fv] += 1;
        graph[tv].push((fv, 1));
        rgraph[tv].push((fv, 1));
    }
    let mut used = vec![0; 202020];
    for i in 0..202020 {
        if cnt[i] == 0 {
            state[i] = 0;
            st.push_back(i);
        }
    }

    while !st.is_empty() {
        let v = st.pop_front().unwrap();
        if used[v] == 1 {
            continue;
        }
        // println!("{:?}", v);

        used[v] = 1;
        for j in 0..rgraph[v].len() {
            let nv = rgraph[v][j].0;
            if state[nv] == UINF && true {
                //遷移条件を入れる
                cnt[nv] -= 1;
                if state[v] == 0 {
                    //子供に負けの状態が１つでもあるならばこのノードは勝ちの状態に確定
                    state[nv] = 1;
                    st.push_back(nv);
                } else if cnt[nv] == 0 {
                    //最後まで勝ちの状態にならなかったノードは負けの状態で確定
                    state[nv] = 0;
                    st.push_back(nv);
                }
            }
        }
    }
    for i in 0..n {
        // d!(state[data[i]]);
        p!(match state[data[i]] {
            UINF => "Draw",
            0 => "Takahashi",
            1 => "Aoki",
            _ => "x",
        })
    }

    return;
}
