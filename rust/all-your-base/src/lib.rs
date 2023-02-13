#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    use Error::*;
    if from_base < 2 {
        return Err(InvalidInputBase);
    }
    if to_base < 2 {
        return Err(InvalidOutputBase);
    }

    let mut n: u32 = 0;
    for d in number.iter() {
        if *d >= from_base {
            return Err(InvalidDigit(*d));
        }
        n *= from_base;
        n += *d;
    }

    let mut res = Vec::<u32>::new();
    while n > 0 {
        let d = n % to_base;
        n /= to_base;
        res.push(d);
    }
    res.reverse();

    if res.is_empty() {
        res.push(0);
    }
    Ok(res)
}
