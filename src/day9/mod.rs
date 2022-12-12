use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let mut rope_p1 = Rope::new(1);
    let mut rope_p2 = Rope::new(2);
    let moves = parse("src/day9/input.txt");

    let pos_set1 = rope_p1.apply_moves(&moves);
    let pos_set2 = rope_p2.apply_moves(&moves);

    println!("Part 1: {}\nPart 2: {}", pos_set1.len(), pos_set2.len());
}

#[derive(Debug)]
struct Rope {
    knots: HashMap<usize, (i32, i32)>,
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Right,
    Left,
    Down,
    Up,
}

impl Dir {
    fn delta(&self) -> (i32, i32) {
        match self {
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Down => (0, -1),
            Dir::Up => (0, 1),
        }
    }

    fn from_string(letter: &str) -> Dir {
        match letter {
            "L" => Dir::Left,
            "R" => Dir::Right,
            "D" => Dir::Down,
            "U" => Dir::Up,
            _ => panic!(),
        }
    }
}

impl Rope {
    fn new(part: u32) -> Rope {
        let mut knots = HashMap::new();
        let mut amount = 10;
        if part == 1 {
            amount = 2;
        }
        for k in 0..amount {
            knots.insert(k, (0, 0));
        }
        Rope { knots }
    }

    fn is_adj(me: &(i32, i32), ref_knot: &(i32, i32)) -> bool {
        let (tx, ty) = me;
        for x in -1..2 {
            for y in -1..2 {
                if (tx + x, ty + y) == *ref_knot {
                    return true;
                }
            }
        }
        return false;
    }

    fn adjust_knot(me: &(i32, i32), ref_knot: &(i32, i32)) -> (i32, i32) {
        let (hx, hy) = ref_knot;

        let mut out = *me;

        if Self::is_adj(me, ref_knot) {
            return out;
        }

        while !Self::is_adj(&out, ref_knot) {
            let (tx, ty) = out;
            let (dx, dy) = ((hx - tx).signum(), (hy - ty).signum());
            if ty == *hy {
                out = (tx + dx, ty);
                continue;
            }

            if tx == *hx {
                out = (tx, ty + dy);
                continue;
            }

            if hy > &ty {
                if hx > &tx {
                    out = (tx + 1, ty + 1);
                    continue;
                } else {
                    out = (tx - 1, ty + 1);
                    continue;
                }
            } else {
                if hx > &tx {
                    out = (tx + 1, ty - 1);
                    continue;
                } else {
                    out = (tx - 1, ty - 1);
                    continue;
                }
            }
        }

        return out;
    }

    fn mov(&mut self, dir: Dir, amount: usize) -> HashSet<(i32, i32)> {
        let (dx, dy) = dir.delta();
        let mut out = HashSet::new();

        let tail = self.knots.len() - 1;
        out.insert(self.knots.get(&tail).unwrap().clone());

        for _ in 0..amount {
            let (hx, hy) = self.knots.get(&0).unwrap();
            self.knots.insert(0, (hx + dx, hy + dy));

            for k in 1..tail + 1 {
                let me = self.knots.get(&k).unwrap();
                let ref_knot = self.knots.get(&(k - 1)).unwrap();
                let new = Self::adjust_knot(me, ref_knot);
                self.knots.insert(k, new);
            }
            out.insert(self.knots.get(&tail).unwrap().clone());
        }

        out
    }

    fn apply_moves(&mut self, moves: &Vec<(Dir, usize)>) -> HashSet<(i32, i32)> {
        let mut out = HashSet::new();
        for (dir, amount) in moves {
            let pos_set = self.mov(*dir, *amount);
            for pos in pos_set {
                out.insert(pos);
            }
        }

        out
    }
}

fn parse(path: &str) -> Vec<(Dir, usize)> {
    let mut out = Vec::new();

    let contents = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = contents.trim().split("\n").collect();

    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();
        out.push((Dir::from_string(split[0]), split[1].parse().unwrap()));
    }

    return out;
}
