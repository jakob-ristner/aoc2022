use std::fs;

pub fn solve() {
    let (crates, moves) = parse("src/day5/input.txt");
    let p1 = get_sol(&mut crates.clone(), &moves, true);
    let p2 = get_sol(&mut crates.clone(), &moves, false);
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

fn get_sol(crates: &mut Vec<String>, moves: &Vec<Move>, rev: bool) -> String {
    let mut aws = String::new();
    for m in moves {
        m.apply(crates, rev);
    }
    for c in crates {
        aws.push_str(&c[0..1]);
    }
    aws
}

fn parse(file_path: &str) -> (Vec<String>, Vec<Move>) {
    let contents = fs::read_to_string(file_path).unwrap();
    let split: Vec<&str> = contents.split("\n\n").collect();
    let crates = get_crates(split[0]);
    let moves = get_moves(split[1]);
    (crates, moves)
}

fn get_crates(contents: &str) -> Vec<String> {
    let mut crates: Vec<String> = Vec::new();

    let split: Vec<&str> = contents.split("\n").collect();

    for line in split {
        let initial: String = line
            .chars()
            // .filter(|x| x.is_alphanumeric())
            .map(|c| convert(c))
            .collect::<Vec<char>>()
            .iter()
            .collect();
        crates.push(initial);
    }
    let trans = transpose(crates);
    trans
}

fn get_moves(contents: &str) -> Vec<Move> {
    let split: Vec<&str> = contents.trim().split("\n").collect();
    split.into_iter().map(|x| Move::new(x)).collect()
}

fn convert(c: char) -> char {
    match c {
        '[' => ' ',
        ']' => ' ',
        a => a,
    }
}

fn transpose(raw_crates: Vec<String>) -> Vec<String> {
    let max: u32 = raw_crates[raw_crates.len() - 1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .max()
        .unwrap();

    let mut crates: Vec<String> = Vec::new();
    for _ in 0..max {
        crates.push("".to_string());
    }

    let nums: &Vec<char> = &raw_crates[raw_crates.len() - 1].chars().collect();

    for line in raw_crates {
        let mut new_index;
        let vec: Vec<char> = line.chars().collect();

        for raw_index in 0..line.len() {
            if vec[raw_index].is_alphabetic() {
                new_index = nums[raw_index].to_digit(10).unwrap() - 1;
                crates
                    .get_mut(new_index as usize)
                    .unwrap()
                    .push(vec[raw_index]);
            }
        }
    }
    crates
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn new(line: &str) -> Move {
        let cs: Vec<usize> = line
            .chars()
            .filter(|x| x.is_numeric() || *x == ' ')
            .collect::<String>()
            .trim()
            .to_string()
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Move {
            amount: cs[0],
            from: cs[1] - 1,
            to: cs[2] - 1,
        }
    }

    fn apply(&self, crates: &mut Vec<String>, rev: bool) {
        let mut taken: String = crates.get(self.from).unwrap()[0..self.amount].to_string();
        if rev {
            taken = taken.chars().rev().collect();
        }
        taken.push_str(crates.get(self.to).unwrap());
        _ = std::mem::replace(&mut crates[self.to], taken);
        crates
            .get_mut(self.from)
            .unwrap()
            .replace_range(0..self.amount, "");
    }
}
