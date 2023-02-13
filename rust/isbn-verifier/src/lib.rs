pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: String = isbn.chars().filter(|c| *c != '-').collect();
    if !isbn.chars().enumerate().all(|(i, c)| c.is_digit(10) || (i == 9 && c == 'X')) {
        return false;
    }

    if isbn.len() != 10 {
        return false;
    }

    let mut sum = 0;
    for (i, c) in isbn.chars().enumerate() {
        let x = match c {
            c if c.is_digit(10) => c.to_digit(10).unwrap(),
            'X' => 10,
            _ => return false,
        };

        sum += x * (10 - i as u32);
    }

    sum % 11 == 0
}
