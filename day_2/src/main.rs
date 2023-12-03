use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
struct Set {
    red: u8,
    green: u8,
    blue: u8,
}

impl Set {
    fn from(s: &str) -> Self {
        let parts = s.split(',').map(|e| e.trim()).collect::<Vec<&str>>();
        let mut result: Set = Default::default();

        for part in parts {
            let ball = part.split(' ').collect::<Vec<&str>>();
            let num = ball[0].parse::<u8>().unwrap();
            match ball[1] {
                "red" => result.red = num,
                "green" => result.green = num,
                "blue" => result.blue = num,
                _ => unreachable!(),
            }
        }
        result
    }
}

#[derive(Default)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn get_game_id(s: &str) -> u32 {
    s.chars()
        .skip(5)
        .take(3)
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn parse_sets(s: &str) -> Vec<Set> {
    let mut result = Vec::new();

    let parts = s.split(';').map(|e| e.trim()).collect::<Vec<&str>>();

    for part in parts {
        result.push(Set::from(part));
    }

    result
}

fn load_games() -> Vec<Game> {
    let file = File::open("data").unwrap();
    let reader = BufReader::new(file);
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(':').collect();
        let game = Game {
            id: get_game_id(parts[0]),
            sets: parse_sets(parts[1]),
        };
        result.push(game);
    }
    result
}

fn part_1() -> u32 {
    let games = load_games();
    let mut result: u32 = 0;
    for game in games {
        let mut possible = true;
        for set in game.sets {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                possible = false;
                break;
            }
        }
        if possible {
            result += game.id;
        }
    }
    result
}

fn part_2() -> u32 {
    let games = load_games();
    let mut result: u32 = 0;
    for game in games {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for set in game.sets {
            min_red = std::cmp::max(set.red, min_red);
            min_green = std::cmp::max(set.green, min_green);
            min_blue = std::cmp::max(set.blue, min_blue);
        }
        result += min_red as u32 * min_blue as u32 * min_green as u32;
    }
    result
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}
