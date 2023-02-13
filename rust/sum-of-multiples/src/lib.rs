pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum: u32 = 0;
    let mut used = Vec::<u32>::new();
    for num in 1..limit {
        for f in factors {
            if *f == 0 {
                continue;
            }

            if num % f == 0 {
                if !used.iter().any(|&v| v == num) {
                    sum += num;
                    used.push(num);
                }
            }
        }
    }

    sum
}
