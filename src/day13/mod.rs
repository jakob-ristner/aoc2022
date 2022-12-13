use std::{cmp::Ordering, fs};

pub fn solve() {
    let packet_pairs = parse("src/day13/input.txt");
    let mut full_packet_list: Vec<Packet> = Vec::new();

    let p1 = part1(&packet_pairs);

    for (left, right) in packet_pairs {
        full_packet_list.push(left);
        full_packet_list.push(right);
    }
    let p2 = part2(&mut full_packet_list);
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

fn part2(packets: &mut Vec<Packet>) -> usize {
    let t1 = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);
    let t2 = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
    packets.push(t1.clone());
    packets.push(t2.clone());
    packets.sort();
    packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| packet == &&t1 || packet == &&t2)
        .map(|(index, _)| index + 1)
        .product()
}

fn part1(packets: &Vec<(Packet, Packet)>) -> usize {
    packets
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(index, _)| index + 1)
        .sum()
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Num(left_val), Packet::Num(right_val)) => left_val.cmp(right_val),
            (packet, Packet::Num(num)) => packet.cmp(&Packet::List(vec![Packet::Num(*num)])),
            (Packet::Num(num), packet) => Packet::List(vec![Packet::Num(*num)]).cmp(packet),
            (Packet::List(left_list), Packet::List(right_list)) => {
                for i in 0..*vec![left_list.len(), right_list.len()]
                    .iter()
                    .min()
                    .unwrap()
                {
                    match left_list[i].cmp(&right_list[i]) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    };
                }
                left_list.len().cmp(&right_list.len())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

fn parse(path: &str) -> Vec<(Packet, Packet)> {
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

fn get_packet(line: &Vec<char>, start: usize) -> Packet {
    let mut list: Vec<Packet> = Vec::new();
    let mut i = start;
    while line[i] != ']' {
        match line[i] {
            '[' => {
                list.push(get_packet(line, i + 1));
                i = jump(line, i);
            }
            ',' => i += 1,
            _ => {
                let (m, _) = line.iter().enumerate().find(|(p, x)| p > &i && !x.is_numeric()).unwrap();
                let value: u32 = line[i..m].iter().collect::<String>().parse().unwrap();
                i = m;
                list.push(Packet::Num(value))
            }
        };
    }
    Packet::List(list)
}

fn from_raw_pair(pair_raw: Vec<&str>) -> (Packet, Packet) {
    let c1: Vec<char> = pair_raw[0].chars().collect();
    let c2: Vec<char> = pair_raw[1].chars().collect();
    (get_packet(&c1, 1), get_packet(&c2, 1))
}
