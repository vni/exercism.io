pub fn is_armstrong_number(num: u32) -> bool {
    let as_string = num.to_string();
    let power = as_string.len() as u32;

    as_string
        .chars()
        .map(|c| u32::pow(c.to_digit(10).unwrap(), power))
        .sum::<u32>()
        == num
}
