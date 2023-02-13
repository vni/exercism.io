pub fn reply(message: &str) -> &str {
    let all_capitals = |msg: &str| {
        msg.chars()
            .all(|c| !c.is_alphabetic() || (c.is_alphabetic() && c.is_uppercase()))
            && msg.chars().any(|c| c.is_uppercase())
    };

    match message.trim() {
        x if all_capitals(x) && x.ends_with('?') => "Calm down, I know what I'm doing!",
        x if all_capitals(x) => "Whoa, chill out!",
        x if x.ends_with('?') => "Sure.",
        x if x.is_empty() => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
