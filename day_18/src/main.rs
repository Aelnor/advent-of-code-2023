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
        }
    }

    fn from_v2(s: &str) -> Self {
        let parts: Vec<&str> = s.split(' ').collect();
        let val = String::from(parts[2])
            .chars()
            .skip(2)
            .take(6)
            .collect::<String>();
        Command {
            direction: match val.chars().nth(5).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => unreachable!(),
            },
            steps: usize::from_str_radix(val.chars().take(5).collect::<String>().as_str(), 16)
                .unwrap(),
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

fn parse_input_2() -> Vec<Command> {
    let reader = BufReader::new(File::open("data").unwrap());

    reader
        .lines()
        .into_iter()
        .map(|line| Command::from_v2(&line.unwrap()))
        .collect()
}

fn solve(commands: &Vec<Command>) -> usize {
    let mut x = 0 as isize;
    let mut y = 0 as isize;
    let mut area = 0 as isize;

    for command in commands {
        let old_x = x;
        let old_y = y;

        match command.direction {
            Direction::Up => y -= command.steps as isize,
            Direction::Down => y += command.steps as isize,
            Direction::Left => x -= command.steps as isize,
            Direction::Right => x += command.steps as isize,
        }
        area += (x + old_x) * (y - old_y) + command.steps as isize;
    }

    (area / 2 + 1) as usize
}

fn part_1() -> usize {
    solve(&parse_input())
}

fn part_2() -> usize {
    solve(&parse_input_2())
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}
