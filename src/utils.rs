
pub fn split_n_chars(s: &str, n: usize) -> Vec<&str> {
    let mut result = vec![];

    let mut i = 0;
    while i < s.len() {
        result.push(&s[i..i + n]);
        i += n;
    }

    result
}
