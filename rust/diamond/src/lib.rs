pub fn get_diamond(c: char) -> Vec<String> {
    let mut res = Vec::<String>::new();
    let n = (c as u32 - 'A' as u32) as u8 + 1;
    for i in 0..n {
        let s = if i == 0 {
            format!(
                "{}{}{}",
                " ".repeat((n - i - 1) as usize),
                (i + b'A') as char,
                " ".repeat((n - i - 1) as usize)
            )
        } else {
            format!(
                "{}{}{}{}{}",
                " ".repeat((n - i - 1) as usize),
                (i + b'A') as char,
                " ".repeat((i * 2 - 1) as usize),
                (i + b'A') as char,
                " ".repeat((n - i - 1) as usize)
            )
        };
        res.push(s);
    }
    for i in (0..n - 1).rev() {
        res.push(res[i as usize].clone());
    }
    res
}
