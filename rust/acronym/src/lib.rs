fn split_by_this(c: char) -> bool {
    !c.is_alphabetic() && c != '\''
}

fn abbreviate_word(s: &str) -> String {
    let mut result = String::new();
    if s.is_empty() {
        return result;
    }
    let mut chs = s.chars();
    result.push(chs.next().unwrap().to_uppercase().next().unwrap());
    for c in chs {
        if c.is_uppercase() {
            result.push(c);
        }
    }
    if result.len() == s.len() {
        // all uppercase
        result = result[0..1].to_string();
    }
    result
}

pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    for w in phrase.split(split_by_this) {
        result.push_str(abbreviate_word(w).as_str());
    }
    result
}
