use std::collections::HashSet;

pub fn solve() {
    let rocks = parse();
    let bottom = lower_bound(&rocks);
    let p1 = iterate_sand(&mut rocks.clone(), bottom, 1);
    println!("Part 1: {}", p1);
    let p2 = iterate_sand(&mut rocks.clone(), bottom, 2);
    println!("Part 2: {}", p2);

}

type Pos = (usize, usize);

fn iterate_sand(rocks: &mut HashSet<Pos>, lower_bound: usize, part: u32) -> u32 {
    let mut accum = 0;
    loop {
        accum += 1;
        match sand_fall(rocks, lower_bound, part) {
            Some(pos) => {
                rocks.insert(pos);
            }
            None => return accum - (part % 2), // lol
        }
    }
}

fn lower_bound(rocks: &HashSet<Pos>) -> usize {
    *rocks.into_iter().map(|(_, y)| y).max().unwrap()
}

fn is_floor(lower_bound: usize, pos: &Pos) -> bool {
    pos.1 == lower_bound + 2
}

fn sand_fall(rocks: &HashSet<Pos>, lower_bound: usize, part: u32) -> Option<Pos> {
    let (mut x, mut y) = (500, 0);

    loop {
        let left = (x - 1, y + 1);
        let right = (x + 1, y + 1);
        let down = (x, y + 1);

        if (y > lower_bound) && part == 1 {
            return None;
        }

        if rocks.contains(&down) || is_floor(lower_bound, &down) {
            // Go left
            if rocks.contains(&left) || is_floor(lower_bound, &left) {
                //GO right
                if rocks.contains(&right) || is_floor(lower_bound, &right) {
                    if (x, y) == (500, 0) && part == 2 {
                        return None;
                    }
                    return Some((x, y));
                } else {
                    (x, y) = (x + 1, y + 1);
                }
            } else {
                (x, y) = (x - 1, y + 1)
            }
        } else {
            (x, y) = (x, y + 1);
        }
    }
}

fn parse() -> HashSet<Pos> {
    let lines = include_str!("input.txt").trim().split("\n");
    let mut positions = HashSet::new();
    for line in lines {
        from_line(line, &mut positions)
    }
    positions
}

fn from_line(line: &str, positions: &mut HashSet<Pos>) {
    let vertices_raw: Vec<&str> = line.split(" -> ").collect();
    for i in 0..vertices_raw.len() - 1 {
        let from_raw: Vec<&str> = vertices_raw[i].split(",").collect();
        let to_raw: Vec<&str> = vertices_raw[i + 1].split(",").collect();
        let (fx, fy): Pos = (from_raw[0].parse().unwrap(), from_raw[1].parse().unwrap());
        let (tx, ty): Pos = (to_raw[0].parse().unwrap(), to_raw[1].parse().unwrap());

        if fx == tx {
            // Only one of these will ever happen, but ugly tho
            for ny in fy..=ty {
                positions.insert((fx, ny));
            }
            for ny in ty..=fy {
                positions.insert((fx, ny));
            }
        } else {
            // Only one of these will ever happen, but ugly tho
            for nx in fx..=tx {
                positions.insert((nx, fy));
            }
            for nx in tx..=fx {
                positions.insert((nx, fy));
            }
        }
    }
}
