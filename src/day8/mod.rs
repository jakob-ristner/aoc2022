use std::fs;

type Forest = Vec<Vec<u32>>;

pub fn solve() {
    let forest = parse("src/day8/input.txt");
    let p1 = count_visible(&forest);
    let p2 = max_score(&forest);

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

fn parse(path: &str) -> Forest {
    let contents = fs::read_to_string(path).unwrap();
    contents
        .trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn max_score(forest: &Forest) -> u32 {
    let mut max = 0;
    for y in 0..forest.len() {
        for x in 0..forest[0].len() {
            let score = scenic_score(x, y, forest);
            if score > max {
                max = score;
            }
        }
    }
    max
}

fn count_visible(forest: &Forest) -> u32 {
    let mut accum = 0;
    for y in 0..forest.len() {
        for x in 0..forest[0].len() {
            if is_visible(x, y, forest) {
                accum += 1;
            }
        }
    }
    accum
}

fn is_visible(x: usize, y: usize, forest: &Forest) -> bool {
    let tree = forest[y][x];
    forest[y][x + 1..forest.len()]
        .into_iter()
        .all(|t| t < &tree)
        || forest[y][0..x].into_iter().all(|t| t < &tree)
        || forest[y + 1..forest.len()].into_iter().all(|t| t[x] < tree)
        || forest[0..y].into_iter().all(|t| t[x] < tree)
}

fn scenic_score(x: usize, y: usize, forest: &Forest) -> u32 {
    let tree = forest[y][x];

    score_segment(&tree, &forest[y][x + 1..forest.len()].to_vec())
        * score_segment(&tree, &forest[y][0..x].to_vec().into_iter().rev().collect())
        * score_segment(
            &tree,
            &forest[y + 1..forest.len()]
                .into_iter()
                .map(|t| t[x])
                .collect(),
        )
        * score_segment(
            &tree,
            &forest[0..y].into_iter().map(|t| t[x]).rev().collect(),
        )
}

fn score_segment(tree: &u32, tree_line: &Vec<u32>) -> u32 {
    let mut score = 0;
    for t in tree_line {
        score += 1;
        if t >= tree {
            break;
        }
    }
    score
}
