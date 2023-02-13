pub fn factors(n: u64) -> Vec<u64> {
    let mut fs: Vec<u64> = Vec::new();

    let mut left = n;
    let mut factor: u64 = 2;

    while left != 1 {
        while left % factor == 0 {
            fs.push(factor);
            left /= factor;
        }
        factor += 1;
    }

    fs
}
