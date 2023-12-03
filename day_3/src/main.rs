use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn has_adjacent_symbol(
    schematic: &Vec<Vec<char>>,
    starting_x: usize,
    ending_x: usize,
    y: usize,
    number: u32,
    star_map: &mut HashMap<(usize, usize), Vec<u32>>,
) -> bool {
    let mut result = false;
    if starting_x != 0 {
        if y != 0 {
            let symbol = schematic[y - 1][starting_x - 1];
            if !symbol.is_digit(10) && symbol != '.' {
                if symbol == '*' {
                    star_map
                        .entry((y - 1, starting_x - 1))
                        .and_modify(|v| v.push(number))
                        .or_insert(vec![number]);
                }
                result = true;
            }
        }
        let symbol = schematic[y][starting_x - 1];
        if !symbol.is_digit(10) && symbol != '.' {
            if symbol == '*' {
                star_map
                    .entry((y, starting_x - 1))
                    .and_modify(|v| v.push(number))
                    .or_insert(vec![number]);
            }
            result = true;
        }
        if y != schematic.len() - 1 {
            let symbol = schematic[y + 1][starting_x - 1];
            if !symbol.is_digit(10) && symbol != '.' {
                if symbol == '*' {
                    star_map
                        .entry((y + 1, starting_x - 1))
                        .and_modify(|v| v.push(number))
                        .or_insert(vec![number]);
                }
                result = true;
            }
        }
    }
    if ending_x != schematic[y].len() - 1 {
        if y != 0 {
            let symbol = schematic[y - 1][ending_x + 1];
            if !symbol.is_digit(10) && symbol != '.' {
                if symbol == '*' {
                    star_map
                        .entry((y - 1, ending_x + 1))
                        .and_modify(|v| v.push(number))
                        .or_insert(vec![number]);
                }
                result = true;
            }
        }
        let symbol = schematic[y][ending_x + 1];
        if !symbol.is_digit(10) && symbol != '.' {
            if symbol == '*' {
                star_map
                    .entry((y, ending_x + 1))
                    .and_modify(|v| v.push(number))
                    .or_insert(vec![number]);
            }
            result = true;
        }
        if y != schematic.len() - 1 {
            let symbol = schematic[y + 1][ending_x + 1];
            if !symbol.is_digit(10) && symbol != '.' {
                if symbol == '*' {
                    star_map
                        .entry((y + 1, ending_x + 1))
                        .and_modify(|v| v.push(number))
                        .or_insert(vec![number]);
                }
                result = true;
            }
        }
    }
    if y != 0 {
        for x in starting_x..=ending_x {
            let symbol = schematic[y - 1][x];
            if !symbol.is_digit(10) && symbol != '.' {
                if symbol == '*' {
                    star_map
                        .entry((y - 1, x))
                        .and_modify(|v| v.push(number))
                        .or_insert(vec![number]);
                }
                result = true;
            }
        }
    }
    if y != schematic.len() - 1 {
        for x in starting_x..=ending_x {
            let symbol = schematic[y + 1][x];
            if !symbol.is_digit(10) && symbol != '.' {
                if symbol == '*' {
                    star_map
                        .entry((y + 1, x))
                        .and_modify(|v| v.push(number))
                        .or_insert(vec![number]);
                }
                result = true;
            }
        }
    }
    result
}

fn part_1(schematic: &mut Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    let mut star_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for y in 0..schematic.len() {
        let mut x = 0;
        while x < schematic[y].len() {
            if !schematic[y][x].is_digit(10) {
                x += 1;
                continue;
            }
            let mut number = String::new();
            let starting_x = x;

            while x < schematic[y].len() && schematic[y][x].is_digit(10) {
                number.push(schematic[y][x]);
                schematic[y][x] = '.';
                x += 1;
            }
            let number = number.parse::<u32>().unwrap();
            if has_adjacent_symbol(&schematic, starting_x, x - 1, y, number, &mut star_map) {
                result += number;
            }
        }
    }
    let mut result2 = 0;

    for v in star_map.values() {
        if v.len() == 2 {
            result2 += v[0] * v[1];
        }
    }
    println!("{}", result2);
    result
}

fn main() {
    let file = File::open("data").unwrap();
    let reader = BufReader::new(file);

    let mut schematic = Vec::new();
    reader
        .lines()
        .for_each(|line| schematic.push(line.unwrap().chars().collect::<Vec<char>>()));

    println!("part 1: {}", part_1(&mut schematic));
}
