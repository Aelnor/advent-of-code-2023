use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    score: usize,
}

impl Hand {
    fn from(s: &str, part2: bool) -> Hand {
        let mut result = Hand {
            cards: Vec::new(),
            score: 0,
        };
        for c in s.chars() {
            if c.is_digit(10) {
                result.cards.push(c.to_digit(10).unwrap());
                continue;
            }
            match c {
                'T' => result.cards.push(10),
                'J' => {
                    if part2 {
                        result.cards.push(1)
                    } else {
                        result.cards.push(11)
                    }
                }
                'Q' => result.cards.push(12),
                'K' => result.cards.push(13),
                'A' => result.cards.push(14),
                _ => unreachable!(),
            }
        }
        result.score = result.get_score();
        result
    }

    fn get_score_impl(v: &Vec<u32>) -> usize {
        let frequencies = v.iter().copied().fold(HashMap::new(), |mut map, val| {
            map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
            map
        });

        if frequencies.values().find(|v| **v == 5).is_some() {
            return 7;
        }
        if frequencies.values().find(|v| **v == 4).is_some() {
            return 6;
        }
        if frequencies.values().find(|v| **v == 3).is_some()
            && frequencies.values().find(|v| **v == 2).is_some()
        {
            return 5;
        }
        if frequencies.values().find(|v| **v == 3).is_some() {
            return 4;
        }
        if frequencies.values().filter(|v| **v == 2).count() == 2 {
            return 3;
        }
        if frequencies.values().find(|v| **v == 2).is_some() {
            return 2;
        }
        if frequencies.len() == 5 {
            return 1;
        }
        0
    }

    fn get_score_recursive(v: &Vec<u32>) -> usize {
        let index = v.iter().position(|&r| r == 1);
        if index.is_none() {
            return Self::get_score_impl(v);
        }
        let index = index.unwrap();
        let mut max = 0;
        let mut v_clone = v.clone();
        for i in 2..=14 {
            v_clone[index] = i;
            let score = Self::get_score_recursive(&v_clone);
            max = std::cmp::max(max, score);
            if score == 7 {
                break;
            }
        }
        max
    }

    fn get_score(&self) -> usize {
        Self::get_score_recursive(&self.cards)
    }
}

#[derive(Debug)]
struct HandAndBid {
    hand: Hand,
    bid: u32,
}

impl HandAndBid {
    fn from(s: &str, part2: bool) -> Self {
        let parts = s.split(' ').collect::<Vec<&str>>();
        HandAndBid {
            hand: Hand::from(parts[0], part2),
            bid: parts[1].parse::<u32>().unwrap(),
        }
    }
}

fn parse_input(part2: bool) -> Vec<HandAndBid> {
    let reader = BufReader::new(File::open("data").unwrap());

    reader
        .lines()
        .map(|line| HandAndBid::from(&line.unwrap(), part2))
        .collect::<Vec<HandAndBid>>()
}

fn calc_score(part2: bool) -> u32 {
    let mut data = parse_input(part2);
    data.sort_by(|a, b| {
        let a_score = a.hand.score;
        let b_score = b.hand.score;
        if a_score != b_score {
            return a_score.cmp(&b_score);
        }

        for i in 0..5 {
            if a.hand.cards[i] != b.hand.cards[i] {
                return a.hand.cards[i].cmp(&b.hand.cards[i]);
            }
        }

        std::cmp::Ordering::Equal
    });

    let mut result = 0;
    for i in 0..data.len() {
        result += (i as u32 + 1) * data[i].bid;
    }
    result
}

fn main() {
    //println!("part 1: {}", calc_score(false));
    println!("part 2: {}", calc_score(true));
}
