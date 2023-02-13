use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;

thread_local!(
    static USED_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::<String>::new())
);

pub struct Robot(String);

impl Robot {
    fn generate_new_name() -> String {
        let mut res = String::new();
        let mut rng = rand::thread_rng();

        loop {
            for _ in 0..2 {
                let l: u8 = rng.gen_range(0..26) + b'A';
                res.push(l as char);
            }

            for _ in 0..3 {
                let l: u8 = rng.gen_range(0..10) + b'0';
                res.push(l as char);
            }

            if USED_NAMES.with(|set| set.borrow().contains(&res)) {
                res = String::new();
            } else {
                USED_NAMES.with(|set| set.borrow_mut().insert(res.clone()));
                break;
            }
        }
        res
    }

    pub fn new() -> Self {
        Robot(Self::generate_new_name())
    }

    pub fn name(&self) -> &str {
        self.0.as_str()
    }

    pub fn reset_name(&mut self) {
        USED_NAMES.with(|set| set.borrow_mut().remove(&self.0));
        self.0 = Self::generate_new_name();
    }
}
