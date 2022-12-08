use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("src/day7/input.txt").unwrap();
    let lines: Vec<&str> = contents.trim().split("\n").collect();
    let fs = get_file_system(lines);

    let p1 = part1(&fs);
    let p2 = part2(&fs);

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

fn part1(fs: &FileSystem) -> u32 {
    fs.keys()
        .map(|dir| get_size(dir, &fs))
        .filter(|x| *x <= 100000)
        .sum()
}

fn part2(fs: &FileSystem) -> u32 {
    let diff = 30000000 - (70000000 - get_size(&vec!["/".to_string()], fs));

    let sizes: Vec<u32> = fs.keys().map(|dir| get_size(dir, &fs)).collect();

    let mut min_viable = sizes.iter().max().unwrap().clone();
    for size in sizes {
        if size >= diff && size < min_viable {
            min_viable = size;
        }
    }
    min_viable
}

#[derive(Debug)]
enum Item {
    Dir { path: Vec<String> },
    File { size: u32 },
}

type FileSystem = HashMap<Vec<String>, Vec<Item>>;

fn get_file_system(lines: Vec<&str>) -> FileSystem {
    let mut fs = HashMap::new();
    fs.insert(vec!["/".to_string()], Vec::new());
    let mut dir_stack: Vec<String> = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();

        if split[1] == "cd" {
            if split[2] == ".." {
                dir_stack.pop();
            } else {
                dir_stack.push(split[2].to_string())
            }
        }

        if split[0] == "dir" {
            let mut path = dir_stack.clone();
            path.push(split[1].to_string());
            fs.insert(path.clone(), Vec::new());
            fs.get_mut(&dir_stack).unwrap().push(Item::Dir { path })
        }

        if split[0].parse::<u32>().is_ok() {
            fs.get_mut(&dir_stack).unwrap().push(Item::File {
                size: split[0].parse().unwrap(),
            });
        }
    }
    fs
}

fn get_size(dir: &Vec<String>, fs: &FileSystem) -> u32 {
    let mut accum = 0;
    for item in fs.get(dir).unwrap() {
        accum += match item {
            Item::Dir { path } => get_size(path, fs),
            Item::File { size } => *size,
        };
    }
    accum
}
