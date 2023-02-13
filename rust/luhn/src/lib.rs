pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|x| !x.is_digit(10) && !x.is_whitespace()) {
        return false;
    }

    let mut v: Vec<u8> = code
        .chars()
        .filter(|x| x.is_digit(10))
        .map(|x| x.to_digit(10).unwrap() as u8)
        .rev()
        .collect();

    if v.len() <= 1 {
        return false;
    }

    for v in v.iter_mut().skip(1).step_by(2) {
        *v = *v + *v;
        if *v > 9 {
            *v -= 9;
        }
    }

    v.iter().sum::<u8>() % 10 == 0
}
