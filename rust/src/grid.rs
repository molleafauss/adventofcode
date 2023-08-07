use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Hash, Clone)]
pub(crate) struct GridPos {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl GridPos {
    pub(crate) fn of(x: i32, y: i32) -> GridPos {
        GridPos {x, y}
    }

    pub(crate) fn distance(&self, other: &GridPos) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    pub(crate) fn move_by(&mut self, other: &GridPos) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Display for GridPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
