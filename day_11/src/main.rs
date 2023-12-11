use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<Vec<char>> {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        result.push(line.chars().collect());
    }
    result
}

fn expand_galaxy(map: &mut Vec<Vec<char>>) {
    let mut i = 0;
    while i < map.len() {
        let mut empty_line = true;
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                empty_line = false;
                break;
            }
        }
        if empty_line {
            map.insert(i, vec!['|'; map[0].len()]);
            i += 1;
        }
        i += 1;
    }

    let mut i = 0;
    while i < map[0].len() {
        let mut empty_column = true;
        for j in 0..map.len() {
            if map[j][i] == '#' {
                empty_column = false;
                break;
            }
        }
        if empty_column {
            for j in 0..map.len() {
                map[j].insert(i, '-')
            }
            i += 1;
        }
        i += 1;
    }
}

struct Point {
    x: usize,
    y: usize,
}

fn find_galaxies(map: &Vec<Vec<char>>, part2: bool) -> Vec<Point> {
    let mut result = Vec::new();
    let mut real_x = 0;
    let mut real_y = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let c = map[y][x];
            if part2 {
                if c == '|' {
                    real_y += 999998;
                    break;
                }
                if c == '-' {
                    real_x += 999999;
                    continue;
                }
            }

            if c == '#' {
                result.push(Point {
                    x: real_x,
                    y: real_y,
                });
            }
            real_x += 1;
        }
        real_x = 0;
        real_y += 1;
    }
    result
}

fn distance(p1: &Point, p2: &Point) -> usize {
    p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)
}

fn part_1() {
    let mut map = parse_input();
    expand_galaxy(&mut map);
    let coordinates = find_galaxies(&map, false);

    let mut result = 0;

    for i in 0..coordinates.len() - 1 {
        for j in i..coordinates.len() {
            result += distance(&coordinates[i], &coordinates[j]);
        }
    }

    println!("part 1: {}", result);
}

fn part_2() {
    let mut map = parse_input();
    expand_galaxy(&mut map);
    let coordinates = find_galaxies(&map, true);

    let mut result = 0;

    for i in 0..coordinates.len() - 1 {
        for j in i..coordinates.len() {
            result += distance(&coordinates[i], &coordinates[j]);
        }
    }

    println!("part 2: {}", result);
}

fn main() {
    part_1();
    part_2();
}
