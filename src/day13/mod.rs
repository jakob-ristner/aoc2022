use std::{cmp::Ordering, fs};

pub fn solve() {
    let tree_pairs = parse("src/day13/input.txt");
    let mut full_tree_list: Vec<Tree> = Vec::new();

    let p1 = part1(&tree_pairs);

    for (left, right) in tree_pairs {
        full_tree_list.push(left);
        full_tree_list.push(right);
    }
    let p2 = part2(&mut full_tree_list);
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

fn part2(trees: &mut Vec<Tree>) -> usize {
    let t1 = Tree::List {
        list: vec![Tree::List {
            list: vec![Tree::Num { value: 6 }],
        }],
    };
    let t2 = Tree::List {
        list: vec![Tree::List {
            list: vec![Tree::Num { value: 2 }],
        }],
    };
    trees.push(t1.clone());
    trees.push(t2.clone());
    trees.sort();
    trees
        .iter()
        .enumerate()
        .filter(|(_, tree)| tree == &&t1 || tree == &&t2)
        .map(|(index, _)| index + 1)
        .product()
}

fn part1(trees: &Vec<(Tree, Tree)>) -> usize {
    trees
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left.cmp(right) == Ordering::Less)
        .map(|(index, _)| index + 1)
        .sum()
}


impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Tree::Num { value: left_val }, Tree::Num { value: right_val }) => {
                left_val.cmp(right_val)
            }
            (tree, Tree::Num { value: num }) => tree.cmp(&Tree::List {
                list: vec![Tree::Num { value: *num }],
            }),

            (Tree::Num { value: num }, tree) => Tree::List {
                list: vec![Tree::Num { value: *num }],
            }
            .cmp(tree),

            (Tree::List { list: left_list }, Tree::List { list: right_list }) => {
                let left_len = left_list.len();
                let right_len = right_list.len();
                let max: usize;
                if left_len < right_len {
                    max = left_len;
                } else {
                    max = right_len;
                }

                for i in 0..max {
                    match left_list[i].cmp(&right_list[i]) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    };
                }
                left_len.cmp(&right_len)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tree {
    Num { value: u32 },
    List { list: Vec<Tree> },
}

fn parse(path: &str) -> Vec<(Tree, Tree)> {
    let contents = fs::read_to_string(path).unwrap();
    let split: Vec<Vec<&str>> = contents
        .trim()
        .split("\n\n")
        .map(|pair| pair.split("\n").collect::<Vec<&str>>())
        .collect();
    split
        .into_iter()
        .map(|pair_raw| from_raw_pair(pair_raw))
        .collect()
}

fn jump(line: &Vec<char>, index: usize) -> usize {
    let mut accum = 0;
    for i in index..line.len() {
        if line[i] == '[' {
            accum += 1;
        }
        if line[i] == ']' {
            accum -= 1;
            if accum == 0 {
                return i + 1;
            }
        }
    }
    panic!("End of list not found");
}

fn get_tree(line: &Vec<char>, start: usize) -> Tree {
    let mut list: Vec<Tree> = Vec::new();
    let mut i = start;
    while line[i] != ']' {
        match line[i] {
            '[' => {
                let sub_list = get_tree(line, i + 1);
                list.push(sub_list);
                i = jump(line, i);
                if i == line.len() {
                    break;
                }
            }
            ',' => i += 1,
            _ => {
                let mut num_raw: Vec<char> = Vec::new();
                num_raw.push(line[i]);
                for val in i + 1..line.len() {
                    if line[val].is_numeric() {
                        num_raw.push(line[val]);
                    } else {
                        i = val;
                        break;
                    }
                }
                let value: u32 = num_raw.iter().collect::<String>().parse().unwrap();
                list.push(Tree::Num { value })
            }
        };
    }
    Tree::List { list }
}

fn from_raw_pair(pair_raw: Vec<&str>) -> (Tree, Tree) {
    let c1: Vec<char> = pair_raw[0].chars().collect();
    let c2: Vec<char> = pair_raw[1].chars().collect();
    (get_tree(&c1, 1), get_tree(&c2, 1))
}
