pub fn number(user_number: &str) -> Option<String> {
    if user_number.chars().any(|c| {
        !c.is_ascii_digit() && c != ' ' && c != '-' && c != '(' && c != ')' && c != '.' && c != '+'
    }) {
        return None;
    }

    let mut num = user_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<char>>();

    if num.len() < 10 || num.len() > 11 {
        return None;
    }

    // check international country code
    if num.len() == 11 {
        if num[0] != '1' {
            return None;
        }
        num.remove(0);
    }

    // first digit of area code and exchange code should be >= 2 && <= 9
    if num[0] < '2' || num[3] < '2' {
        return None;
    }

    Some(num.into_iter().collect::<String>())
}
