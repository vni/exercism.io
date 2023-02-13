#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if let Some(x) = self.scores.last() {
            return Some(*x);
        }
        None
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None;
        }

        let mut max = 0;
        for v in self.scores {
            if *v > max {
                max = *v;
            }
        }

        Some(max)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v: Vec<u32> = self.scores.to_vec();
        v.sort_by(|a, b| b.partial_cmp(a).unwrap());
        v.truncate(3);
        v
    }
}
