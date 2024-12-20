use std::collections::{HashMap, VecDeque};

pub fn solution() -> i32 {
    let (map, end) = parse_input();

    let dijkstra = dijkstra((end.0, end.1, 0), map.clone());

    let jump_map = parse_dijkstra(&dijkstra, map.clone());

    let mut short_cuts: Vec<i32> = Vec::new();

    for ((y, x), distance_to_end) in dijkstra.iter() {
        let targets = create_jump_targets(&jump_map, (*y, *x));
        for target in targets {
            let current_distance = distance_to_end;
            let new_distance = jump_map[target.0][target.1];
            let cheat_duration = target.2 as i32;

            let diff = current_distance - new_distance - cheat_duration;

            if diff > 0 {
                short_cuts.push(diff);
            }
        }
    }

    let count = short_cuts.iter().filter(|&&x| x >= 100).count() as i32;

    println!("{:?}", count);
    count
}

fn create_jump_targets(
    map: &Vec<Vec<i32>>,
    current_position: (usize, usize),
) -> Vec<(usize, usize, usize)> {
    let mut jump_targets: Vec<(usize, usize, usize)> = Vec::new();
    let (y, x) = current_position;

    for dy in -(20 as isize)..=(20 as isize) {
        for dx in -(20 as isize)..=(20 as isize) {
            let target_y = y.wrapping_add(dy as usize);
            let target_x = x.wrapping_add(dx as usize);

            let distance = dy.abs() as usize + dx.abs() as usize;

            if target_y < map.len()
                && target_x < map[0].len()
                && map[target_y][target_x] != -1
                && distance > 0
                && distance <= 20
            {
                jump_targets.push((target_y, target_x, distance));
            }
        }
    }

    jump_targets
}

fn parse_dijkstra(
    cheapest_paths: &HashMap<(usize, usize), i32>,
    map: Vec<Vec<char>>,
) -> Vec<Vec<i32>> {
    let mut jump_map: Vec<Vec<i32>> = Vec::new();
    for (row_i, row) in map.iter().enumerate() {
        let mut new_row: Vec<i32> = Vec::new();
        for (col_i, cell) in row.iter().enumerate() {
            if *cell == '#' {
                new_row.push(-1);
            } else {
                let dist = cheapest_paths.get(&(row_i, col_i)).unwrap();
                new_row.push(*dist);
            }
        }
        jump_map.push(new_row);
    }

    jump_map
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

fn parse_input() -> (Vec<Vec<char>>, (usize, usize)) {
    let input: String = std::fs::read_to_string("src/inputs/day20.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();
    let mut map = Vec::new();

    let mut end = (0, 0);

    for (line_i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (col_i, char) in line.chars().enumerate() {
            if char == 'E' {
                end = (line_i, col_i);
            }
            row.push(char);
        }
        map.push(row);
    }

    (map, end)
}
