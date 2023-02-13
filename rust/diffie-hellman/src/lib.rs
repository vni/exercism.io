pub fn private_key(p: u64) -> u64 {
    use rand::Rng;
    rand::thread_rng().gen_range(2..p)
}

// Right-to-left binary method from https://en.wikipedia.org/wiki/Modular_exponentiation
fn modular_pow(mut b: u64, mut e: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }

    let big_multiply_and_mod =
        |a: u64, b: u64, m: u64| -> u64 { (a as u128 * b as u128 % m as u128) as u64 };

    let mut result: u64 = 1;
    b %= m;
    while e > 0 {
        if e % 2 == 1 {
            result = big_multiply_and_mod(result, b, m);
        }
        e >>= 1;
        b = big_multiply_and_mod(b, b, m);
    }

    result
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}
