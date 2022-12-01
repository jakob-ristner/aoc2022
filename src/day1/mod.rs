use std::fs;

pub fn solve() {
    let file_path = "src/day1/input.txt";
    let p1 = part1(&parse(file_path));
    let p2 = part2(&parse(file_path));

    println!("Part1: {}\nPart2: {}", p1, p2);
}

fn parse(file_path: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let chunks: Vec<&str> = contents.split("\n\n").collect();
    chunks
        .into_iter()
        .map(|x| x.trim().split("\n").map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn part1(cals: &Vec<Vec<u32>>) -> u32 {
    cals.into_iter().map(|x| x.iter().sum()).max().unwrap()
}

fn part2(cals: &Vec<Vec<u32>>) -> u32 {
    let mut summed: Vec<u32> = cals.into_iter().map(|x| x.iter().sum()).collect();
    summed.sort();
    summed.iter().rev().take(3).sum()
}
