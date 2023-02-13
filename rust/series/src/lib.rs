pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec![];
    }

    let mut result = Vec::<String>::with_capacity(digits.len() - len + 1);
    let mut start_pos = 0usize;
    while start_pos + len <= digits.len() {
        result.push(digits[start_pos..start_pos + len].to_string());
        start_pos += 1;
    }

    result
}
