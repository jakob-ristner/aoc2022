use std::{collections::VecDeque, fs};

pub fn solve() {
    let monkeys = parse("src/day11/input.txt");
    let p1 = sim(&mut monkeys.clone(), 20, true);
    let p2 = sim(&mut monkeys.clone(), 10000, false);
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

fn sim(monkeys: &mut Vec<Monkey>, rounds: u32, part1: bool) -> usize {
    let lcm: usize = monkeys.into_iter().map(|m| m.test).product();
    let div: usize;
    if part1 {
        div = 3;
    } else {
        div = 1;
    }
    for _ in 0..rounds {
        round(monkeys, div, lcm);
    }
    let mut inspections: Vec<usize> = monkeys.into_iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.reverse();
    inspections[0..2].iter().product()
}

fn round(monkeys: &mut Vec<Monkey>, div: usize, lcm: usize) {
    for i in 0..monkeys.len() {
        let monkey = monkeys.get_mut(i).unwrap();
        let mut throws: Vec<(Item, usize)> = Vec::new();
        while !monkey.items.is_empty() {
            monkey.inspections = monkey.inspections + 1;
            let item = monkey.items.pop_front().unwrap();
            let mut new_worry = monkey.op.apply(item.worry) / div;
            new_worry = new_worry % lcm;
            let new_item = Item { worry: new_worry };
            let index: usize;
            if new_worry % monkey.test == 0 {
                index = monkey.true_monkey;
            } else {
                index = monkey.false_monkey;
            }
            throws.push((new_item, index));
        }
        for (item, index) in throws {
            let to_monkey = monkeys.get_mut(index).unwrap();
            to_monkey.items.push_back(item);
        }
    }
}

#[derive(Debug, Clone)]
struct Item {
    worry: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Add { value: usize },
    Mul { value: usize },
    Square,
}

impl Operation {
    fn apply(&self, num: usize) -> usize {
        match self {
            Self::Add { value } => num + value,
            Self::Mul { value } => num * value,
            Self::Square => num * num,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<Item>,
    test: usize,
    op: Operation,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize,
}

impl Monkey {
    fn new(
        items: VecDeque<Item>,
        test: usize,
        op: Operation,
        true_monkey: usize,
        false_monkey: usize,
    ) -> Monkey {
        Monkey {
            items,
            test,
            op,
            true_monkey,
            false_monkey,
            inspections: 0,
        }
    }
}

fn parse(path: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let contents = fs::read_to_string(path).unwrap();
    let monkeys_raw: Vec<&str> = contents.trim().split("\n\n").collect();

    for monkey_raw in monkeys_raw {
        let monkey: Vec<&str> = monkey_raw.split("\n").collect();

        let nums_raw = monkey[1].split(":").collect::<Vec<&str>>()[1];
        let no_ws: String = nums_raw.chars().filter(|c| !c.is_whitespace()).collect();
        let items: VecDeque<Item> = no_ws
            .split(",")
            .map(|x| Item {
                worry: x.parse().unwrap(),
            })
            .collect();
        let op_line: Vec<&str> = monkey[2].split_whitespace().collect();
        let op: Operation;
        if op_line[op_line.len() - 1] == "old" {
            op = Operation::Square;
        } else {
            let value: usize = op_line[op_line.len() - 1].parse().unwrap();
            op = match op_line[op_line.len() - 2] {
                "+" => Operation::Add { value },
                "*" => Operation::Mul { value },
                _ => panic!(),
            };
        }

        let test_line: Vec<&str> = monkey[3].split_whitespace().collect();
        let test: usize = test_line[test_line.len() - 1].parse().unwrap();

        let true_line: Vec<&str> = monkey[4].split_whitespace().collect();
        let true_monkey: usize = true_line[true_line.len() - 1].parse().unwrap();

        let false_line: Vec<&str> = monkey[5].split_whitespace().collect();
        let false_monkey: usize = false_line[false_line.len() - 1].parse().unwrap();
        monkeys.push(Monkey::new(items, test, op, true_monkey, false_monkey));
    }
    monkeys
}
