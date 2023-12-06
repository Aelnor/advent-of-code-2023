use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Filter {
    source_start: usize,
    destination_start: usize,
    range: usize,
}

fn part_1(data: &Vec<usize>, filters: &Vec<Vec<Filter>>) -> usize {
    let mut copy = data.clone();
    for filter_set in filters {
        for i in 0..copy.len() {
            for filter in filter_set {
                if copy[i] >= filter.source_start && copy[i] < filter.source_start + filter.range {
                    copy[i] = filter.destination_start + (copy[i] - filter.source_start);

                    break;
                }
            }
        }
    }
    *copy.iter().min().unwrap()
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn intersects(&self, other: &Range) -> Option<Range> {
        let start = std::cmp::max(self.start, other.start);
        let end = std::cmp::min(self.end, other.end);
        if end < start {
            return None;
        }

        Some(Range { start, end })
    }
}

fn part_2(data: &Vec<usize>, filters: &Vec<Vec<Filter>>) -> usize {
    let mut ranges = Vec::new();
    for i in (0..data.len() - 1).step_by(2) {
        ranges.push(Range {
            start: data[i],
            end: data[i] + data[i + 1] - 1,
        });
    }

    for filter_set in filters {
        let mut next_step = Vec::new();
        let mut i = 0;
        while i < ranges.len() {
            let mut filtered = false;
            for filter in filter_set {
                let new_range = ranges[i].intersects(&Range {
                    start: filter.source_start,
                    end: filter.source_start + filter.range,
                });

                if new_range.is_none() {
                    continue;
                }
                filtered = true;

                let new_range = new_range.unwrap();

                if ranges[i].start < new_range.start {
                    ranges.push(Range {
                        start: ranges[i].start,
                        end: new_range.start - 1,
                    });
                }

                if ranges[i].end > new_range.end {
                    ranges.push(Range {
                        start: new_range.end + 1,
                        end: ranges[i].end,
                    });
                }
                let delta = filter.destination_start as isize - filter.source_start as isize;
                next_step.push(Range {
                    start: new_range.start.checked_add_signed(delta).unwrap(),
                    end: new_range.end.checked_add_signed(delta).unwrap(),
                })
            }
            if !filtered {
                next_step.push(ranges[i].clone());
            }
            i += 1;
        }
        ranges = next_step;
    }

    ranges
        .iter()
        .min_by(|x, y| x.start.cmp(&y.start))
        .unwrap()
        .start
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());

    let mut data = Vec::new();
    let mut filters = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if data.is_empty() {
            let line = line.chars().skip(7).collect::<String>();
            data = line
                .split(' ')
                .map(|e| e.parse::<usize>().unwrap())
                .collect();

            continue;
        }

        if line.is_empty() {
            filters.push(Vec::new());
            continue;
        }

        if line.contains("-to-") {
            continue;
        }

        let parts: Vec<usize> = line
            .split(" ")
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        filters.last_mut().unwrap().push(Filter {
            source_start: parts[1],
            destination_start: parts[0],
            range: parts[2],
        });
    }
    println!("part 1: {}", part_1(&data, &filters));
    println!("part 2: {}", part_2(&data, &filters));
}
