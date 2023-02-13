pub fn encode(plain: &str) -> String {
    plain.to_lowercase().chars().filter(|c| c.is_ascii_alphanumeric())
        .map(|c| match c.is_ascii_alphabetic() {
            true => (b'z' - (c as u8) + b'a') as char,
            _ => c,
        })
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 5 == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
}

pub fn decode(cipher: &str) -> String {
    cipher.chars().filter(|c| c.is_ascii_alphanumeric())
        .map(|c| match c.is_ascii_lowercase() {
            true => (b'z' - (c as u8) + b'a') as char,
            _ => c,
        })
        .collect::<String>()
}
