use std::collections::{HashMap, VecDeque};

pub fn solution() -> i32 {
    let (bytes, _) = parse_input();

    for i in 1024..bytes.len() {
        let (_, mut map) = parse_input();

        for byte in bytes.iter().take(i) {
            map[byte.0][byte.1] = '#';
        }

        if !is_reachable((0, 0), (70, 70), map) {
            println!("{:?}", bytes[i - 1]);
            return (i as i32) - 1024;
        }
    }

    0
}

fn is_reachable(start: (usize, usize), end: (usize, usize), map: Vec<Vec<char>>) -> bool {
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    queue.push_back(start);

    while let Some((y, x)) = queue.pop_front() {
        if (y, x) == end {
            return true;
        }

        if visited.get(&(y, x)).is_some() {
            continue;
        }
        visited.insert((y, x), true);

        let neighbors = get_valid_neighbors(map.clone(), (y, x));
        for neighbor in neighbors {
            queue.push_back(neighbor);
        }
    }

    false
}

fn get_valid_neighbors(map: Vec<Vec<char>>, current: (usize, usize)) -> Vec<(usize, usize)> {
    let (current_y, current_x) = current;

    let neighbor_cells: Vec<(usize, usize)> = vec![
        (current_y.wrapping_sub(1), current_x),
        (current_y + 1, current_x),
        (current_y, current_x.wrapping_sub(1)),
        (current_y, current_x + 1),
    ];

    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    for &(neighbor_y, neighbor_x) in &neighbor_cells {
        if neighbor_y >= map.len()
            || neighbor_x >= map[0].len()
            || map[neighbor_y][neighbor_x] == '#'
        {
            continue;
        }

        neighbors.push((neighbor_y, neighbor_x));
    }

    neighbors
}

fn parse_input() -> (Vec<(usize, usize)>, Vec<Vec<char>>) {
    let input: String = std::fs::read_to_string("src/inputs/day18.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();

    let mut bytes: Vec<(usize, usize)> = Vec::new();
    for line in input.lines() {
        let split: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
        let byte = (
            split[0].parse::<usize>().unwrap(),
            split[1].parse::<usize>().unwrap(),
        );
        bytes.push(byte);
    }
    let map: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];

    (bytes, map)
}
