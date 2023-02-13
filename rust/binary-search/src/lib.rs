pub fn find<T, AR>(array: AR, key: T) -> Option<usize>
where
    AR: AsRef<[T]>,
    T: PartialEq + PartialOrd,
{
    let mut a = array.as_ref();
    if a.is_empty() {
        return None;
    }

    let mut idx = 0;
    while a.len() != 1 {
        let m = a.len() / 2;
        if a[m] > key {
            a = &a[0..m];
        } else {
            a = &a[m..a.len()];
            idx += m;
        }
    }

    if a[0] == key {
        Some(idx)
    } else {
        None
    }
}
