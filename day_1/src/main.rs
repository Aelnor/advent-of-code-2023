use std::fs::File;
use std::io::{BufRead, BufReader};

fn replace_numbers(s: String) -> String {
    let mut result = String::new();
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..s.len() {
            let c = s.chars().nth(i).unwrap();
            if c.is_digit(10) {
                result.push(c);
                continue;
            }
            let d3 = s.chars().skip(i).take(3).collect::<String>();
            let d5 = s.chars().skip(i).take(5).collect::<String>();
            let d4 = s.chars().skip(i).take(4).collect::<String>();
            if d3 == "one" {
                result.push('1');
                continue;
            }
            if d3 == "two" {
                result.push('2');
                continue;
            }
            if d5 == "three" {
                result.push('3');
                continue;
            }
            if d4 == "four" {
                result.push('4');
                continue;
            }
            if d4 == "five" {
                result.push('5');
                continue;
            }
            if d3 == "six" {
                result.push('6');
                continue;
            }
            if d5 == "seven" {
                result.push('7');
                continue;
            }
            if d5 == "eight" {
                result.push('8');
                continue;
            }
            if d4 == "nine" {
                result.push('9');
                continue;
            }
        }
    }
    result
}

fn main() {
    let filename = "data";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let line = replace_numbers(line.unwrap());
        let mut first_digit = None;
        let mut last_digit = None;
        for c in line.chars() {
            if c.is_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(c);
                }
                last_digit = Some(c);
            }
        }
        let mut number = String::new();
        number.push(first_digit.unwrap());
        number.push(last_digit.unwrap());
        result += number.parse::<i32>().unwrap();
    }
    println!("result: {}", result);
}
