use std::collections::{HashMap, VecDeque};

pub fn solution() -> i32 {
    let (bytes, mut map) = parse_input();

    for byte in bytes.iter().take(1024) {
        map[byte.0][byte.1] = '#';
    }

    let x = dijkstra((0, 0, 0), map);
    println!("{:?}", x.get(&(70, 70)));

    *x.get(&(70, 70)).unwrap()
}

fn dijkstra(state: (usize, usize, i32), map: Vec<Vec<char>>) -> HashMap<(usize, usize), i32> {
    let mut cheapest_paths: HashMap<(usize, usize), i32> = HashMap::new();

    let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
    queue.push_back((state.0, state.1, state.2));
    while !queue.is_empty() {
        let (y, x, score) = queue.pop_front().unwrap();

        if let Some(&prev_score) = cheapest_paths.get(&(y, x)) {
            if prev_score <= score {
                continue;
            }
        }

        cheapest_paths.insert((y, x), score);

        let neighbors = get_valid_neighbors(map.clone(), (y, x, score));
        for neighbor in neighbors {
            queue.push_back(neighbor);
        }
    }

    return cheapest_paths;
}

fn get_valid_neighbors(
    map: Vec<Vec<char>>,
    current_state: (usize, usize, i32),
) -> Vec<(usize, usize, i32)> {
    let (current_y, current_x, current_step_count) = current_state;

    let neighbor_cells: Vec<(usize, usize)> = vec![
        (current_y.wrapping_sub(1), current_x),
        (current_y + 1, current_x),
        (current_y, current_x.wrapping_sub(1)),
        (current_y, current_x + 1),
    ];

    let mut neighbors: Vec<(usize, usize, i32)> = Vec::new();

    for &(neighbor_y, neighbor_x) in &neighbor_cells {
        if neighbor_y >= map.len()
            || neighbor_x >= map[0].len()
            || map[neighbor_y][neighbor_x] == '#'
        {
            continue;
        }

        let new_step_count = current_step_count + 1;

        neighbors.push((neighbor_y, neighbor_x, new_step_count));
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
