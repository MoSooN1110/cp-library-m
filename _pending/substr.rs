// source snippet: key=lib_substr  prefix=lib_substr

fn sub_string(instr: &String, index_b: usize, index_e: usize) -> String {
    let mut str1 = "".to_string();
    for (ii, c) in instr.chars().enumerate() {
        if ii >= index_b && ii < index_e {
            str1.push(c);
        }
    }
    str1
}
