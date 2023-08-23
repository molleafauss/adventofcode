use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) struct GridPos {
    pub(crate) x: i64,
    pub(crate) y: i64,
}

pub(crate) const MOVE_U : GridPos = GridPos::of(0, 1);
pub(crate) const MOVE_D : GridPos = GridPos::of(0, -1);
pub(crate) const MOVE_R : GridPos = GridPos::of(1, 0);
pub(crate) const MOVE_L : GridPos = GridPos::of(-1, 0);
pub(crate) const MOVE_UR : GridPos = GridPos::of(1, 1);
pub(crate) const MOVE_UL : GridPos = GridPos::of(-1, 1);
pub(crate) const MOVE_DL : GridPos = GridPos::of(-1, -1);
pub(crate) const MOVE_DR : GridPos = GridPos::of(1, -1);

impl GridPos {
    /// Creates a position from the x/y pair given.
    pub(crate) const fn of(x: i64, y: i64) -> GridPos {
        GridPos {x, y}
    }

    /// returns the distance as tuple (dx, dy)
    pub(crate) fn distance(&self, other: &GridPos) -> (i64, i64) {
        (self.x - other.x, self.y - other.y)
    }

    /// moves this point by the given "delta" (given as a GridPos)
    pub(crate) fn move_by(&mut self, other: &GridPos) {
        self.x += other.x;
        self.y += other.y;
    }

    /// creates another GridPos shifted by the given GridPos
    pub(crate) fn add(&self, delta: &GridPos) -> GridPos {
        GridPos::of(self.x + delta.x, self.y + delta.y)
    }
}

impl Display for GridPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
