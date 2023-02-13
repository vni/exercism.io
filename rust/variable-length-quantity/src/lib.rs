#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

fn from_u32(mut v: u32) -> Vec<u8> {
    let mut r = Vec::<u8>::new();
    let mut mask: u8 = 0x00;

    loop {
        r.push((v & 0x7F) as u8 | mask);
        v >>= 7;
        mask = 0x80;

        if v == 0 {
            break;
        }
    }

    r.reverse();
    r
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut res = Vec::<u8>::new();
    for v in values {
        let bs = from_u32(*v);
        for b in bs {
            res.push(b);
        }
    }
    res
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if !bytes.is_empty() && bytes[bytes.len() - 1] & 0x80 == 0x80 {
        return Err(Error::IncompleteNumber);
    }

    let mut res = Vec::<u32>::new();
    let mut val = 0;
    let high7 = 0xfe << 25;

    for b in bytes {
        if (val & high7) != 0 {
            return Err(Error::Overflow);
        }

        val <<= 7;
        val |= (b & 0x7f) as u32;
        if b & 0x80 == 0 {
            res.push(val);
            val = 0;
        }
    }

    Ok(res)
}
