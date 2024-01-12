#[derive(Debug)]
pub struct HighScores<'a> {
    all_scores: &'a [u32],
}

impl HighScores<'_> {
    pub fn new(scores: &[u32]) -> HighScores {
        HighScores {
            all_scores : scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.all_scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.all_scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut scores = Vec::from(self.all_scores);
        scores.sort(); //self.scores.iter().max().cloned()
        return scores.last().copied();
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if !self.all_scores.is_empty() {
            let mut scores = Vec::from(self.all_scores);
            scores.sort_by(|a, b| b.cmp(a));
            return scores[..=std::cmp::min(scores.len() - 1, 2)].to_vec();  // res_vec.truncate(3);
        } 

        return vec![];
    }
}
