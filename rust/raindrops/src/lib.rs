pub fn raindrops(n: u32) -> String {
    // convert number to string if it is not divisible by 3, 5 or 7
    let mut stringify_me = true;
    let mut answer = String::with_capacity(16);

    if n % 3 == 0 {
        answer.push_str("Pling");
        stringify_me = false;
    }

    if n % 5 == 0 {
        answer.push_str("Plang");
        stringify_me = false;
    }

    if n % 7 == 0 {
        answer.push_str("Plong");
        stringify_me = false;
    }

    if stringify_me {
        answer = n.to_string();
    }

    answer
}
