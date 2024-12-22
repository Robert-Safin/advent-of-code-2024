//["029A", "980A", "179A", "456A", "379A"]
use std::collections::{HashSet, VecDeque};

pub fn solution() -> i32 {
    let input = parse_input();

    let numeric_keypad: Vec<Vec<char>> = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ];

    let directional_keypad: Vec<Vec<char>> = vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']];

    let mut human_pointer = (0, 2);
    let mut robot_one_pointer = (0, 2);
    let mut robot_two_pointer = (0, 2);
    let mut robot_three_pointer = (3, 2);

    for code in input.iter().take(1) {
        let mut inputs: Vec<char> = Vec::new();

        compute_char(
            code.to_string(),
            &mut human_pointer,
            &mut robot_one_pointer,
            &mut robot_two_pointer,
            &mut robot_three_pointer,
            &numeric_keypad,
            &directional_keypad,
        );
    }

    0
}

fn compute_char(
    code: String,
    human_pointer: &mut (usize, usize),
    robot_one_pointer: &mut (usize, usize),
    robot_two_pointer: &mut (usize, usize),
    robot_three_pointer: &mut (usize, usize),
    numeric_keypad: &Vec<Vec<char>>,
    directional_keypad: &Vec<Vec<char>>,
) {
    let mut global_todos: Vec<char> = Vec::new();
    // robot 3
    for c in code.chars().take(1) {
        let target_coords = find_coords(c, numeric_keypad);
        let path = dfs_path(&robot_three_pointer, &target_coords, numeric_keypad);
        let mut todos: Vec<char> = vec![];
        let moves = map_path_to_moves(path);
        if moves.len() > 0 {
            todos = moves;
        }
        *robot_three_pointer = target_coords;
        println!("robot 3 pointer {:?}", &robot_three_pointer);
        println!("robot 3 todos {:?}", &todos);
        println!("-----------");

        //robot 2
        for todo in todos {
            let target_coords = find_coords(todo, directional_keypad);
            let path = dfs_path(&robot_two_pointer, &target_coords, directional_keypad);
            // let mut todos: Vec<char> = vec![];
            let mut moves = map_path_to_moves(path);
            moves.push('A');
            *robot_two_pointer = target_coords;
            println!("robot 2 pointer {:?}", &robot_two_pointer);
            println!("robot 2 todos {:?}", &moves);
            println!("-----------");
          }
        //     // robot 1
        //     for todo in todos {
        //         let target_coords = find_coords(todo, directional_keypad);
        //         let path = dfs_path(&robot_one_pointer, &target_coords, directional_keypad);
        //         let mut todos: Vec<char> = vec![];
        //         let moves = map_path_to_moves(path);
        //         if moves.len() > 0 {
        //             todos = moves;
        //         }
        //         *robot_one_pointer = target_coords;
        //         global_todos.push(todo);
        //         //println!("robot 1 pointer {:?}", &robot_one_pointer);
        //         //println!("robot 1 todos {:?}", &todos);
        //         //println!("-----------");

        //         for todo in todos {
        //             let target_coords = find_coords(todo, directional_keypad);
        //             let path = dfs_path(&human_pointer, &target_coords, directional_keypad);
        //             let mut todos: Vec<char> = vec![];
        //             let moves = map_path_to_moves(path);
        //             if moves.len() > 0 {
        //                 todos = moves;
        //             }
        //             *human_pointer = target_coords;
        //             global_todos.push(todo);
        //             //println!("human pointer {:?}", &human_pointer);
        //             //println!("human todos {:?}", &todos);
        //             //println!("-----------");
        //         }
        //     }
        // }

    }
    println!("{:?}", global_todos);
}

fn map_path_to_moves(path: Vec<(usize, usize)>) -> Vec<char> {
    let mut moves: Vec<char> = Vec::new();

    for pair in path.windows(2) {
        let (y1, x1) = pair[0];
        let (y2, x2) = pair[1];

        if y1 == y2 {
            if x1 < x2 {
                moves.push('>');
            } else {
                moves.push('<');
            }
        } else {
            if y1 < y2 {
                moves.push('v');
            } else {
                moves.push('^');
            }
        }
    }

    moves
}

fn dfs_path(
    start: &(usize, usize),
    end: &(usize, usize),
    keypad: &Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    let mut queue: VecDeque<Vec<(usize, usize)>> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back(vec![*start]);
    visited.insert(*start);

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some(path) = queue.pop_front() {
        let current_position = *path.last().unwrap();

        if current_position == *end {
            return path;
        }

        for &(dy, dx) in &directions {
            let new_y = current_position.0 as isize + dy;
            let new_x = current_position.1 as isize + dx;

            if new_y >= 0
                && new_x >= 0
                && (new_y as usize) < keypad.len()
                && (new_x as usize) < keypad[0].len()
                && keypad[new_y as usize][new_x as usize] != '#'
                && !visited.contains(&(new_y as usize, new_x as usize))
            {
                let mut new_path = path.clone();
                new_path.push((new_y as usize, new_x as usize));
                queue.push_back(new_path);
                visited.insert((new_y as usize, new_x as usize));
            }
        }
    }

    Vec::new()
}

fn find_coords(char: char, numeric_keypad: &Vec<Vec<char>>) -> (usize, usize) {
    for (row_i, row) in numeric_keypad.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col == char {
                return (row_i, col_i);
            }
        }
    }

    unreachable!()
}

fn parse_input() -> Vec<String> {
    let input: Vec<String> = std::fs::read_to_string("src/inputs/day21.txt")
        .expect("Can't find input file")
        .lines()
        .map(|line| line.to_string())
        .collect();

    input
}
