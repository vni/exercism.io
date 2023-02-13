pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();
    let mut sieve: Vec<bool> = vec![true; (upper_bound + 1) as usize];

    for i in 2..sieve.len() {
        if sieve[i] == true {
            for j in (2 * i..sieve.len()).step_by(i) {
                sieve[j] = false;
            }

            v.push(i as u64);
        }
    }

    v
}
