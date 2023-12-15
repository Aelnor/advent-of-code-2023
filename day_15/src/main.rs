use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<String> {
    let reader = BufReader::new(File::open("data").unwrap());
    let line = reader.lines().nth(0).unwrap().unwrap();
    line.split(',').map(|e| String::from(e)).collect()
}

fn calculate_hash(s: &str) -> usize {
    let mut result = 0;
    for c in s.chars() {
        result += c as usize;
        result *= 17;
        result %= 256;
    }
    result
}

fn part_1() -> usize {
    let lines = parse_input();
    let mut result = 0;
    lines.into_iter().for_each(|e| result += calculate_hash(&e));
    result
}

enum OperationType {
    Remove,
    Add(u8),
}

struct Operation {
    label: String,
    op: OperationType,
}

impl Operation {
    fn from(s: &str) -> Operation {
        let mut label = String::new();
        let mut i = 0;
        while i < s.len() {
            let c = s.chars().nth(i).unwrap();
            if !c.is_alphabetic() {
                break;
            }
            label.push(c);
            i += 1;
        }
        let c = s.chars().nth(i).unwrap();
        if c == '-' {
            return Operation {
                label,
                op: OperationType::Remove,
            };
        }
        let num = s.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
        Operation {
            label,
            op: OperationType::Add(num as u8),
        }
    }
}

#[derive(Default)]
struct Lens {
    label: String,
    focal_length: u8,
}

fn parse_input_part2() -> Vec<Operation> {
    let operations = parse_input();
    operations
        .into_iter()
        .map(|e| Operation::from(&e))
        .collect()
}

fn part_2() -> usize {
    let operations = parse_input_part2();
    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();
    for op in operations {
        let box_number = calculate_hash(&op.label);
        let lenses = boxes.get_mut(&box_number);
        match op.op {
            OperationType::Remove => {
                if lenses.is_none() {
                    continue;
                }
                let lenses = lenses.unwrap();
                lenses.retain(|e| e.label != op.label);
            }
            OperationType::Add(length) => {
                if lenses.is_none() {
                    boxes.insert(
                        box_number,
                        vec![Lens {
                            label: op.label,
                            focal_length: length,
                        }],
                    );
                    continue;
                }
                let lenses = lenses.unwrap();
                let mut added = false;
                for i in 0..lenses.len() {
                    if lenses[i].label == op.label {
                        lenses[i].focal_length = length;
                        added = true;
                        break;
                    }
                }
                if !added {
                    lenses.push(Lens {
                        label: op.label,
                        focal_length: length,
                    });
                }
            }
        }
    }

    let mut result = 0;
    for (k, b) in &boxes {
        for (i, lens) in b.iter().enumerate() {
            result += (k + 1) * (i + 1) * lens.focal_length as usize;
        }
    }

    result
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}
