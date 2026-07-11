// source snippet: key=lib_dfs_order_gen  prefix=lib_dfs_order_gen

fn dfs_order_gen(
    //input
    v: usize,
    parent: usize,
    g: &Vec<Vec<(usize, usize)>>,
    //output
    order: &mut Vec<usize>,
    parents: &mut Vec<usize>,
) {
    for &(to, cost) in &g[v] {
        if to == parent {
            continue;
        }
        dfs_order_gen(to, v, g, order, parents);
    }
    order.push(v);
    parents[v] = parent;
    return;
}
