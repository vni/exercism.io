pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() {
        return Vec::new();
    }

    let rows = input.len();
    let cols = input[0].len();
    let mut res: Vec<(usize, usize)> = Vec::new();

    for r in 0..rows {
        'lbl: for c in 0..cols {
            let n = input[r][c];

            // greater or equal to every element in the row
            for c in 0..cols {
                if n < input[r][c] {
                    continue 'lbl;
                }
            }

            // less or equal to every element in the column
            for r in 0..rows {
                if n > input[r][c] {
                    continue 'lbl;
                }
            }

            res.push((r, c));
        }
    }
    res
}
