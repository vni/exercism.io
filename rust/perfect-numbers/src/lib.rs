#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let sum: u64 = (1..=num / 2).filter(|i| num % i == 0).sum();

    match sum.cmp(&num) {
        std::cmp::Ordering::Less => Some(Classification::Deficient),
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
    }
}
