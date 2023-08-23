// What did I learn?
// small change from the original as GridPos is x,y and not row,col (oops?)

use std::collections::{HashMap, HashSet};

use crate::grid::{GridPos, MOVE_D, MOVE_DL, MOVE_DR, MOVE_L, MOVE_R, MOVE_U, MOVE_UL, MOVE_UR};
use crate::Solver;

pub(crate) struct Solution<'a> {
    width: usize,
    height: usize,
    elves: Vec<Elf<'a>>,
}

impl<'a> Solution<'a> {
    pub(crate) fn new() -> Solution<'a> {
        Solution {
            width: 0,
            height: 0,
            elves: Vec::new(),
        }
    }

    fn should_move(&self, elf: &Elf, positions: &HashSet<GridPos>) -> Option<GridPos> {
        println!("Checking move for Elf {} => {:?}", elf.id, elf.dir);
        // no elf in surrounding: stay put, else if can move: which direction?
        let can_move = elf.dir.iter()
            .find(|dirs| !positions.contains(&elf.pos.add(dirs[0]))
                && !positions.contains(&elf.pos.add(dirs[1]))
                && !positions.contains(&elf.pos.add(dirs[2])));
        match can_move {
            None => None,
            Some(dir) => Some(elf.pos.add(dir[1]))
        }
    }

    fn find_grid(&self) -> (GridPos, GridPos) {
        // maybe clone first elf pos
        let elf = &self.elves[0];
        let mut top_left = elf.pos.clone();
        let mut bottom_right = elf.pos.clone();

        for elf in &self.elves {
            if elf.pos.y < top_left.y {
                top_left.y = elf.pos.y;
            } else if elf.pos.y > bottom_right.y {
                bottom_right.y = elf.pos.y;
            }
            if elf.pos.x < top_left.x {
                top_left.x = elf.pos.x;
            } else if elf.pos.x > bottom_right.x {
                bottom_right.x = elf.pos.x;
            }
        }

        (top_left, bottom_right)
    }
}

impl<'a> Solver for Solution<'a> {
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

    fn solve(&mut self) {
        let mut positions: HashSet<GridPos> = self.elves.iter().map(|elf| elf.pos.clone()).collect();
        println!("Will move around {} elves", self.elves.len());
        let mut rounds = 0;
        let mut tl = GridPos::of(0, 0);
        let mut br = GridPos::of(0, 0);
        let mut area = 0;
        loop {
            let mut moves = 0;
            rounds += 1;
            if rounds > 10 {
                return;
            }
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
            self.elves.iter_mut().for_each(|elf| elf.rotate_dirs());
            if rounds == 10 {
                (tl, br) = self.find_grid();
                area = (br.y - tl.y + 1) * (br.x - tl.x + 1) - self.elves.len() as i64;
            }
            if moves == 0 {
                break;
            }
            println!("=> Round {rounds}: {moves} moves");
        }

        println!("[1] Empty area is {:?}, {:?}  => {area}", tl, br);
        println!("[2] Round {rounds} => no moves");
    }
}

const SURROUNDING: [&GridPos; 8] = [&MOVE_D, &MOVE_DR, &MOVE_R, &MOVE_UR, &MOVE_U, &MOVE_UL, &MOVE_L, &MOVE_DL];
const MOVE_N: [&GridPos; 3] = [&MOVE_DL, &MOVE_D, &MOVE_DR];
const MOVE_E: [&GridPos; 3] = [&MOVE_DR, &MOVE_R, &MOVE_UR];
const MOVE_S: [&GridPos; 3] = [&MOVE_UR, &MOVE_U, &MOVE_UL];
const MOVE_W: [&GridPos; 3] = [&MOVE_UL, &MOVE_L, &MOVE_DL];


struct Elf<'a> {
    id: usize,
    pos: GridPos,
    dir: Vec<[&'a GridPos; 3]>,
}

impl<'a> Elf<'a> {
    fn new(id: usize, row: usize, col: usize) -> Elf<'a> {
        Elf {
            id,
            // remember row, col => y, x
            pos: GridPos::of(col as i64, row as i64),
            dir: vec![MOVE_N, MOVE_S, MOVE_W, MOVE_E],
        }
    }

    fn rotate_dirs(&mut self) {
        let first = self.dir.remove(0);
        self.dir.push(first);
    }
}