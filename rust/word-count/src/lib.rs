use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut ws = HashMap::<String, u32>::new();

    let splitted =
        words.split(|c: char| !(c.is_alphabetic() || c.is_digit(10) || c == '-' || c == '\''));

    for mut w in splitted {
        if w.is_empty() {
            continue;
        }

        w = w.trim_matches(|c: char| c.is_ascii_punctuation());
        *ws.entry(w.to_lowercase()).or_insert(0) += 1;
    }

    ws
}
