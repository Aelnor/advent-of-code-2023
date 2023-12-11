use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Exits {
    left: String,
    right: String,
}

fn parse_input() -> (String, HashMap<String, Exits>) {
    let mut result = HashMap::new();
    let reader = BufReader::new(File::open("data").unwrap());
    let mut instructions = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if instructions.is_empty() {
            instructions = line;
            continue;
        }
        if line.is_empty() {
            continue;
        }

        let split = line
            .split('=')
            .map(|e| String::from(e.trim()))
            .collect::<Vec<String>>();

        let directions = split[1]
            .chars()
            .skip(1)
            .take(split[1].len() - 2)
            .collect::<String>()
            .split(',')
            .map(|e| String::from(e))
            .collect::<Vec<String>>();

        result.insert(
            split[0].clone(),
            Exits {
                left: String::from(directions[0].trim()),
                right: String::from(directions[1].trim()),
            },
        );
    }
    (instructions, result)
}

fn part_2(start_position: &str) -> usize {
    let (instructions, map) = parse_input();
    let mut position = String::from(start_position);
    let mut step = 0;
    while position.chars().rev().nth(0).unwrap() != 'Z' {
        let direction = instructions.chars().nth(step % instructions.len()).unwrap();
        step += 1;

        match direction {
            'R' => position = map[&position].right.clone(),
            'L' => position = map[&position].left.clone(),
            _ => unreachable!(),
        }
    }
    step
}

fn main() {
    let (instructions, map) = parse_input();
    let mut step = 0;
    let mut position = String::from("AAA");

    while position != "ZZZ" {
        let direction = instructions.chars().nth(step % instructions.len()).unwrap();
        step += 1;

        match direction {
            'R' => position = map[&position].right.clone(),
            'L' => position = map[&position].left.clone(),
            _ => unreachable!(),
        }
    }
    println!("part 1: {}", step);

    let mut positions = Vec::new();
    for pos in map.keys() {
        if pos.chars().rev().nth(0).unwrap() == 'A' {
            positions.push(pos.clone());
        }
    }
    positions.sort();

    println!("{:?}", positions);
    for pos in &positions {
        println!("{}", part_2(pos));
    }
}
