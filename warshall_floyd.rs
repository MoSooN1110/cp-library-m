// source snippet: key=lib_warshall_floyd  prefix=lib_warshall_floyd

fn warshall_floyd(graph: &Vec<Vec<(usize, usize)>>) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut res: Vec<Vec<usize>> =
        vec![vec![INF as usize; (graph.len()) as usize]; (graph.len()) as usize];
    for i in 0..n {
        res[i][i] = 0;
    }
    for i in 0..n {
        for v in graph[i].iter() {
            res[i][v.0] = v.1;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                res[i][j] = min(res[i][j], res[i][k] + res[k][j]);
            }
        }
    }
    res
}
