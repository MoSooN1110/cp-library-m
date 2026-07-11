// source snippet: key=lib_vector_accumulation  prefix=lib_vector_accumulation

#[allow(dead_code)]
fn vector_accumulation(vec: &Vec<i64>) -> Vec<i64> {
    let mut res = Vec::new();
    let size = vec.len();
    res.push(0);
    for i in 0..size {
        res.push(vec[i]);
    }
    for i in 0..size {
        res[i + 1] += res[i];
    }
    res
}
