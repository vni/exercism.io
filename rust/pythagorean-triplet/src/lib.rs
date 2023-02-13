use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut res: HashSet<[u32; 3]> = HashSet::new();
    for a in 1..=sum / 3 {
        let a2 = a * a;
        for b in a + 1..=sum / 2 {
            let c = sum - a - b;

            if c < a || c < b {
                break;
            }

            if a2 + b * b == c * c {
                res.insert([a, b, c]);
            }
        }
    }

    res
}
