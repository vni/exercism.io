pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = Vec::<Vec<u32>>::with_capacity(row_count as usize);
        for i in 0..row_count as usize {
            let mut row = Vec::<u32>::with_capacity(i + 1);
            for j in 0..=i as usize {
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    row.push(rows[i - 1][j - 1] + rows[i - 1][j]);
                }
            }
            rows.push(row);
        }
        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
