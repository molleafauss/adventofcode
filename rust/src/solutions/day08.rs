// What did I learn?
// I ended up using a one-dimension vector to store the matrix, as the two dimensional was a bit more
// complex to handle ownerships and mutability.
// As in the python solution, the code could reuse some factoring...

use crate::Solver;

pub(crate) struct Solution {
    matrix: Vec<Tree>,
    width: usize,
    height: usize
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            matrix: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn tree_at(&mut self, row: usize, col: usize) -> &mut Tree {
        let pos = row * self.width + col;
        &mut self.matrix[pos]
    }

    fn check_column(&mut self, col: usize) -> usize {
        let mut visible = 0;
        let mut tree_size = self.tree_at(0, col).size;
        // view from top
        (1..self.height - 1).for_each(|row| {
            let tree = self.tree_at(row, col);
            if tree.size > tree_size {
                visible += if tree.visible { 0 } else { 1 };
                tree.visible = true;
                tree_size = tree.size;
            }
        });
        // view from top
        tree_size = self.tree_at(self.height - 1, col).size;
        (1..self.height - 1).rev().for_each(|row| {
            let tree = self.tree_at(row, col);
            if tree.size > tree_size {
                visible += if tree.visible { 0 } else { 1 };
                tree.visible = true;
                tree_size = tree.size;
            }
        });
        visible
    }

    fn check_row(&mut self, row: usize) -> usize {
        let mut visible = 0;
        let mut tree_size = self.tree_at(row, 0).size;
        // view from left
        (1..self.width - 1).for_each(|col| {
            let tree = self.tree_at(row, col);
            if tree.size > tree_size {
                visible += if tree.visible { 0 } else { 1 };
                tree.visible = true;
                tree_size = tree.size;
            }
        });
        // view from right
        tree_size = self.tree_at(row, self.width - 1).size;
        (1..self.width - 1).rev().for_each(|col| {
            let tree = self.tree_at(row, col);
            if tree.size > tree_size {
                visible += if tree.visible { 0 } else { 1 };
                tree.visible = true;
                tree_size = tree.size;
            }
        });
        visible
    }

    fn tree_score(&mut self, row: usize, col: usize) -> usize {
        let mut score = 1;
        let tree_size = self.tree_at(row, col).size;
        // go north
        let mut r = row - 1;
        while r > 0 && self.tree_at(r, col).size < tree_size {
            r -= 1;
        }
        score *= row.abs_diff(r);
        // go south
        r = row + 1;
        while r < self.height - 1 && self.tree_at(r, col).size < tree_size {
            r += 1
        }
        score *= row.abs_diff(r);
        // go east
        let mut c = col + 1;
        while c < self.width - 1 && self.tree_at(row,c).size < tree_size {
            c += 1;
        }
        score *= col.abs_diff(c);
        // go west (young man)
        c = col - 1;
        while c > 0 && self.tree_at(row, c).size < tree_size {
            c -= 1;
        }
        score *= col.abs_diff(c);

        return score;
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        line.as_bytes().iter().map(|ch| Tree::new(ch)).for_each(|tree| self.matrix.push(tree));
        if self.width == 0 {
            self.width = line.len();
        } else {
            assert_eq!(line.len(), self.width as usize, "Mismatched width!");
        }
        self.height += 1;
    }

    fn solve(&mut self) {
        println!("Forest size: {}x{}", self.width, self.height);
        // part 1
        let mut inside = 0;
        for col in 1..self.width - 1 {
            inside += self.check_column(col);
        }
        for row in 1..self.height - 1 {
            inside += self.check_row(row);
        }
        // borders: 2 x (width + height) - 4 (corners, to not count them multiple times)
        let borders = self.width * 2 + self.height * 2 - 4;
        println!("[1] number of trees visible inside: {} borders: {} => visible {}",
                 inside, borders, inside + borders);


        // part 2
        let mut scenic_max = 0;
        (1..self.height - 1).for_each(|row| {
            (1..self.width - 1).for_each(|col| {
                let score = self.tree_score(row, col);
                if score > scenic_max {
                    scenic_max = score;
                }
            })
        });
        println!("[2] max scenic score {scenic_max}");
    }
}

const ZERO: u8 = '0' as u8;

struct Tree {
    size: u8,
    visible: bool
}

impl Tree {
    fn new(ch: &u8) -> Tree {
        Tree { size: ch - ZERO, visible: false }
    }
}


