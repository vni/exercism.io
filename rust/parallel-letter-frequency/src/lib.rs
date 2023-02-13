use std::thread;
use std::collections::HashMap;

pub fn frequency(input: &'static [&str], worker_count: usize) -> HashMap<char, usize> {
    let mut h = HashMap::<char, usize>::new();
    let _worker_count = worker_count;

println!("input: '{:?}'", input);

    let mut handles = vec![];

    for v in input {
        let handle = thread::spawn(|| {
            let mut h = HashMap::<char, usize>::new();
            for c in v.chars() {
                *h.entry(c).or_insert(0) += 1;
            }
            h
        });
        handles.push(handle);
    }

    for h in handles.iter() {
        let res = h.join().unwrap();
//        h += res;
    }

    h
}
