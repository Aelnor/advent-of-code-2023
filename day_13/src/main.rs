use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<Vec<Vec<char>>> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open("data").unwrap());
    let mut current_pattern = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            result.push(current_pattern.clone());
            current_pattern.clear();
            continue;
        }
        current_pattern.push(line.chars().collect());
    }
    result.push(current_pattern);

    result
}

fn find_horizontal_symmetry(pattern: &Vec<Vec<char>>, number_of_fails: usize) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 1..pattern.len() {
        let mut current_fails = 0;
        let mut symmetry = true;
        let mut j = 0;
        loop {
            if i + j == pattern.len() || i - 1 < j {
                break;
            }

            for index in 0..pattern[i + j].len() {
                if pattern[i + j][index] != pattern[i - j - 1][index] {
                    current_fails += 1;
                    if current_fails > number_of_fails {
                        symmetry = false;
                        break;
                    }
                }
            }
            if !symmetry {
                break;
            }
            j += 1;
        }

        if symmetry && current_fails == number_of_fails {
            result.push(i);
        }
    }
    result
}

fn find_vertical_symmetry(pattern: &Vec<Vec<char>>, number_of_fails: usize) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 1..pattern[0].len() {
        let mut current_fails = 0;
        let mut symmetry = true;
        let mut j = 0;
        loop {
            if i + j == pattern[0].len() || i - 1 < j {
                break;
            }

            for index in 0..pattern.len() {
                if pattern[index][i + j] != pattern[index][i - j - 1] {
                    current_fails += 1;
                    if current_fails > number_of_fails {
                        symmetry = false;
                        break;
                    }
                }
            }
            if !symmetry {
                break;
            }
            j += 1;
        }

        if symmetry && current_fails == number_of_fails {
            result.push(i);
        }
    }
    result
}

fn solve(patterns: &Vec<Vec<Vec<char>>>, number_of_fails: usize) -> usize {
    let mut columns = 0;
    let mut rows = 0;
    for pattern in patterns {
        let symmetry = find_vertical_symmetry(pattern, number_of_fails);
        for s in &symmetry {
            columns += *s;
        }
    }
    for pattern in patterns {
        let symmetry = find_horizontal_symmetry(pattern, number_of_fails);
        for s in &symmetry {
            rows += *s;
        }
    }
    rows * 100 + columns
}

fn main() {
    let patterns = parse_input();
    println!("part 1: {}", solve(&patterns, 0));
    println!("part 2: {}", solve(&patterns, 1));
}
