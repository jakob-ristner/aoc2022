use std::fs;

pub fn solve() {
    let rounds1 = parse("src/day2/input.txt");
    let rounds2 = parse2("src/day2/input.txt");
    let p1 = calc(&rounds1);
    let p2 = calc(&rounds2);
    println!("Part 1: {}\nPart2: {}", p1, p2);
}

fn calc(rounds: &Vec<Round>) -> u32 {
    rounds.into_iter().map(|round| score(round)).sum()
}

type Round = (Shape, Shape);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }

    fn get(letter: &str) -> Shape {
        match letter {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissor,
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissor,
            _ => panic!("Could not determine type"),
        }
    }
    fn get_my_choice(opp: &Shape, letter: &str) -> Shape {
        match (opp, letter) {
            (_, "Y") => opp.clone(),
            (Shape::Rock, "X") => Shape::Scissor,
            (Shape::Rock, "Z") => Shape::Paper,
            (Shape::Paper, "X") => Shape::Rock,
            (Shape::Paper, "Z") => Shape::Scissor,
            (Shape::Scissor, "X") => Shape::Paper,
            (Shape::Scissor, "Z") => Shape::Rock,
            (_, _) => panic!("Could not determine choice"),
        }
    }
}

fn line_to_round(line: &str) -> Round {
    let letters: Vec<&str> = line.split(" ").collect();
    (Shape::get(letters[0]), Shape::get(letters[1]))
}

fn line_to_round2(line: &str) -> Round {
    let letters: Vec<&str> = line.split(" ").collect();
    let opp = Shape::get(letters[0]);
    (opp, Shape::get_my_choice(&opp, letters[1]))
}


fn score(round: &Round) -> u32 {
    let win_score = match round {
        (Shape::Rock, Shape::Paper) => 6,
        (Shape::Rock, Shape::Scissor) => 0,
        (Shape::Scissor, Shape::Rock) => 6,
        (Shape::Scissor, Shape::Paper) => 0,
        (Shape::Paper, Shape::Scissor) => 6,
        (Shape::Paper, Shape::Rock) => 0,
        (_, _) => 3,
    };
    win_score + round.1.score()
}

fn parse(file_path: &str) -> Vec<Round> {
    let text = fs::read_to_string(file_path).expect("Could not read file");
    let parsed: Vec<&str> = text.trim().split("\n").collect();
    parsed.into_iter().map(|line| line_to_round(line)).collect()
}

fn parse2(file_path: &str) -> Vec<Round> {
    let text = fs::read_to_string(file_path).expect("Could not read file");
    let parsed: Vec<&str> = text.trim().split("\n").collect();
    parsed.into_iter().map(|line| line_to_round2(line)).collect()
}
