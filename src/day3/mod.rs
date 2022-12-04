use array_tool::vec::Intersect;
use std::fs;
pub fn solve() {
    let parsed = parse("src/day3/input.txt");
    let p1 = part1(&parsed);
    let p2 = part2(&parsed);
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

fn part1(sacks: &Vec<String>) -> u32 {
    let sacks: Vec<(String, String)> = sacks.into_iter().map(|line| half(line)).collect();
    let commons: Vec<Vec<char>> = sacks.into_iter().map(|sack| common(&sack)).collect();
    let mut concat: Vec<char> = Vec::new();
    for sack in commons {
        concat.append(&mut sack.clone());
    }
    concat.into_iter().map(|c| to_number(c)).sum()
}

fn to_number(c: char) -> u32 {
    if c.is_lowercase() {
        return c as u32 - 96;
    }
    return c as u32 - 38;
}

fn parse(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap();
    let split: Vec<&str> = contents.trim().split("\n").collect();
    return split.into_iter().map(|line| line.to_string()).collect();
}

fn half(line: &str) -> (String, String) {
    let length = line.len();
    (
        line[0..length / 2].to_string(),
        line[length / 2..length].to_string(),
    )
}

fn common(sack: &(String, String)) -> Vec<char> {
    let first: Vec<char> = sack.0.chars().collect();
    let second: Vec<char> = sack.1.chars().collect();
    first.intersect(second)
}

fn part2(sacks: &Vec<String>) -> u32 {
    let triplets = threes(sacks);
    let mut common: Vec<char> = Vec::new();
    for trip in triplets {
        common.append(&mut triple_intersect(trip));
    }
    common.into_iter().map(|c| to_number(c)).sum()

}

fn triple_intersect(sacks: (String, String, String)) -> Vec<char> {
    let first: Vec<char> = sacks.0.chars().collect();
    let second: Vec<char> = sacks.1.chars().collect();
    let third: Vec<char> = sacks.2.chars().collect();
    first.intersect(second).intersect(third)
}

fn threes(lines: &Vec<String>) -> Vec<(String, String, String)> {
    let mut triplets: Vec<(String, String, String)> = Vec::new();
    let mut index = 0;
    for i in 0..(lines.len() / 3) {
        index = i * 3;
        triplets.push((
            lines.get(index).unwrap().to_string(),
            lines.get(index + 1).unwrap().to_string(),
            lines.get(index + 2).unwrap().to_string(),
        ));
    }
    triplets
}
