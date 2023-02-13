pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let mut res = String::with_capacity(text.len());
        let n = self.0 as usize;
        let width = 2 * n - 2;

        let str = text.chars().collect::<Vec<char>>();
        for r in 0..n {
            let mut idx1 = r;
            let mut idx2 = width - idx1;
            let skip: bool = idx1 == idx2 || idx2 == width;

            loop {
                if idx1 >= str.len() {
                    break;
                }
                res.push(str[idx1]);
                idx1 += width;

                if !skip {
                    if idx2 >= str.len() {
                        break;
                    }
                    res.push(str[idx2]);
                    idx2 += width;
                }
            }
        }

        res
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut it = cipher.chars();
        let mut v = vec![' '; cipher.len()];
        let n = self.0 as usize;
        let width = 2 * n - 2;

        for r in 0..n {
            let mut idx1: usize = r;
            let mut idx2: usize = width - idx1;
            let skip: bool = idx1 == idx2 || idx2 == width;

            loop {
                if idx1 >= v.len() {
                    break;
                }
                v[idx1] = it.next().unwrap();
                idx1 += width;

                if !skip {
                    if idx2 >= v.len() {
                        break;
                    }
                    v[idx2] = it.next().unwrap();
                    idx2 += width;
                }
            }
        }
        v.iter().collect()
    }
}
