fn count_bombs(f: &[&str], r: usize, c: usize) -> char {
    if f[r].chars().nth(c) == Some('*') {
        return '*';
    }

    let mut count = 0;

    if r > 0 {
        if c > 0 {
            if f[r - 1].chars().nth(c - 1) == Some('*') {
                count += 1;
            }
        }
        if f[r - 1].chars().nth(c) == Some('*') {
            count += 1;
        }
        if c < f[r - 1].len() - 1 {
            if f[r - 1].chars().nth(c + 1) == Some('*') {
                count += 1;
            }
        }
    }

    if c > 0 {
        if f[r].chars().nth(c - 1) == Some('*') {
            count += 1;
        }
    }

    if c < f[r].len() - 1 {
        if f[r].chars().nth(c + 1) == Some('*') {
            count += 1;
        }
    }

    if r < f.len() - 1 {
        if c > 0 {
            if f[r + 1].chars().nth(c - 1) == Some('*') {
                count += 1;
            }
        }
        if f[r + 1].chars().nth(c) == Some('*') {
            count += 1;
        }
        if c < f[r + 1].len() - 1 {
            if f[r + 1].chars().nth(c + 1) == Some('*') {
                count += 1;
            }
        }
    }

    if count > 0 {
        (count as u8 + b'0') as char
    } else {
        ' '
    }
}

pub fn annotate(mfield: &[&str]) -> Vec<String> {
    let mut res = Vec::<String>::with_capacity(mfield.len());

    if mfield.is_empty() {
        return res;
    }

    for r in 0..mfield.len() {
        let mut row = String::with_capacity(mfield[r].len());
        for c in 0..mfield[r].len() {
            row.push(count_bombs(&mfield, r, c));
        }
        res.push(row);
    }

    res
}
