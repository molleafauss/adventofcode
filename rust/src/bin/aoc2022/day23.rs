// What did I learn?
// small change from the original as GridPos is x,y and not row,col (oops?)
// before I realised all elves share the same set of moves, I was trying to use a slice of references
// for the directions to checks for moving, but it's not needed anymore.

use std::collections::{HashMap, HashSet};
use log::{debug, info};

use adventofcode::grid::{GridPos, MOVE_D, MOVE_DL, MOVE_DR, MOVE_L, MOVE_R, MOVE_U, MOVE_UL, MOVE_UR};
use adventofcode::Solver;

pub(crate) struct Solution {
    width: usize,
    height: usize,
    elves: Vec<Elf>,
    moves: Vec<[GridPos; 3]>,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            width: 0,
            height: 0,
            elves: Vec::new(),
            moves: vec![MOVE_N, MOVE_S, MOVE_W, MOVE_E],
        }
    }

    fn should_move(&self, elf: &Elf, positions: &HashSet<GridPos>) -> Option<GridPos> {
        // println!("Checking move for Elf {} => {:?}", elf.id, self.moves);
        // no elf in surrounding: stay put
        if SURROUNDING.iter().filter(|dir| positions.contains(&elf.pos.add(dir)))
            .count() == 0 {
            return None;
        }
        // else check first direction that can be moved in (all 3 spots should be empty)
        let can_move = self.moves.iter()
            .find(|dirs| !positions.contains(&elf.pos.add(&dirs[0]))
                && !positions.contains(&elf.pos.add(&dirs[1]))
                && !positions.contains(&elf.pos.add(&dirs[2])));
        match can_move {
            None => None,
            Some(dir) => Some(elf.pos.add(&dir[1]))
        }
    }

    fn find_grid(&self) -> (GridPos, GridPos) {
        // maybe clone first elf pos
        let elf = &self.elves[0];
        let mut top_left = elf.pos.clone();
        let mut bottom_right = elf.pos.clone();

        for elf in &self.elves {
            if elf.pos.row < top_left.row {
                top_left.row = elf.pos.row;
            } else if elf.pos.row > bottom_right.row {
                bottom_right.row = elf.pos.row;
            }
            if elf.pos.col < top_left.col {
                top_left.col = elf.pos.col;
            } else if elf.pos.col > bottom_right.col {
                bottom_right.col = elf.pos.col;
            }
        }

        (top_left, bottom_right)
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        if self.width == 0 {
            self.width = line.len();
        }
        line.match_indices("#").for_each(|(pos, _)| {
            let elf = Elf::new(self.elves.len(), self.height, pos);
            self.elves.push(elf);
        });
        self.height += 1;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        let mut positions: HashSet<GridPos> = self.elves.iter().map(|elf| elf.pos.clone()).collect();
        debug!("Will move around {} elves", self.elves.len());
        let mut rounds = 0;
        let mut tl = GridPos::of(0, 0);
        let mut br = GridPos::of(0, 0);
        let mut area = 0;
        loop {
            let mut moves = 0;
            rounds += 1;
            // if rounds > 10 {
            //     return;
            // }
            let mut planned_moves: HashMap<GridPos, Vec<usize>> = HashMap::new();
            for elf in &self.elves {
                if let Some(new_pos) = self.should_move(elf, &positions) {
                    if planned_moves.contains_key(&new_pos) {
                        planned_moves.get_mut(&new_pos).unwrap().push(elf.id);
                    } else {
                        planned_moves.insert(new_pos.clone(), vec![elf.id]);
                    }
                //     println!("Elf {} will move {:?} => {:?}", elf.id, (elf.pos.x, elf.pos.y), (new_pos.x, new_pos.y));
                // } else {
                //     println!("Elf {} won't move => {:?}", elf.id, (elf.pos.x, elf.pos.y));
                }
            }
            for (id, next_pos) in planned_moves.drain()
                    .filter(|(_pos, elves)| elves.len() == 1)
                    .map(|(next_pos, mut elves)| (elves.pop().unwrap(), next_pos)) {
                let elf = self.elves.get_mut(id).unwrap();
                positions.remove(&elf.pos);
                elf.pos = next_pos;
                positions.insert(elf.pos.clone());
                moves += 1;
            }
            // rotate dirs
            let first = self.moves.remove(0);
            self.moves.push(first);
            if rounds == 10 {
                (tl, br) = self.find_grid();
                area = (br.row - tl.row + 1) * (br.col - tl.col + 1) - self.elves.len() as i64;
            }
            if moves == 0 {
                break;
            }
            debug!("=> Round {rounds}: {moves} moves");
        }

        info!("[1] Empty area is {:?}, {:?}  => {area}", tl, br);
        info!("[2] Round {rounds} => no moves");
        Some((area.to_string(), rounds.to_string()))
    }
}

const SURROUNDING: [GridPos; 8] = [MOVE_D, MOVE_DR, MOVE_R, MOVE_UR, MOVE_U, MOVE_UL, MOVE_L, MOVE_DL];
const MOVE_N: [GridPos; 3] = [MOVE_DL, MOVE_D, MOVE_DR];
const MOVE_E: [GridPos; 3] = [MOVE_DR, MOVE_R, MOVE_UR];
const MOVE_S: [GridPos; 3] = [MOVE_UR, MOVE_U, MOVE_UL];
const MOVE_W: [GridPos; 3] = [MOVE_UL, MOVE_L, MOVE_DL];


struct Elf {
    id: usize,
    pos: GridPos,
}

impl Elf {
    fn new(id: usize, row: usize, col: usize) -> Elf {
        Elf {
            id,
            // remember row, col => y, x
            pos: GridPos::of(col as i64, row as i64),
        }
    }
}