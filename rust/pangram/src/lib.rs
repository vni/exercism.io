use std::collections::HashSet;
pub fn is_pangram(sentence: &str) -> bool {
    let hs: HashSet<char> = sentence.to_lowercase().chars().collect();
    !('a'..='z').any(|c| !hs.contains(&c))
}
