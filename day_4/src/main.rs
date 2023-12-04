use std::fs::File;
use std::io::{BufRead, BufReader};

struct Card {
    matching_numbers: usize,
    count: usize,
}

impl Card {
    fn from(s: &str) -> Card {
        let parts = s.split(':').collect::<Vec<&str>>();

        let number_parts = parts[1].split('|').map(|e| e.trim()).collect::<Vec<&str>>();
        let winning_numbers : Vec<usize> = number_parts[0].split(' ').filter(|&e| !e.is_empty()).map(|e| e.trim().parse::<usize>().unwrap()).collect();
        let my_numbers : Vec<usize> = number_parts[1].split(' ').filter(|&e| !e.is_empty()).map(|e| e.trim().parse::<usize>().unwrap()).collect();
        let mut matching_numbers = 0;
        for number in &winning_numbers {
            if my_numbers.contains(&number) {
                matching_numbers += 1;
            }
        }

        Card {
            matching_numbers,
            count: 1,
        }
    }

    fn points(&self) -> usize {
        if self.matching_numbers == 0 {
            0
        } else {
            (2 as usize).pow(self.matching_numbers as u32 - 1)
        }
    }

}

fn main() {
    let file = File::open("data").unwrap();
    let reader = BufReader::new(file);

    let mut cards = reader.lines().map(|e| Card::from(&e.unwrap())).collect::<Vec<Card>>();

    let result = cards.iter().fold(0, |sum, val| sum + val.points());
    println!("part 1: {}", result);

    for i in 0..cards.len() {
        for j in 0..cards[i].matching_numbers {
            let index = i + j + 1;
            if index == cards.len() {
                break;
            }
            cards[index].count += cards[i].count;
        }
    }

    println!("part 2: {}", cards.iter().fold(0, |sum, elem| sum + elem.count));
}
