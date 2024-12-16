pub fn solution() -> i32 {
    let (mut map, instructions, mut robot_position) = parse_input();

    for instruction in instructions {
        move_robot(&mut map, &instruction, &mut robot_position);
    }

    let gps = calculate_gps(&map);
    println!("{}", gps);

    gps as i32
}

fn move_robot(map: &mut Vec<Vec<char>>, instruction: &char, robot_position: &mut (usize, usize)) {
    if let Some((row_index, col_index)) = calculate_target_cell(robot_position, instruction) {
        match map[row_index][col_index] {
            '#' => return,
            '.' => move_to_empty_cell(map, robot_position, row_index, col_index),
            'O' => try_push_crate(map, instruction, robot_position, row_index, col_index),
            _ => panic!("Something funny happened"),
        }
    } else {
        panic!("Something funny happened");
    }
}

fn try_push_crate(
    map: &mut Vec<Vec<char>>,
    instruction: &char,
    robot_position: &mut (usize, usize),
    row_index: usize,
    col_index: usize,
) {
    match instruction {
        '^' => {
            let mut target_col_slice = map.iter().map(|row| row[col_index]).collect::<Vec<char>>();
            let new_robot_y = push_left_or_up(&mut target_col_slice);
            for (row_i, row) in map.iter_mut().enumerate() {
                row[col_index] = target_col_slice[row_i];
            }
            robot_position.0 = new_robot_y;
        }
        'v' => {
            let mut target_col_slice = map.iter().map(|row| row[col_index]).collect::<Vec<char>>();
            let new_robot_y = push_right_or_down(&mut target_col_slice);
            for (row_i, row) in map.iter_mut().enumerate() {
                row[col_index] = target_col_slice[row_i];
            }
            robot_position.0 = new_robot_y;
        }
        '<' => {
            let mut target_row_slice = map[row_index].clone();
            let new_robot_x = push_left_or_up(&mut target_row_slice);
            map[row_index] = target_row_slice;
            robot_position.1 = new_robot_x;
        }
        '>' => {
            let mut target_row_slice = map[row_index].clone();
            let new_robot_x = push_right_or_down(&mut target_row_slice);
            map[row_index] = target_row_slice;
            robot_position.1 = new_robot_x;
        }
        _ => {
            panic!("Something funny happened");
        }
    }
}

fn move_to_empty_cell(
    map: &mut Vec<Vec<char>>,
    robot_position: &mut (usize, usize),
    row_index: usize,
    col_index: usize,
) {
    map[row_index][col_index] = '@';
    map[robot_position.0][robot_position.1] = '.';
    robot_position.0 = row_index;
    robot_position.1 = col_index;
}

fn calculate_gps(map: &Vec<Vec<char>>) -> i32 {
    let mut total_gps = 0;

    for (row_i, row) in map.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col == 'O' {
                total_gps += 100 * row_i + col_i;
            }
        }
    }
    return total_gps as i32;
}

fn calculate_target_cell(
    robot_position: &(usize, usize),
    instruction: &char,
) -> Option<(usize, usize)> {
    match instruction {
        '^' => Some((robot_position.0 - 1, robot_position.1)),
        'v' => Some((robot_position.0 + 1, robot_position.1)),
        '<' => Some((robot_position.0, robot_position.1 - 1)),
        '>' => Some((robot_position.0, robot_position.1 + 1)),
        _ => None,
    }
}

fn parse_input() -> (Vec<Vec<char>>, Vec<char>, (usize, usize)) {
    let input: String = std::fs::read_to_string("src/inputs/day15.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();

    let split = input.split("\n\n").collect::<Vec<&str>>();

    let mut robot_position: (usize, usize) = (0, 0);

    let mut map: Vec<Vec<char>> = Vec::new();
    for (row_i, line) in split[0].lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (col_i, cell) in line.chars().enumerate() {
            if cell == '@' {
                robot_position = (row_i, col_i);
            }
            row.push(cell);
        }
        map.push(row);
    }
    let mut instructions: Vec<char> = Vec::new();
    for line in split[1].lines() {
        for char in line.chars() {
            instructions.push(char);
        }
    }

    return (map, instructions, robot_position);
}

fn push_right_or_down(vec: &mut Vec<char>) -> usize {
    let robot_index = vec.iter().position(|&x| x == '@').unwrap();
    let mut current_index = robot_index + 1;

    while current_index < vec.len() - 1 && vec[current_index] != '.' {
        if vec[current_index as usize] == '#' {
            return robot_index;
        }
        current_index += 1;
    }

    if current_index == vec.len() - 1 {
        return robot_index;
    }
    while current_index != robot_index {
        vec.swap(current_index - 1, current_index);
        current_index -= 1
    }
    return vec.iter().position(|&x| x == '@').unwrap();
}

fn push_left_or_up(vec: &mut Vec<char>) -> usize {
    let robot_index = vec.iter().position(|&x| x == '@').unwrap();
    let mut current_index = robot_index - 1;

    while current_index > 0 && vec[current_index] != '.' {
        if vec[current_index as usize] == '#' {
            return robot_index;
        }
        current_index -= 1;
    }

    if current_index == 0 {
        return robot_index;
    }

    while current_index != robot_index {
        vec.swap(current_index + 1, current_index);
        current_index += 1
    }

    return vec.iter().position(|&x| x == '@').unwrap();
}
