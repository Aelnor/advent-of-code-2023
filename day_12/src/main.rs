use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Row {
    value: String,
    numbers: Vec<usize>,
}

fn parse_input() -> Vec<Row> {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts = line.split(' ').collect::<Vec<&str>>();
        let numbers = parts[1]
            .split(',')
            .map(|e| e.trim().parse::<usize>().unwrap())
            .collect();
        result.push(Row {
            value: String::from(parts[0]),
            numbers,
        });
    }
    result
}

fn expand_map(map: &mut Vec<Row>) {
    for i in 0..map.len() {
        let mut new_string = map[i].value.clone();
        for _ in 0..4 {
            new_string.push('?');
            new_string.push_str(&map[i].value);
        }
        map[i].value = new_string;
        let v = map[i].numbers.clone();
        for _ in 0..4 {
            let mut new_numbers = map[i].numbers.clone();
            for n in &v {
                new_numbers.push(*n);
            }
            map[i].numbers = new_numbers;
        }
    }
}

fn process_line(
    s: &str,
    pattern: &Vec<usize>,
    current_line_len: usize,
    cache: &mut HashMap<(String, Vec<usize>, usize), usize>,
) -> usize {
    let val = cache.get(&(String::from(s), pattern.clone(), current_line_len));
    if val.is_some() {
        return *val.unwrap();
    }
    if s.is_empty() {
        return if (current_line_len == 0 && pattern.is_empty())
            || (pattern.len() == 1 && pattern[0] == current_line_len)
        {
            1
        } else {
            0
        };
    }

    let symbols = match s.chars().nth(0).unwrap() {
        '?' => vec!['#', '.'],
        _ => vec![s.chars().nth(0).unwrap()],
    };

    let next_string = String::from(s.chars().skip(1).collect::<String>());
    let mut result = 0;

    for s in &symbols {
        match s {
            '#' => {
                if !pattern.is_empty() && pattern[0] > current_line_len {
                    result += process_line(&next_string, pattern, current_line_len + 1, cache);
                }
            }
            '.' => {
                if current_line_len == 0 {
                    result += process_line(&next_string, pattern, 0, cache);
                    continue;
                }
                // next step of pattern fulfilled
                if !pattern.is_empty() && pattern[0] == current_line_len {
                    result += process_line(
                        &next_string,
                        &pattern.clone().into_iter().skip(1).collect(),
                        0,
                        cache,
                    );
                }
            }
            _ => unreachable!(),
        }
    }
    cache.insert((String::from(s), pattern.clone(), current_line_len), result);

    result
}

fn main() {
    let mut map = parse_input();
    let mut result = 0;
    for row in &map {
        let mut cache = HashMap::new();
        let p = process_line(&row.value, &row.numbers, 0, &mut cache);
        result += p;
    }
    println!("part1: {}", result);

    expand_map(&mut map);
    let mut result = 0;
    for row in map {
        let mut cache = HashMap::new();
        let p = process_line(&row.value, &row.numbers, 0, &mut cache);
        result += p;
    }
    println!("part2: {}", result);
}
