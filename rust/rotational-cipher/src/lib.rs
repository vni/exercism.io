pub fn rotate(input: &str, mut key: i8) -> String {
    if key < 0 {
        key += 26;
    }

    input
        .chars()
        .map(|c| match c {
            c if c.is_ascii_lowercase() => (((((c as u8) - b'a') + key as u8) % 26) + b'a') as char,
            c if c.is_ascii_uppercase() => (((((c as u8) - b'A') + key as u8) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}
