fn is_prime(n: u32) -> bool {
    if n % 2 == 0 {
        return false;
    }

    let sq = (n as f64).sqrt() as u32 + 1;

    for divisor in (3..=sq).step_by(2) {
        if n % divisor == 0 {
            return false;
        }
    }

    true
}

pub fn nth(mut n: u32) -> u32 {
    let mut current_prime: u32 = 2;
    let mut number: u32 = 3;

    while n > 0 {
        if is_prime(number) {
            current_prime = number;
            n -= 1;
        }

        number += 1;
    }

    current_prime
}
