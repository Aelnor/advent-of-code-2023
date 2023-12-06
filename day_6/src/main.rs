struct Race {
    time: usize,
    distance: usize,
}

#[allow(dead_code)]
fn get_test_races() -> Vec<Race> {
    vec![
        Race {
            time: 7,
            distance: 9,
        },
        Race {
            time: 15,
            distance: 40,
        },
        Race {
            time: 30,
            distance: 200,
        },
    ]
}

fn get_my_races() -> Vec<Race> {
    vec![
        Race {
            time: 56,
            distance: 499,
        },
        Race {
            time: 97,
            distance: 2210,
        },
        Race {
            time: 77,
            distance: 1097,
        },
        Race {
            time: 93,
            distance: 1440,
        },
    ]
}

#[allow(dead_code)]
fn get_part2_test_races() -> Vec<Race> {
    vec![Race {
        time: 71530,
        distance: 940200,
    }]
}

fn get_part2_my_races() -> Vec<Race> {
    vec![Race {
        time: 56977793,
        distance: 499221010971440,
    }]
}

fn calc_points(races: &Vec<Race>) -> usize {
    let mut result = 1;
    for race in races {
        let mut count = 0;
        let mut winning = false;
        for i in 1..=race.time {
            let remaining_time = race.time - i;
            if i * remaining_time > race.distance {
                winning = true;
                count += 1;
                continue;
            }
            if winning {
                break;
            }
        }
        result *= count;
    }
    result
}

fn main() {
    let races = get_my_races();
    println!("part 1: {}", calc_points(&races));
    let races = get_part2_my_races();
    println!("part 2: {}", calc_points(&races));
}
