use std::collections::HashSet;
use std::collections::HashMap;

fn make_heatmap(s: &String) -> HashMap<char, usize> {
    let mut res = HashMap::<char, usize>::new();
    for c in s.chars() {
        *res.entry(c).or_insert(0) += 1;
    }
    res
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut res: HashSet<&str> = HashSet::new();

    let word_lowercase = word.to_lowercase();
    let w1 = make_heatmap(&word_lowercase);

    for possible in possible_anagrams.iter() {
        let possible_lowercase = possible.to_lowercase();
        let w2 = make_heatmap(&possible_lowercase);

        if w1 == w2 && word_lowercase != possible_lowercase {
            res.insert(possible);
        }
    }
    res
}
