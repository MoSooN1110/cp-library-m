// source snippet: key=lib_dijkstra_top2  prefix=lib_dijkstra_top2
// Your description.

fn dijkstra_top2(graph: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<[usize; 2]> {
	let n = graph.len();
	let mut dist = vec![[usize::MAX; 2]; n]; // dist[v][0] が最短距離, dist[v][1] が2番目
	let mut heap = BinaryHeap::new();

	dist[start][0] = 0;
	heap.push(Reverse((0, start)));

	while let Some(Reverse((cost, v))) = heap.pop() {
		// もし既にこのコストが2番目より大きい場合はスキップ
		if cost > dist[v][1] {
			continue;
		}
		for &(nv, w) in &graph[v] {
			let nc = cost + w;
			if nc < dist[nv][0] {
				// より短い距離を見つけた場合、今の最短を2番目に押し出す
				dist[nv][1] = dist[nv][0];
				dist[nv][0] = nc;
				heap.push(Reverse((nc, nv)));
			} else if dist[nv][0] < nc && nc < dist[nv][1] {
				// 2番目に短い距離を更新
				dist[nv][1] = nc;
				heap.push(Reverse((nc, nv)));
			}
		}
	}

	dist
}
