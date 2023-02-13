fn is_key_valid(key: &str) -> bool {
    !key.is_empty() && key.chars().all(|c| c.is_ascii_lowercase())
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_key_valid(key) {
        return None;
    }

    let key: Vec<u8> = key.chars().map(|c| c as u8 - b'a').collect();
    let mut key_iter = key.iter().cycle();

    Some(s.chars().map(|c| {
        let shift = key_iter.next().unwrap();
        match c {
            c if c.is_ascii_lowercase() => (((c as u8 - b'a' + shift) % 26) + b'a') as char,
            c if c.is_ascii_uppercase() => (((c as u8 - b'A' + shift) % 26) + b'A') as char,
            _ => c,
        }
    }).collect::<String>())
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_key_valid(key) {
        return None;
    }

    let key: Vec<u8> = key.chars().map(|c| c as u8 - b'a' ).collect();
    let mut key_iter = key.iter().cycle();

    Some(s.chars().map(|c| {
        let shift = key_iter.next().unwrap();
        match c {
            c if c.is_ascii_lowercase() => (((c as u8 - b'a' + 26 - shift) % 26) + b'a') as char,
            c if c.is_ascii_uppercase() => (((c as u8 - b'A' + 26 - shift) % 26) + b'A') as char,
            _ => c,
        }
    }).collect::<String>())
}

pub fn encode_random(s: &str) -> (String, String) {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let len = rng.gen_range(100, 1000);
    let mut key = String::with_capacity(len);

    for _ in 0..len {
        let k = rng.gen_range(b'a', b'z');
        key.push(k as char);
    }

    let encoded = encode(&key[..], s).unwrap();
    (key, encoded)
}
