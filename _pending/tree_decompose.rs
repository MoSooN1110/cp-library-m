// source snippet: key=lib_tree_decompose  prefix=lib_tree_decompose

fn tree_decompose_edge(
    graph: &Vec<Vec<(usize, usize)>>,
    vertex1: usize,
    vertex2: usize,
) -> Vec<Vec<Vec<(usize, usize)>>> {
    let mut res = vec![vec![vec![]; graph.len()]; 0];
    let mut root = vec![];
    let n = graph.len();
    root.push(vertex1);
    root.push(vertex2);
    let mut bt = BTreeMap::new();
    for i in 0..n {
        for j in 0..graph[i].len() {
            let v = i;
            let nv = graph[i][j].0;
            bt.insert((v, nv), graph[i][j].1);
        }
    }
    let mut visited = vec![0; n];
    visited[vertex1] = 1;
    visited[vertex2] = 1;
    for i in root {
        let mut es = vec![];
        let mut st = VecDeque::new();
        es.push((vertex1, i));

        st.push_front(i);
        while !st.is_empty() {
            let v = st.pop_front().unwrap();

            visited[v] = 1;
            for j in 0..graph[v].len() {
                let nv = graph[v][j];
                if visited[nv.0] == 0 {
                    st.push_front(nv.0);
                    es.push((v, nv.0));
                }
            }
        }
        let mut g = vec![vec![]; n];
        for i in es.iter() {
            g[i.0].push((i.1, *bt.get(&(i.0, i.1)).unwrap()));
            g[i.1].push((i.0, *bt.get(&(i.0, i.1)).unwrap()));
        }
        res.push(g);
    }

    res
}
fn tree_decompose_vertex(
    graph: &Vec<Vec<(usize, usize)>>,
    vertex: usize,
) -> Vec<Vec<Vec<(usize, usize)>>> {
    let mut res = vec![vec![vec![]; graph.len()]; 0];
    let mut root = vec![];
    let n = graph.len();
    for i in graph[vertex].iter() {
        root.push(i.0);
    }
    println!("{:?}", root);
    let mut bt = BTreeMap::new();
    for i in 0..n {
        for j in 0..graph[i].len() {
            let v = i;
            let nv = graph[i][j].0;
            bt.insert((v, nv), graph[i][j].1);
        }
    }
    let mut visited = vec![0; n];
    visited[vertex] = 1;
    for i in root {
        let mut es = vec![];
        let mut st = VecDeque::new();
        es.push((vertex, i));

        st.push_front(i);
        while !st.is_empty() {
            let v = st.pop_front().unwrap();
            if visited[v] == 1 {
                continue;
            }

            visited[v] = 1;
            for j in 0..graph[v].len() {
                let nv = graph[v][j];
                if visited[nv.0] == 0 {
                    st.push_front(nv.0);
                    es.push((v, nv.0));
                }
            }
        }
        let mut g = vec![vec![]; n];
        for i in es.iter() {
            g[i.0].push((i.1, *bt.get(&(i.0, i.1)).unwrap()));
            g[i.1].push((i.0, *bt.get(&(i.0, i.1)).unwrap()));
        }
        res.push(g);
    }

    res
}
//test
#[test]
fn test_tree_decompose_vertex() {
    let n: usize = 6;
    let mut graph: Vec<Vec<(usize, usize)>> = vec![
        vec![(1, 1)],
        vec![(0, 1), (2, 1)],
        vec![(1, 1), (3, 1), (5, 1)],
        vec![(2, 1), (4, 1)],
        vec![(3, 1)],
        vec![(2, 1)],
    ];

    d!(graph);
    for x in tree_decompose_vertex(&graph, 2).iter() {
        d!(x);
    }
    return;
}
