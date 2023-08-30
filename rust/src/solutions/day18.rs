// What did I learn?
// quite straight-forward. Was using an enum for the Voxels, but then equality was not out of the box
// and constructs were a bit me so I fell back to u8.
// Used a struct for the XYZ position so I added operations there.
// Ranges/Range is just a convenience for clean code, could've used (a, b) tuples. Didn't use "None"
// like in python and added a flag for the whole "Ranges" object before you keep track of the fist
// voxel.

use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;
use log::{debug, info};

use crate::Solver;

pub(crate) struct Solution {
    cubes: u32,
    faces: u32,
    voxels: HashMap<Pos, u8>,
    ranges: Ranges,
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            cubes: 0,
            faces: 0,
            voxels: HashMap::new(),
            ranges: Ranges::empty(),
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let mut iter = line.split(",").into_iter().map(|v| i32::from_str(v).unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        assert!(iter.next().is_none(), "Invalid line? {}", line);
        let pos = Pos(x, y, z);
        if let Some(&VOXEL_LAVA) = self.voxels.get(&pos) {
            panic!("Found already assigned lava in {:?}", pos);
        }
        let mut visible = 6;
        for n in pos.neighbours() {
            if self.voxels.contains_key(&n) {
                self.faces -= 1;
                visible -= 1;
            }
        }
        self.voxels.insert(pos.clone(), VOXEL_LAVA);
        self.ranges.add_voxel(pos);
        self.faces += visible;
        self.cubes += 1;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] {} cubes: {} visible", self.voxels.len(), self.faces);

        debug!("Ranges: {:?}", self.ranges);
        // add AIR all in the layer around and then grow it until it touches the lava. Count the faces this way
        // back
        self.add_air((self.ranges.x.min - 1, self.ranges.x.max + 1), (self.ranges.y.min - 1, self.ranges.y.max + 1), (self.ranges.z.min - 1, self.ranges.z.min));
        // front
        self.add_air((self.ranges.x.min - 1, self.ranges.x.max + 1), (self.ranges.y.min - 1, self.ranges.y.max + 1), (self.ranges.z.max + 1, self.ranges.z.max+ 2));
        // top
        self.add_air((self.ranges.x.min - 1, self.ranges.x.max + 1), (self.ranges.y.min - 1, self.ranges.y.min), (self.ranges.z.min - 1, self.ranges.z.max + 1));
        // bottom
        self.add_air((self.ranges.x.min - 1, self.ranges.x.max + 1), (self.ranges.y.max + 1, self.ranges.y.max + 2), (self.ranges.z.min - 1, self.ranges.z.max+ 1));
        // top
        self.add_air((self.ranges.x.min - 1, self.ranges.x.min), (self.ranges.y.min - 1, self.ranges.y.max + 1), (self.ranges.z.min - 1, self.ranges.z.max + 1));
        // bottom
        self.add_air((self.ranges.x.max + 1, self.ranges.x.max + 2), (self.ranges.y.min - 1, self.ranges.y.max + 1), (self.ranges.z.min - 1, self.ranges.z.max+ 1));

        // expand air and count faces touched
        let mut faces = HashSet::new();
        let mut air: VecDeque<Pos> = self.voxels.iter()
            .filter(|(_k, v)| **v == VOXEL_AIR)
            .map(|(k, _v)| k.clone())
            .collect();
        while air.len() > 0 {
            let pos = air.pop_front().unwrap();
            for n in pos.neighbours() {
                if !self.ranges.contains(&n) {
                    continue;
                }
                if !self.voxels.contains_key(&n) {
                    air.push_back(n);
                } else if self.voxels[&n] == VOXEL_LAVA {
                    let face = pos.face(&n);
                    faces.insert(face);
                }
            }
            self.voxels.insert(pos, VOXEL_AIR);
        }

        info!("[2] Found {} outside facing faces", faces.len());
        Some((self.faces.to_string(), faces.len().to_string()))
    }
}

impl Solution {
    fn add_air(&mut self, rangex: (i32, i32), rangey: (i32, i32), rangez: (i32, i32)) {
        debug!("Add air: {:?}, {:?}, {:?}", rangex, rangey, rangez);
        for x in rangex.0..rangex.1 {
            for y in rangey.0..rangey.1 {
                for z in rangez.0..rangez.1 {
                    self.voxels.insert(Pos(x, y, z), VOXEL_AIR);
                }
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Pos(i32, i32, i32);

const MOVE_RIGHT: Pos = Pos(1, 0, 0);
const MOVE_LEFT: Pos = Pos(-1, 0, 0);
const MOVE_UP: Pos = Pos(0, 1, 0);
const MOVE_DOWN: Pos = Pos(0, -1, 0);
const MOVE_FRONT: Pos = Pos(0, 0, 1);
const MOVE_BACK: Pos = Pos(0, 0, -1);

const VOXEL_AIR: u8 = 0;
const VOXEL_LAVA: u8 = 1;

impl Pos {
    fn neighbours(&self) -> Vec<Pos> {
        vec![
            self.add(&MOVE_RIGHT),
            self.add(&MOVE_LEFT),
            self.add(&MOVE_UP),
            self.add(&MOVE_DOWN),
            self.add(&MOVE_FRONT),
            self.add(&MOVE_BACK)
        ]
    }

    fn add(&self, other: &Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    fn face(&self, pos: &Pos) -> (i32, i32, i32, i32, i32, i32) {
        (self.0, self.1, self.2, self.0 - pos.0, self.1 - pos.1, self.2 - pos.2)
    }
}

#[derive(Debug)]
struct Range {
    min: i32,
    max: i32,
}

#[derive(Debug)]
struct Ranges {
    empty: bool,
    x: Range,
    y: Range,
    z: Range,
}

impl Ranges {
    fn empty() -> Ranges {
        Ranges {
            empty: true,
            x: Range {min: 0, max: 0},
            y: Range {min: 0, max: 0},
            z: Range {min: 0, max: 0},
        }
    }

    fn add_voxel(&mut self, pos: Pos) {
        if self.empty || self.x.min > pos.0 {
            self.x.min = pos.0;
        }
        if self.empty || self.x.max < pos.0 {
            self.x.max = pos.0;
        }
        if self.empty || self.y.min > pos.1 {
            self.y.min = pos.1;
        }
        if self.empty || self.y.max < pos.1 {
            self.y.max = pos.1;
        }
        if self.empty || self.z.min > pos.2 {
            self.z.min = pos.2;
        }
        if self.empty || self.z.max < pos.2 {
            self.z.max = pos.2;
        }
        self.empty = false;
    }

    fn contains(&self, pos: &Pos) -> bool {
        self.x.min <= pos.0 && pos.0 <= self.x.max
            && self.y.min <= pos.1 && pos.1 <= self.y.max
            && self.z.min <= pos.2 && pos.2 <= self.z.max
    }
}
