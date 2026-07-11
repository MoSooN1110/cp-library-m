// source snippet: key=lib_dijkstra  prefix=lib_dijkstra

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<usize> {
    let mut dist = vec![INF as usize; graph.len()];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0 as usize, start)));
    dist[start] = 0;
    while let Some(Reverse(x)) = heap.pop() {
        let cost = x.0;
        let v = x.1;
        if cost > dist[v] {
            continue;
        }
        for edge in &graph[v] {
            let nc = cost + edge.1;
            let nv = edge.0;
            if nc < dist[nv] {
                heap.push(Reverse((nc, nv)));
                dist[nv] = nc;
            }
        }
    }
    return dist;
}
