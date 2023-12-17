use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<Vec<char>> {
    let reader = BufReader::new(File::open("data").unwrap());
    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}
use std::io::{stdin, stdout, Read, Write};
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
}

fn solve(map: &Vec<Vec<char>>, starting_beam: Beam) -> usize {
    let mut enlightened = HashSet::new();
    let mut cache = HashSet::new();
    let mut beams = VecDeque::from(vec![starting_beam]);
    enlightened.insert((0, 0));
    while !beams.is_empty() {
        let mut beam = beams.pop_back().unwrap();
        loop {
            let (old_x, old_y) = (beam.x, beam.y);
            match map[beam.y][beam.x] {
                '.' => {}
                '|' => {
                    if beam.direction == Direction::Left || beam.direction == Direction::Right {
                        beam.direction = Direction::Down;
                        let new_beam = Beam {
                            x: beam.x,
                            y: beam.y,
                            direction: Direction::Up,
                        };
                        if !cache.contains(&new_beam) {
                            cache.insert(new_beam);
                            beams.push_back(new_beam);
                        }
                    }
                }
                '-' => {
                    if beam.direction == Direction::Up || beam.direction == Direction::Down {
                        beam.direction = Direction::Left;
                        let new_beam = Beam {
                            x: beam.x,
                            y: beam.y,
                            direction: Direction::Right,
                        };
                        if !cache.contains(&new_beam) {
                            cache.insert(new_beam);
                            beams.push_back(new_beam);
                        }
                    }
                }
                '/' => {
                    beam.direction = match beam.direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    }
                }
                '\\' => {
                    beam.direction = match beam.direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    }
                }

                _ => {
                    unreachable!()
                }
            }
            match beam.direction {
                Direction::Right => {
                    if beam.x != map[0].len() - 1 {
                        beam.x += 1;
                    }
                }
                Direction::Left => {
                    if beam.x != 0 {
                        beam.x -= 1;
                    }
                }
                Direction::Up => {
                    if beam.y != 0 {
                        beam.y -= 1;
                    }
                }
                Direction::Down => {
                    if beam.y != map.len() - 1 {
                        beam.y += 1;
                    }
                }
            }

            if old_x == beam.x && old_y == beam.y {
                break;
            }
            enlightened.insert((beam.x, beam.y));

            /*
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if beam.x == j && beam.y == i {
                        match beam.direction {
                            Direction::Up => print!("^"),
                            Direction::Left => print!("<"),
                            Direction::Down => print!("V"),
                            Direction::Right => print!(">"),
                        }
                        continue;
                    }
                    print!("{}", map[i][j]);
                }
                println!("");
            }
            //pause();
            */
        }
    }

    enlightened.into_iter().count()
}

fn part_1(map: &Vec<Vec<char>>) -> usize {
    solve(
        map,
        Beam {
            x: 0,
            y: 0,
            direction: Direction::Right,
        },
    )
}

fn part_2(map: &Vec<Vec<char>>) -> usize {
    let mut max = 0;
    for i in 0..map.len() {
        max = std::cmp::max(
            max,
            solve(
                map,
                Beam {
                    x: 0,
                    y: i,
                    direction: Direction::Right,
                },
            ),
        );
    }
    for i in 0..map.len() {
        max = std::cmp::max(
            max,
            solve(
                map,
                Beam {
                    x: map[0].len() - 1,
                    y: i,
                    direction: Direction::Left,
                },
            ),
        );
    }
    for i in 0..map[0].len() {
        max = std::cmp::max(
            max,
            solve(
                map,
                Beam {
                    x: i,
                    y: 0,
                    direction: Direction::Down,
                },
            ),
        );
    }
    for i in 0..map[0].len() {
        max = std::cmp::max(
            max,
            solve(
                map,
                Beam {
                    x: i,
                    y: map.len() - 1,
                    direction: Direction::Up,
                },
            ),
        );
    }
    max
}

fn main() {
    let map = parse_input();
    println!("part 1: {}", part_1(&map));
    println!("part 2: {}", part_2(&map));
}
