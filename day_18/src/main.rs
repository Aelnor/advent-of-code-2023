use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash)]
struct Command {
    direction: Direction,
    steps: usize,
    color: usize,
}

impl Command {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(' ').collect();
        Command {
            direction: match parts[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            },
            steps: parts[1].parse::<usize>().unwrap(),
            color: 0,
        }
    }
}

fn parse_input() -> Vec<Command> {
    let reader = BufReader::new(File::open("data").unwrap());

    reader
        .lines()
        .into_iter()
        .map(|line| Command::from(&line.unwrap()))
        .collect()
}

#[allow(dead_code)]
fn pretty_print(vec: &Vec<Vec<char>>) {
    for y in 0..vec.len() {
        for x in 0..vec[y].len() {
            print!("{}", vec[y][x]);
        }
        println!("");
    }
}

fn find_start_position(diggings: &Vec<Vec<char>>) -> (usize, usize) {
    for x in 0..diggings[0].len() {
        if diggings[0][x] == '#' {
            return (x + 1, 1);
        }
    }
    unreachable!();
}

fn convert_to_matrix(
    hash_set: &HashSet<(i32, i32)>,
    min_y: i32,
    max_y: i32,
    min_x: i32,
    max_x: i32,
) -> Vec<Vec<char>> {
    let mut result = Vec::with_capacity((max_y - min_y) as usize);

    for y in min_y..=max_y {
        let mut v = Vec::with_capacity((max_x - min_x) as usize);
        for x in min_x..=max_x {
            if hash_set.contains(&(x, y)) {
                v.push('#');
            } else {
                v.push('.');
            }
        }
        result.push(v);
    }

    result
}

fn flood_fill(diggings: &mut Vec<Vec<char>>, x: usize, y: usize) {
    let mut queue = VecDeque::from(vec![(x, y)]);
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if diggings[y][x] == '#' {
            continue;
        }

        diggings[y][x] = '#';

        if x != 0 && diggings[y][x - 1] != '#' {
            queue.push_back((x - 1, y));
        }
        if x != diggings[0].len() - 1 && diggings[y][x + 1] != '#' {
            queue.push_back((x + 1, y));
        }
        if y != 0 && diggings[y - 1][x] != '#' {
            queue.push_back((x, y - 1));
        }
        if y != diggings.len() - 1 && diggings[y + 1][x] != '#' {
            queue.push_back((x, y + 1));
        }
    }
}

fn part_1() -> usize {
    let commands = parse_input();
    let mut x = 0;
    let mut y = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut max_x = 0;
    let mut diggings: HashSet<(i32, i32)> = HashSet::new();
    for command in commands {
        match command.direction {
            Direction::Up => {
                for i in 0..command.steps {
                    y -= 1;
                    diggings.insert((x, y));
                }
            }
            Direction::Down => {
                for i in 0..command.steps {
                    y += 1;
                    diggings.insert((x, y));
                }
            }
            Direction::Left => {
                for i in 0..command.steps {
                    x -= 1;
                    diggings.insert((x, y));
                }
            }
            Direction::Right => {
                for i in 0..command.steps {
                    x += 1;
                    diggings.insert((x, y));
                }
            }
        }
        min_y = std::cmp::min(min_y, y);
        max_y = std::cmp::max(max_y, y);
        min_x = std::cmp::min(min_x, x);
        max_x = std::cmp::max(max_x, x);
    }
    println!(
        "min_x: {}, max_x: {}, min_y: {}, max_y: {}",
        min_x, max_x, min_y, max_y
    );

    let mut vec = convert_to_matrix(&diggings, min_y, max_y, min_x, max_x);
    pretty_print(&vec);
    let (start_x, start_y) = find_start_position(&vec);
    flood_fill(&mut vec, start_x, start_y);

    println!("");
    pretty_print(&vec);
    vec.into_iter().flatten().filter(|e| *e == '#').count()
}

fn main() {
    println!("part 1: {}", part_1());
}
