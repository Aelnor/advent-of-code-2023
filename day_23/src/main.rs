use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_map() -> Vec<Vec<char>> {
    let file = File::open("data");
    let reader = BufReader::new(file.unwrap());
    let mut result = Vec::new();

    for line in reader.lines() {
        let line: Vec<char> = line.unwrap().chars().collect();
        result.push(line);
    }

    result
}

fn find_empty_point(map: &Vec<Vec<char>>, row: usize) -> usize {
    for x in 0..map[row].len() {
        if map[row][x] == '.' {
            return x;
        }
    }
    unreachable!()
}

fn find_longest_path(
    map: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    target_x: usize,
    target_y: usize,
) -> usize {
    let mut queue = VecDeque::new();
    let steps = HashSet::new();
    let mut len_map = Vec::new();
    for _ in 0..map.len() {
        len_map.push(vec![0; map[0].len()]);
    }

    queue.push_back((start_x, start_y, 1, steps));

    while let Some((x, y, d, st)) = queue.pop_front() {
        if len_map[y][x] > d {
            continue;
        }

        if st.contains(&(x, y)) {
            continue;
        }

        len_map[y][x] = d;

        if x == target_x && y == target_y {
            continue;
        }

        let mut s = st.clone();
        s.insert((x, y));
        match map[y][x] {
            '.' => {}
            '>' => {
                queue.push_back((x + 1, y, d + 1, s));
                continue;
            }
            '<' => {
                queue.push_back((x - 1, y, d + 1, s));
                continue;
            }
            '^' => {
                queue.push_back((x, y - 1, d + 1, s));
                continue;
            }
            'v' => {
                queue.push_back((x, y + 1, d + 1, s));
                continue;
            }
            '#' => {
                println!("Hit a wall at {}, {}!", x, y);
                unreachable!();
            }
            _ => {
                unreachable!();
            }
        }

        let dirs: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

        for (dx, dy) in dirs {
            let new_x = add_i32_to_usize(x, dx);
            let new_y = add_i32_to_usize(y, dy);

            if new_x.is_none() || new_y.is_none() {
                continue;
            }

            let new_x = new_x.unwrap();
            let new_y = new_y.unwrap();

            if map.len() == new_y {
                continue;
            }

            if map[0].len() == new_x {
                continue;
            }

            if map[new_y][new_x] == '#' {
                continue;
            }

            queue.push_back((new_x, new_y, d + 1, s.clone()));
        }
    }

    len_map[target_y][target_x] - 1
}

fn add_i32_to_usize(u: usize, i: i32) -> Option<usize> {
    if i >= 0 {
        u.checked_add(i as usize)
    } else {
        u.checked_sub((-i) as usize)
    }
}

type Position = (usize, usize);
#[derive(Debug)]
struct Edge {
    pos: Position,
    weight: usize,
}
type Graph = HashMap<Position, Vec<Edge>>;

fn build_graph(map: &Vec<Vec<char>>) -> Graph {
    let mut graph = Graph::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != '.' {
                continue;
            }
            let mut edges = Vec::new();
            for (dx, dy) in &directions {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
                if new_x >= 0
                    && new_y >= 0
                    && new_x < map[y].len() as isize
                    && new_y < map.len() as isize
                    && map[new_y as usize][new_x as usize] == '.'
                {
                    edges.push(Edge {
                        pos: (new_x as usize, new_y as usize),
                        weight: 1,
                    });
                }
            }
            graph.insert((x, y), edges);
        }
    }

    graph
}

fn simplify_graph(graph: &mut Graph) {
    let mut to_remove = HashSet::new();

    for (k, v) in &mut *graph {
        if v.len() == 2 {
            to_remove.insert(k.clone());
        }
    }

    for pos in &to_remove {
        let edges = graph.remove(&pos).unwrap();
        let first_edge = &edges[0];
        let second_edge = &edges[1];

        let node_to_modify = graph.get_mut(&first_edge.pos).unwrap();
        for i in 0..node_to_modify.len() {
            if node_to_modify[i].pos == *pos {
                node_to_modify[i].pos = second_edge.pos;
                node_to_modify[i].weight += second_edge.weight;
                break;
            }
        }

        let node_to_modify = graph.get_mut(&second_edge.pos).unwrap();
        for i in 0..node_to_modify.len() {
            if node_to_modify[i].pos == *pos {
                node_to_modify[i].pos = first_edge.pos;
                node_to_modify[i].weight += first_edge.weight;
                break;
            }
        }
    }
}

fn dfs(
    graph: &Graph,
    start: &Position,
    end: &Position,
    distance: usize,
    visited: &mut HashSet<Position>,
) -> usize {
    if start == end {
        return distance;
    }
    let node = graph.get(start).unwrap();
    let mut max = distance;
    visited.insert(*start);
    for edge in node {
        if visited.contains(&edge.pos) {
            continue;
        }
        let d = dfs(graph, &edge.pos, end, distance + edge.weight, visited);
        if d > max {
            max = d;
        }
    }
    visited.remove(start);
    max
}

fn main() {
    let mut map = load_map();
    let starting_y = 0;
    let starting_x = find_empty_point(&map, starting_y);
    let target_y = map.len() - 1;
    let target_x = find_empty_point(&map, target_y);

    let longest_path = find_longest_path(&map, starting_x, starting_y, target_x, target_y);
    println!("Result p1: {}", longest_path);

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let c = map[i][j];
            if c == 'v' || c == '>' || c == '<' || c == '^' {
                map[i][j] = '.';
            }
        }
    }

    let mut graph = build_graph(&map);
    simplify_graph(&mut graph);

    let mut visited = HashSet::new();
    let start = (starting_x, starting_y);
    let end = (target_x, target_y);

    let longest_path = dfs(&graph, &start, &end, 0, &mut visited);
    println!("Result p2: {}", longest_path);
}
