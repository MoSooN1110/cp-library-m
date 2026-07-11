// source snippet: key=lib_bipartiate_graph_judgement  prefix=lib_bipartiate_graph_judgement

fn bipartiate_graph_judgement(
    v: usize,
    c: i64,
    color: &mut Vec<i64>,
    graph: &Vec<Vec<(usize, usize)>>,
) -> bool {
    color[v] = c;
    // println!("{:?}", c);
    for x in graph[v].iter() {
        let nv = &x.0;
        if color[*nv] != -1 {
            if color[*nv] == c {
                return false;
            };

            continue;
        }
        if !bipartiate_graph_judgement(*nv, 1 - c, color, &graph) {
            return false;
        }
    }

    true
}
