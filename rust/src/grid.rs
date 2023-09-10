use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct GridPos {
    pub col: i64,
    pub row: i64,
}

pub const MOVE_U : GridPos = GridPos::of(0, 1);
pub const MOVE_D : GridPos = GridPos::of(0, -1);
pub const MOVE_R : GridPos = GridPos::of(1, 0);
pub const MOVE_L : GridPos = GridPos::of(-1, 0);
pub const MOVE_UR : GridPos = GridPos::of(1, 1);
pub const MOVE_UL : GridPos = GridPos::of(-1, 1);
pub const MOVE_DL : GridPos = GridPos::of(-1, -1);
pub const MOVE_DR : GridPos = GridPos::of(1, -1);

impl GridPos {
    /// Creates a position from the x/y pair given.
    pub const fn of(col: i64, row: i64) -> GridPos {
        GridPos { col, row }
    }

    /// returns the distance as tuple (dx, dy)
    pub fn distance(&self, other: &GridPos) -> (i64, i64) {
        (self.col - other.col, self.row - other.row)
    }

    /// moves this point by the given "delta" (given as a GridPos)
    pub fn move_by(&mut self, other: &GridPos) {
        self.col += other.col;
        self.row += other.row;
    }

    /// creates another GridPos shifted by the given GridPos
    pub fn add(&self, delta: &GridPos) -> GridPos {
        GridPos::of(self.col + delta.col, self.row + delta.row)
    }

    /// creates a GridPos wrapping around a linear position on a map of given width/height
    pub fn from_linear(pos: usize, width: usize) -> GridPos {
        let col = pos % width;
        let row = (pos - col) / width;
        GridPos { row: row as i64, col: col as i64 }
    }

    /// if GridPos is within a square 0..width, 0..height, return an index into a linear array, else
    /// return none.
    pub fn to_linear(&self, width: usize, height: usize) -> Option<i64> {
        if self.col < 0 || self.col >= width as i64 || self.row < 0 || self.row >= height as i64 {
            return None
        }
        Some(self.row * width as i64 + self.col)
    }
}

impl Display for GridPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.col, self.row)
    }
}

#[cfg(test)]
mod test {
    use crate::grid::GridPos;

    #[test]
    fn test_from_linear() {
        // pos => (row, col) on a 10, 10 matrix
        let data = [(0, (0, 0)), (10, (1, 0)), (23, (2, 3))];
        data.iter().for_each(|(pos, coord)| {
            let result = GridPos::from_linear(*pos, 10);
            assert_eq!(result.row, coord.0);
            assert_eq!(result.col, coord.1);
        });
    }
}