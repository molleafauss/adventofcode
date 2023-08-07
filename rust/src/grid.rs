use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Hash, Clone)]
pub(crate) struct GridPos {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

pub(crate) const MOVE_U : GridPos = GridPos { x: 0, y: 1 };
pub(crate) const MOVE_D : GridPos = GridPos { x: 0, y: -1 };
pub(crate) const MOVE_R : GridPos = GridPos { x: 1, y: 0 };
pub(crate) const MOVE_L : GridPos = GridPos { x: -1, y: 0 };

impl GridPos {
    /// Creates a position from the x/y pair given.
    pub(crate) fn of(x: i32, y: i32) -> GridPos {
        GridPos {x, y}
    }

    /// returns the distance as tuple (dx, dy)
    pub(crate) fn distance(&self, other: &GridPos) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    /// moves this point by the given "delta" (given as a GridPos)
    pub(crate) fn move_by(&mut self, other: &GridPos) {
        self.x += other.x;
        self.y += other.y;
    }

    /// creates another GridPos shifted by the given GridPos
    pub(crate) fn add(&self, delta: &GridPos) -> GridPos {
        GridPos::of(
            x: self.x + delta.x,
            y: self.y + delta.y,
        )
    }
}

impl Display for GridPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
