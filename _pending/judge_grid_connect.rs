// source snippet: key=]  prefix=lib_judge_grid_connect
// lib_judge_grid_connect

fn judge_grid_connect_dfs(
    x: i64,
    y: i64,
    grid: &Vec<Vec<char>>,
    data: &mut Vec<Vec<usize>>,
    cnt: usize,
) {
    let h = data.len();
    let w = data[0].len();
    if x < 0 || y < 0 || x >= h as i64 || y >= w as i64 {
        return;
    }

    if data[x as usize][y as usize] != 0 {
        return;
    }
    let x = x as usize;
    let y = y as usize;
    if grid[x][y] != 'o' {
        return;
    }
    data[x][y] = cnt;
    judge_grid_connect_dfs(x as i64 - 1, y as i64, grid, data, cnt);
    judge_grid_connect_dfs(x as i64 + 1, y as i64, grid, data, cnt);
    judge_grid_connect_dfs(x as i64, y as i64 - 1, grid, data, cnt);
    judge_grid_connect_dfs(x as i64, y as i64 + 1, grid, data, cnt);
    return;
}
fn judge_grid_connect(grid: &Vec<Vec<char>>) -> bool {
    let mut flg = true;
    let mut data = vec![vec![0 as usize; (grid[0].len()) as usize]; (grid.len()) as usize];
    let mut cnt = 1;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'o' && data[i][j] == 0 {
                judge_grid_connect_dfs(i as i64, j as i64, grid, &mut data, cnt);
                cnt += 1;
            }
        }
    }
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if data[i][j] > 1 {
                flg = false;
            }
        }
    }
    // d!(data);
    flg
}
