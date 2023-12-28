use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<Vec<char>> {
    let reader = BufReader::new(File::open("data").unwrap());
    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
    direction: (isize, isize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.y))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(map: &Vec<Vec<char>>, min_steps: usize, max_steps: usize) -> usize {
    let mut queue = BinaryHeap::new();
    let mut costs = HashMap::new();
    queue.push(State {
        cost: 0,
        x: 0,
        y: 0,
        direction: (0, 0),
    });

    while let Some(state) = queue.pop() {
        if state.y == map.len() - 1 && state.x == map[0].len() - 1 {
            return state.cost;
        }

        if costs
            .get(&(state.x, state.y, state.direction))
            .is_some_and(|&cost| state.cost > cost)
        {
            continue;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if (dx, dy) == (state.direction.0, state.direction.1)
                || (dx, dy) == (-state.direction.0, -state.direction.1)
            {
                continue;
            }
            let mut next_cost = state.cost;

            for i in 1..=max_steps {
                let new_x = (state.x as isize + i as isize * dx) as usize;
                let new_y = (state.y as isize + i as isize * dy) as usize;

                if new_x >= map[0].len() || new_y >= map.len() {
                    break;
                }

                next_cost += map[new_y][new_x].to_digit(10).unwrap() as usize;

                if i < min_steps {
                    continue;
                }

                if next_cost < *costs.get(&(new_x, new_y, (dx, dy))).unwrap_or(&usize::MAX) {
                    costs.insert((new_x, new_y, (dx, dy)), next_cost);
                    queue.push(State {
                        cost: next_cost,
                        x: new_x,
                        y: new_y,
                        direction: (dx, dy),
                    });
                }
            }
        }
    }

    unreachable!()
}

fn part_1(map: &Vec<Vec<char>>) -> usize {
    solve(map, 1, 3)
}

fn part_2(map: &Vec<Vec<char>>) -> usize {
    solve(map, 4, 10)
}

fn main() {
    let map = parse_input();

    println!("part 1: {}", part_1(&map));
    println!("part 2: {}", part_2(&map));
}
