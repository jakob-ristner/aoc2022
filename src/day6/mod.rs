use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let chars = parse("src/day6/input.txt");
    let p1 = sol(&chars, 4).unwrap();
    let p2 = sol(&chars, 14).unwrap();
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

fn parse(file_path: &str) -> Vec<char> {
    fs::read_to_string(file_path)
        .unwrap()
        .trim()
        .chars()
        .collect()
}

fn sol(cs: &Vec<char>, length: usize) -> Option<usize> {
    let mut curr_streak: HashSet<char> = HashSet::new();

    for i in 0..cs.len() - length - 1 {
        let mut pos = i;
        while !curr_streak.contains(&cs[pos]) && pos - i < length {
            curr_streak.insert(cs[pos]);
            pos += 1;
        }

        if pos - i == length {
            return Some(pos);
        }
        curr_streak.clear();
    }

    None
}
