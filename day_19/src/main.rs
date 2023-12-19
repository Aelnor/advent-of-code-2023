use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Clone, PartialEq, Eq)]
enum Command {
    Accept,
    Reject,
    GotoWorkflow(String),
}

impl Command {
    fn from(s: &str) -> Self {
        match s {
            "R" => Command::Reject,
            "A" => Command::Accept,
            _ => Command::GotoWorkflow(String::from(s)),
        }
    }
}

enum Comparison {
    Greater,
    Less,
    None,
}

struct Rule {
    characteristic: char,
    comparison: Comparison,
    value: usize,
    command: Command,
}

impl Rule {
    fn from(s: &str) -> Self {
        if s.len() == 1 || (!s.contains('<') && !s.contains('>')) {
            return Rule {
                characteristic: '-',
                comparison: Comparison::None,
                value: 0,
                command: Command::from(s),
            };
        }

        let parts: Vec<&str> = s.split(':').collect();
        let comparison = match parts[0].chars().nth(1).unwrap() {
            '>' => Comparison::Greater,
            '<' => Comparison::Less,
            _ => unreachable!(),
        };
        let value = parts[0]
            .chars()
            .skip(2)
            .take(150)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        Rule {
            characteristic: parts[0].chars().nth(0).unwrap(),
            comparison,
            value,
            command: Command::from(parts[1]),
        }
    }
}

fn parse_input() -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut objects = Vec::new();
    let reader = BufReader::new(File::open("data").unwrap());
    let mut parts = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            parts = true;
            continue;
        }

        if !parts {
            let re = Regex::new(r"(\w*)\{(.*)\}").unwrap();
            let caps = re.captures(&line).unwrap();

            workflows.insert(
                String::from(&caps[1]),
                caps[2]
                    .split(',')
                    .map(|e| Rule::from(e))
                    .collect::<Vec<Rule>>(),
            );
            continue;
        }

        let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
        let caps = re.captures(&line).unwrap();
        objects.push(Part {
            x: caps[1].parse::<usize>().unwrap(),
            m: caps[2].parse::<usize>().unwrap(),
            a: caps[3].parse::<usize>().unwrap(),
            s: caps[4].parse::<usize>().unwrap(),
        });
    }

    (workflows, objects)
}

fn apply_workflow<'a>(
    part: &Part,
    workflows: &'a HashMap<String, Vec<Rule>>,
    workflow_name: &str,
) -> &'a Command {
    let mut name = String::from(workflow_name);
    loop {
        let workflow = workflows.get(&name).unwrap();
        for r in workflow {
            let mut exec = false;
            match r.comparison {
                Comparison::None => exec = true,
                Comparison::Less => match r.characteristic {
                    'x' => {
                        if part.x < r.value {
                            exec = true;
                        }
                    }
                    'm' => {
                        if part.m < r.value {
                            exec = true;
                        }
                    }
                    'a' => {
                        if part.a < r.value {
                            exec = true;
                        }
                    }
                    's' => {
                        if part.s < r.value {
                            exec = true;
                        }
                    }
                    _ => unreachable!(),
                },
                Comparison::Greater => match r.characteristic {
                    'x' => {
                        if part.x > r.value {
                            exec = true;
                        }
                    }
                    'm' => {
                        if part.m > r.value {
                            exec = true;
                        }
                    }
                    'a' => {
                        if part.a > r.value {
                            exec = true;
                        }
                    }
                    's' => {
                        if part.s > r.value {
                            exec = true;
                        }
                    }
                    _ => unreachable!(),
                },
            }
            if exec {
                match &r.command {
                    Command::Accept | Command::Reject => return &r.command,
                    Command::GotoWorkflow(flow) => {
                        name = flow.clone();
                        break;
                    }
                }
            }
        }
    }
}

fn main() {
    let (workflows, parts) = parse_input();
    let mut result = 0;
    for p in parts {
        let d = apply_workflow(&p, &workflows, "in");
        match d {
            Command::Accept => {
                result += p.x + p.m + p.a + p.s;
            }
            Command::Reject => {}
            Command::GotoWorkflow(_) => panic!(),
        }
    }
    println!("part 1: {}", result);
}
