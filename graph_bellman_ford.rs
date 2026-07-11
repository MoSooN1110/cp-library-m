// source snippet: key=lib_graph_bellman_ford  prefix=lib_graph_bellman_ford

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

    for ee in es.iter() {
        let e = *ee;
        if (dist[e.0] == INF) {
            continue;
        }
        if dist[e.1] > dist[e.0] + e.2 {
            return none;
        }
    }

    return dist;
}
