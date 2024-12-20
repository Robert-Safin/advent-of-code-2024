use std::collections::HashSet;

// const BASELINE_TEST: i32 = 84;

const BASELINE_INPUT: i32 = 9388;

pub fn solution() -> i32 {
    let (map, start, end) = parse_input();
    let mut visited: HashSet<(usize, usize, bool)> = HashSet::new();

    let mut scores: Vec<i32> = Vec::new();

    dfs(
        (start.0, start.1, false, 0),
        &map,
        &mut visited,
        end,
        &mut scores,
    );
    println!("Result: {:?}", scores.len());

    return scores.len() as i32;
}

fn dfs(
    current_position: (usize, usize, bool, i32),
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize, bool)>,
    end: (usize, usize),
    scores: &mut Vec<i32>,
) {
    let (y, x, has_cheated, steps) = current_position;

    if y >= map.len() || x >= map[0].len() {
        return;
    }

    if steps >= BASELINE_INPUT {
        return;
    }

    if (y, x) == end {
        if BASELINE_INPUT - steps >= 100 {
            scores.push(steps);
            println!("{:?}", scores.len());
        }
        return;
    }

    visited.insert((y, x, has_cheated));

    for n in get_neighbors((y, x, has_cheated), map, visited) {
        dfs((n.0, n.1, n.2, steps + 1), map, visited, end, scores);
    }

    visited.remove(&(y, x, has_cheated));
}

fn get_neighbors(
    current_position: (usize, usize, bool),
    map: &Vec<Vec<char>>,
    visited: &HashSet<(usize, usize, bool)>,
) -> Vec<(usize, usize, bool)> {
    let mut neighbors: Vec<(usize, usize, bool)> = Vec::new();
    let (y, x, has_cheated) = current_position;

    let directions = [
        (y.wrapping_sub(1), x, y > 0),
        (y, x.wrapping_sub(1), x > 0),
        (y + 1, x, y < map.len() - 1),
        (y, x + 1, x < map[0].len() - 1),
    ];

    for &(ny, nx, valid) in &directions {
        if valid && map[ny][nx] != '#' && !visited.contains(&(ny, nx, has_cheated)) {
            neighbors.push((ny, nx, has_cheated));
        }
    }

    if !has_cheated {
        for &(ny, nx, valid) in &directions {
            if valid && map[ny][nx] == '#' && !visited.contains(&(ny, nx, true)) {
                neighbors.push((ny, nx, true));
            }
        }
    }

    neighbors
}

fn parse_input() -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let input: String = std::fs::read_to_string("src/inputs/day20.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();
    let mut map = Vec::new();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (line_i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (col_i, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (line_i, col_i);
            }
            if char == 'E' {
                end = (line_i, col_i);
            }
            row.push(char);
        }
        map.push(row);
    }

    (map, start, end)
}
