pub fn encode(source: &str) -> String {
    let mut res = String::new();
    if source.is_empty() {
        return res;
    }

    let mut source_iter = source.chars();
    let mut last_c: char = source_iter.next().unwrap();
    let mut counter = 1;
    for c in source_iter {
        if c == last_c {
            counter += 1;
        } else {
            if counter > 1 {
                res.push_str(&format!("{}{}", counter, last_c));
                counter = 1;
            } else {
                res.push(last_c);
            }
            last_c = c;
        }
    }

    if counter > 1 {
        res.push_str(&format!("{}{}", counter, last_c));
    } else {
        res.push(last_c);
    }

    res
}

pub fn decode(source: &str) -> String {
    let mut res = String::new();

    let mut repeat = 0;
    for c in source.chars() {
        if c.is_digit(10) {
            let d = c.to_digit(10).unwrap();
            repeat *= 10;
            repeat += d;
        } else {
            if repeat != 0 {
                res.push_str(&c.to_string().repeat(repeat as usize));
            } else {
                res.push(c);
            }
            repeat = 0;
        }
    }

    res
}
