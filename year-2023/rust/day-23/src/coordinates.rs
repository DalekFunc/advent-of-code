use std::ops::RangeInclusive;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn neighbours(
        &self,
        row_bound: RangeInclusive<usize>,
        col_bound: RangeInclusive<usize>,
    ) -> Vec<Self> {
        let up = (self.row != *row_bound.start()).then(|| (self.row - 1, self.col).into());
        let down = (self.row != *row_bound.end()).then(|| (self.row + 1, self.col).into());
        let left = (self.col != *col_bound.start()).then(|| (self.row, self.col - 1).into());
        let right = (self.col != *col_bound.end()).then(|| (self.row, self.col + 1).into());

        [up, down, left, right].into_iter().flatten().collect()
    }
}

impl From<(usize, usize)> for Coord {
    fn from((row, col): (usize, usize)) -> Self {
        Self { row, col }
    }
}
