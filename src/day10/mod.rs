use std::fs;

struct Crt {
    pixels: String,
}

impl Crt {
    fn draw(&mut self, cycle: &i32, x: &X) {
        let pos = (cycle - 1) % 40;
        if x.value - 1 == pos || x.value == pos || x.value + 1 == pos {
            self.pixels.push_str("█");
        } else {
            self.pixels.push_str(" ");
        }
    }

    fn print(&self) {
        let chars: Vec<char> = self.pixels.chars().collect();
        for chunk in chars.chunks(40) {
            println!("{}", chunk.iter().collect::<String>());
        }
    }
}

pub fn solve() {
    let ops = parse("src/day10/input.txt");
    let desired = vec![20, 60, 100, 140, 180, 220];
    let mut crt = Crt {
        pixels: "".to_string(),
    };
    let p1 = apply_ops(&ops, desired, &mut crt);
    println!("Part 1: {}\nPart 2:", p1);
    crt.print();
}

struct X {
    value: i32,
}

fn push_if_desired(cycle: &i32, x: &X, out: &mut Vec<i32>, desired: &Vec<i32>) {
    if desired.contains(cycle) {
        out.push(x.value * cycle);
    }
}

fn apply_ops(ops: &Vec<Op>, desired: Vec<i32>, crt: &mut Crt) -> i32 {
    let mut x = X { value: 1 };
    let mut cycle = 1;
    crt.draw(&cycle, &mut x);
    let mut out = Vec::new();

    for op in ops {
        match op {
            Op::Addx { value } => {
                cycle += 1;
                crt.draw(&cycle, &x);
                push_if_desired(&cycle, &x, &mut out, &desired);
                cycle += 1;
                x.value = x.value + value;
                crt.draw(&cycle, &x);
                push_if_desired(&cycle, &x, &mut out, &desired);
            }
            Op::Noop => {
                cycle += 1;
                crt.draw(&cycle, &x);
                push_if_desired(&cycle, &x, &mut out, &desired);
            }
        };
    }
    out.iter().sum()
}

#[derive(Debug)]
enum Op {
    Noop,
    Addx { value: i32 },
}

impl Op {
    fn from_string(line: &str) -> Op {
        if line == "noop" {
            return Op::Noop;
        } else {
            let split: Vec<&str> = line.split_whitespace().collect();
            return Op::Addx {
                value: split[1].parse().unwrap(),
            };
        }
    }
}

fn parse(path: &str) -> Vec<Op> {
    let contents = fs::read_to_string(path).unwrap();
    contents
        .trim()
        .split("\n")
        .map(|line| Op::from_string(line))
        .collect()
}
