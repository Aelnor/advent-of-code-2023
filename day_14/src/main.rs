use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<Vec<char>> {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut result = Vec::new();

    reader
        .lines()
        .for_each(|line| result.push(line.unwrap().chars().collect()));
    result
}

fn calc_points(map: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    map.iter().enumerate().for_each(|(index, element)| {
        result += element.iter().filter(|e| **e == 'O').count() * (map.len() - index)
    });
    result
}

fn move_north(map: &mut Vec<Vec<char>>) {
    for i in 1..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                let mut new_row = i;
                while new_row != 0 && map[new_row - 1][j] == '.' {
                    new_row -= 1;
                }
                if new_row != i {
                    map[i][j] = '.';
                    map[new_row][j] = 'O';
                }
            }
        }
    }
}

fn move_south(map: &mut Vec<Vec<char>>) {
    for i in (0..map.len() - 1).rev() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                let mut new_row = i;
                while new_row != map.len() - 1 && map[new_row + 1][j] == '.' {
                    new_row += 1;
                }
                if new_row != i {
                    map[i][j] = '.';
                    map[new_row][j] = 'O';
                }
            }
        }
    }
}

fn move_west(map: &mut Vec<Vec<char>>) {
    for j in 1..map[0].len() {
        for i in 0..map.len() {
            if map[i][j] == 'O' {
                let mut new_col = j;
                while new_col != 0 && map[i][new_col - 1] == '.' {
                    new_col -= 1;
                }
                if new_col != j {
                    map[i][j] = '.';
                    map[i][new_col] = 'O';
                }
            }
        }
    }
}

fn move_east(map: &mut Vec<Vec<char>>) {
    for j in (0..map[0].len() - 1).rev() {
        for i in 0..map.len() {
            if map[i][j] == 'O' {
                let mut new_col = j;
                while new_col != map[i].len() - 1 && map[i][new_col + 1] == '.' {
                    new_col += 1;
                }
                if new_col != j {
                    map[i][j] = '.';
                    map[i][new_col] = 'O';
                }
            }
        }
    }
}

fn pretty_print(map: &Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{}", map[i][j]);
        }
        println!("");
    }
    println!("");
}

fn part1() -> usize {
    let mut map = parse_input();
    move_north(&mut map);
    calc_points(&map)
}

fn cycle(map: &mut Vec<Vec<char>>) {
    move_north(map);
    move_west(map);
    move_south(map);
    move_east(map);
}

fn compare_vectors(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    true
}

fn part2() -> usize {
    let mut map = parse_input();
    let mut cache = Vec::new();
    let cycles = 1000000000;

    for i in 0..cycles {
        cycle(&mut map);
        for (index, s) in cache.iter().enumerate() {
            if compare_vectors(&map, s) {
                let step = i - index;
                let last_cycle_start = index + ((cycles - index) / step) * step;
                for _ in last_cycle_start + 1..cycles {
                    cycle(&mut map);
                }

                return calc_points(&map);
            }
        }
        cache.push(map.clone());
    }
    0
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
