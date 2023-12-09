use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open("data").unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        result.push(
            line.split(" ")
                .filter(|e| !e.is_empty())
                .map(|e| e.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
    }
    result
}

fn extrapolate(v: &Vec<i32>, p2: bool) -> i32 {
    let mut data = Vec::new();
    data.push(v.clone());
    loop {
        let last_row = data.last().unwrap();
        let mut new_row = Vec::new();
        for i in 0..last_row.len() - 1 {
            new_row.push(last_row[i + 1] - last_row[i]);
        }
        data.push(new_row);
        let mut all_zeroes = true;

        for el in data.last().unwrap() {
            if *el != 0 {
                all_zeroes = false;
                break;
            }
        }

        if all_zeroes {
            break;
        }
    }

    if p2 {
        for i in 0..data.len() {
            data[i].reverse();
        }
    }
    for i in (0..data.len() - 1).rev() {
        let new_number = if p2 {
            data[i].last().unwrap() - data[i + 1].last().unwrap()
        } else {
            data[i].last().unwrap() + data[i + 1].last().unwrap()
        };
        data[i].push(new_number);
    }
    *data[0].last().unwrap()
}

fn main() {
    let data = parse_input();
    println!(
        "part 1: {}",
        data.iter().fold(0, |sum, el| sum + extrapolate(el, false))
    );
    println!(
        "part 2: {}",
        data.iter().fold(0, |sum, el| sum + extrapolate(el, true))
    );
}
