use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut r = BTreeMap::<char, i32>::new();
    for i in h {
        let (&score, chars) = i;
        for &c in chars {
            r.insert(c.to_ascii_lowercase(), score);
        }
    }
    r
}
