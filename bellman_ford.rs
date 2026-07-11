// source snippet: key=lib_bellman_ford  prefix=lib_bellman_ford

fn bellman_ford(es: &Vec<(usize, usize, i64)>, n: usize, s: usize) -> Vec<i64> {
    let mut dist: Vec<i64> = vec![INF; n];
    let mut none: Vec<i64> = vec![0; 0];
    dist[s] = 0;
    for i in 0..n - 1 {
        for ee in es.iter() {
            let e = *ee;
            if (dist[e.0] == INF) {
                continue;
            }
            dist[e.1] = min(dist[e.1], dist[e.0] + e.2);
        }
    }
    let mut d1 = dist.clone();
    for i in 0..n + 1 {
        for ee in es.iter() {
            let e = *ee;
            if (dist[e.0] == INF) {
                continue;
            }
            if dist[e.1] > dist[e.0] + e.2 {
                dist[e.1] = min(dist[e.1], dist[e.0] + e.2);
            }
        }
    }
    if dist[n - 1] < d1[n - 1] {
        return none;
    }

    return d1;
}

fn bellmanford_interface(graph: &Vec<Vec<(usize, i64)>>, start: usize) -> Vec<i64> {
    let mut n = graph.len();
    let mut es = vec![(0, 0, 0); 0];
    for i in 0..n {
        for j in 0..graph[i].len() {
            es.push((i, graph[i][j].0, graph[i][j].1));
        }
    }
    let mut d = bellman_ford(&es, n, start);
    return d;
}
