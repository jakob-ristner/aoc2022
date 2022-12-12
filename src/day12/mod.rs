use std::char;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::sync::mpsc::channel;

pub fn solve() {
    let (char_mat, start, end) = parse("src/day12/input.txt");
    let graph1 = build_graph(&char_mat, cmp_p1);
    let graph2 = build_graph(&char_mat, cmp_p2);
    let p1 = bfs(&graph1, start, &char_mat, goal_p1);
    let p2 = bfs(&graph2, end, &char_mat, goal_p2);
    println!("Part 1: {}\nPart 2: {}", p1.unwrap(), p2.unwrap());
}

fn goal_p1(c: char) -> bool {
    c == 'E'
}

fn goal_p2(c: char) -> bool {
    c == 'a' || c == 'S'
}

fn cmp_p1(from: i32, to: i32) -> bool {
    to - from <= 1
}

fn cmp_p2(from: i32, to: i32) -> bool {
    from - to <= 1
}

type Pos = (i32, i32);
type Graph = HashMap<Pos, Vec<Pos>>;
type CharMap = HashMap<Pos, char>;

fn parse(path: &str) -> (Vec<Vec<char>>, Pos, Pos) {
    let contents = fs::read_to_string(path).unwrap();
    let char_mat: Vec<Vec<char>> = transpose(
        contents
            .trim()
            .split("\n")
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect(),
    );
    let mut start = (0, 0);
    let mut end = (0, 0);
    for x in 0..char_mat.len() {
        for y in 0..char_mat[0].len() {
            if char_mat[x][y] == 'S' {
                start = (x as i32, y as i32);
            } else if char_mat[x][y] == 'E' {
                end = (x as i32, y as i32);
            }
        }
    }

    (char_mat, start, end)
}


fn bfs(graph: &Graph, start: Pos, char_mat: &Vec<Vec<char>>, done: fn(char) -> bool) -> Option<u32> {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut next: HashSet<Pos> = HashSet::new();
    for pos in graph.get(&start).unwrap() {
        next.insert(*pos);
    }
    let mut level = 0;
    visited.insert(start);

    while !next.is_empty() {
        level += 1;
        let mut new_next: HashSet<Pos> = HashSet::new();
        for pos in &next {
            if done(char_mat[pos.0 as usize][pos.1 as usize]) {
                return Some(level);
            }
            for npos in graph.get(pos).unwrap() {
                new_next.insert(*npos);
            }
        }
        next = new_next;
    }
    None
}

fn get_height(c: &char) -> i32 {
    match c {
        'S' => 'a' as i32,
        'E' => 'z' as i32,
        _ => *c as i32,
    }
}

fn build_graph(char_mat: &Vec<Vec<char>>, cmp: fn(i32, i32) -> bool) -> Graph {
    let mut graph = HashMap::new();
    let width = char_mat.len() as i32;
    let height = char_mat[0].len() as i32;

    for x in 0..width {
        for y in 0..height {
            let c = char_mat[x as usize][y as usize];
            let h_from = get_height(&c);
            let mut conn: Vec<Pos> = Vec::new();
            let adj = get_adjacent_coords(x as i32, y as i32, width, height);
            for (ax, ay) in adj {
                let h_to = get_height(&char_mat[ax as usize][ay as usize]);
                if cmp(h_from, h_to) {
                    conn.push((ax, ay));
                }
            }
            graph.insert((x, y), conn);
        }
    }
    graph
}

fn get_adjacent_coords(x: i32, y: i32, width: i32, height: i32) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nx = x + dx;
        let ny = y + dy;
        if nx >= 0 && nx < width && ny >= 0 && ny < height {
            out.push((nx, ny));
        }
    }
    out
}

//Just to get (x,y) coords
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
