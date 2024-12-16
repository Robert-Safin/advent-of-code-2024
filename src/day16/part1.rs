use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solution() -> i64 {
    let input: String = std::fs::read_to_string("src/inputs/day16.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let reindeer_position = map
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, &x)| if x == 'S' { Some((i, j)) } else { None })
        })
        .unwrap();

    let end_position = map
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, &x)| if x == 'E' { Some((i, j)) } else { None })
        })
        .unwrap();

    let mut cheapest_paths: HashMap<(usize, usize), i32> = HashMap::new();

    let mut queue: VecDeque<(usize, usize, Direction, i32)> = VecDeque::new();
    queue.push_back((
        reindeer_position.0,
        reindeer_position.1,
        Direction::Right,
        0,
    ));

    while !queue.is_empty() {
        let (y, x, direction, score) = queue.pop_front().unwrap();

        if let Some(&prev_score) = cheapest_paths.get(&(y, x)) {
            if prev_score <= score {
                continue;
            }
        }

        cheapest_paths.insert((y, x), score);

        let neighbors = get_valid_neighbors(map.clone(), (y, x, direction, score));
        for neighbor in neighbors {
            queue.push_back(neighbor);
        }
    }

    let mut min_score = i32::MAX;

    if let Some(&score) = cheapest_paths.get(&(end_position)) {
        min_score = min_score.min(score);
    }

    if min_score == i32::MAX {
        panic!("No path found to the end position.");
    }
    println!("{}", min_score);
    min_score as i64
}

fn get_valid_neighbors(
    map: Vec<Vec<char>>,
    current_state: (usize, usize, Direction, i32),
) -> Vec<(usize, usize, Direction, i32)> {
    let (current_y, current_x, current_direction, current_score) = current_state;

    let neighbor_cells: Vec<(usize, usize, Direction)> = vec![
        (current_y.wrapping_sub(1), current_x, Direction::Up),
        (current_y + 1, current_x, Direction::Down),
        (current_y, current_x.wrapping_sub(1), Direction::Left),
        (current_y, current_x + 1, Direction::Right),
    ];

    let mut neighbors: Vec<(usize, usize, Direction, i32)> = Vec::new();

    for &(neighbor_y, neighbor_x, new_direction) in &neighbor_cells {
        if neighbor_y >= map.len()
            || neighbor_x >= map[0].len()
            || map[neighbor_y][neighbor_x] == '#'
        {
            continue;
        }

        let rotation_cost = if new_direction == current_direction {
            0
        } else {
            1000
        };
        let move_cost = 1;
        let total_cost = current_score + rotation_cost + move_cost;

        neighbors.push((neighbor_y, neighbor_x, new_direction, total_cost));
    }

    neighbors
}
