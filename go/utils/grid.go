package utils

import "fmt"

type GridPos struct {
	Col int
	Row int
}

var MOVE_U = GridPos{0, 1}
var MOVE_D = GridPos{0, -1}
var MOVE_R = GridPos{1, 0}
var MOVE_L = GridPos{-1, 0}
var MOVE_UR = GridPos{1, 1}
var MOVE_UL = GridPos{-1, 1}
var MOVE_DL = GridPos{-1, -1}
var MOVE_DR = GridPos{1, -1}
var ALL_ORTHOGONAL = [...]GridPos{MOVE_U, MOVE_R, MOVE_D, MOVE_L}
var ALL_SURROUNDING = [...]GridPos{MOVE_U, MOVE_UR, MOVE_R, MOVE_DR, MOVE_D, MOVE_DL, MOVE_L, MOVE_UL}

// RowColToGridPos creates a GridPos from the Row/Col pair given.
func RowColToGridPos(col int, row int) GridPos {
	return GridPos{col, row}
}

// Distance returns the distance as tuple (dx, dy)
func (self *GridPos) Distance(other *GridPos) (int, int) {
	return self.Col - other.Col, self.Row - other.Row
}

// MoveBy moves this point by the given "delta" (given as a GridPos)
func (self *GridPos) MoveBy(other *GridPos) {
	self.Col += other.Col
	self.Row += other.Row
}

// Add creates another GridPos shifted by the given GridPos
func (self *GridPos) Add(delta GridPos) GridPos {
	return RowColToGridPos(self.Col+delta.Col, self.Row+delta.Row)
}

// GridPosFromArray creates a GridPos by converting a position into an array, assuming each Row is
// of the given width
func GridPosFromArray(pos uint, width uint) GridPos {
	col := pos % width
	row := (pos - col) / width
	return GridPos{Row: int(row), Col: int(col)}
}

// ToArrayPos will return the array position of the receiving GridPos within a square 0..width,
// 0..height.
// If the position cannot be fit inside the array, the error will flag it.
func (self *GridPos) ToArrayPos(width uint, height uint) (uint, error) {
	if self.Col < 0 || self.Col >= int(width) || self.Row < 0 || self.Row >= int(height) {
		return 0, fmt.Errorf("cannot convert to array pos within [%dx%x] %s", width, height, self)
	}
	return uint(self.Row)*width + uint(self.Col), nil
}

func (self GridPos) String() string {
	return fmt.Sprintf("(%d,%d)", self.Col, self.Row)
}

// InBounds checks if the point represented is within the bounds of a rectangle starting at (0,0)
// and of the given width/height (excluded)
func (self *GridPos) InBounds(width int, height int) bool {
	return self.Row >= 0 && self.Row < height && self.Col >= 0 && self.Col < width
}
