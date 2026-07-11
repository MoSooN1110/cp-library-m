// source snippet: key=lib_digit_sum  prefix=lib_digit_sum

fn digit_sum(mut x: i64) -> i64 {
    let mut res = 0;
    while x > 0 {
        res += x % 10;
        x /= 10;
    }
    res
}
