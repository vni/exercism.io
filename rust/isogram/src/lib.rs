use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    let mut hs: HashSet<char> = HashSet::new();
    for c in candidate.to_lowercase().chars().filter(|c| !c.is_whitespace() && *c != '-') {
        if hs.contains(&c) { return false; }
        hs.insert(c);
    }
    true
}
