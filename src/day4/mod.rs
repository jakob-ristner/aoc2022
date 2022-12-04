use std::collections::HashSet;
use std::fs;

type Section = HashSet<u32>;

pub fn solve() {
    let pairs = parse("src/day4/input.txt");
    let p1 = part1(&pairs);
    let p2 = part2(&pairs);
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

fn part2(pairs: &Vec<(Section, Section)>) -> u32 {
    let mut accum = 0;
    for sections in pairs {
        if overlapping(sections) {
            accum += 1;
        }
    }
    accum
}

fn part1(pairs: &Vec<(Section, Section)>) -> u32 {
    let mut accum = 0;
    for sections in pairs {
        if fully_contains(sections) {
            accum += 1;
        }
    }
    accum
}

fn get_section(section: &str) -> Section {
    let split: Vec<&str> = section.split("-").collect();
    let from: u32 = split[0].parse().unwrap();
    let to: u32 = split[1].parse().unwrap();
    (from..to + 1).collect()
}

fn parse_line(sections: &str) -> (Section, Section) {
    let split: Vec<&str> = sections.split(",").collect();
    (get_section(split[0]), get_section(split[1]))
}

fn parse(file_path: &str) -> Vec<(Section, Section)> {
    let contents = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = contents.trim().split("\n").collect();
    lines.into_iter().map(|line| parse_line(line)).collect()
}

fn fully_contains(sections: &(Section, Section)) -> bool {
    sections.0.iter().all(|item| sections.1.contains(item))
        || sections.1.iter().all(|item| sections.0.contains(item))
}

fn overlapping(sections: &(Section, Section)) -> bool {
    sections
        .0
        .intersection(&sections.1)
        .collect::<HashSet<&u32>>()
        .len()
        > 0
}
