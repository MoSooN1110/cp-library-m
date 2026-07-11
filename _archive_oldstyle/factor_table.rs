// source snippet: key=factor_table  prefix=factor_table
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
pub fn factor_table(max_n: usize) -> Vec<usize> {
    let mut res = vec![0; max_n + 1];
    for i in 2..max_n + 1 {
        if res[i] == 0 {
            let mut j = i;
            while j <= max_n {
                res[j] = i;
                j += i;
            }
        }
    }
    res
}
