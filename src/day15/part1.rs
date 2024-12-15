pub fn solution() -> i32 {
    let (mut map, instructions, mut robot_position) = parse_input();

    for instruction in instructions {
        move_robot(&mut map, &instruction, &mut robot_position);
    }

    let gps = calculate_gps(&map);
    println!("{}", gps);

    gps as i32
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

fn move_robot(map: &mut Vec<Vec<char>>, instruction: &char, robot_position: &mut (usize, usize)) {
    if let Some((row_index, col_index)) = calculate_target_cell(robot_position, instruction) {
        match map[row_index][col_index] {
            '#' => return,
            '.' => move_to_empty_cell(map, robot_position, row_index, col_index),
            'O' => try_push_crate(map, instruction, robot_position, row_index, col_index),
            _ => panic!("Unexpected cell value"),
        }
    } else {
        panic!("Invalid target cell coordinates");
    }
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

fn try_push_crate(
    map: &mut Vec<Vec<char>>,
    instruction: &char,
    robot_position: &mut (usize, usize),
    row_index: usize,
    col_index: usize,
) {
    let (next_row, next_col) = match instruction {
        '^' => (row_index - 1, col_index),
        'v' => (row_index + 1, col_index),
        '<' => (row_index, col_index - 1),
        '>' => (row_index, col_index + 1),
        _ => unreachable!(),
    };

    if map[next_row][next_col] == '.' {
        push_single_crate(
            map,
            robot_position,
            row_index,
            col_index,
            next_row,
            next_col,
        );
    } else if map[next_row][next_col] == 'O' {
        push_multiple_crates(map, instruction, robot_position, row_index, col_index);
    }
}

fn push_single_crate(
    map: &mut Vec<Vec<char>>,
    robot_position: &mut (usize, usize),
    row_index: usize,
    col_index: usize,
    next_row: usize,
    next_col: usize,
) {
    map[next_row][next_col] = 'O';
    map[row_index][col_index] = '@';
    map[robot_position.0][robot_position.1] = '.';
    robot_position.0 = row_index;
    robot_position.1 = col_index;
}

fn push_multiple_crates(
    map: &mut Vec<Vec<char>>,
    instruction: &char,
    robot_position: &mut (usize, usize),
    row_index: usize,
    col_index: usize,
) {
    let mut crate_positions = vec![(row_index, col_index)];
    let mut next_crate_row = match instruction {
        '^' => row_index - 1,
        'v' => row_index + 1,
        '<' => row_index,
        '>' => row_index,
        _ => unreachable!(),
    };
    let mut next_crate_col = match instruction {
        '^' | 'v' => col_index,
        '<' => col_index - 1,
        '>' => col_index + 1,
        _ => unreachable!(),
    };

    while map[next_crate_row][next_crate_col] == 'O' {
        crate_positions.push((next_crate_row, next_crate_col));

        match instruction {
            '^' => next_crate_row -= 1,
            'v' => next_crate_row += 1,
            '<' => next_crate_col -= 1,
            '>' => next_crate_col += 1,
            _ => unreachable!(),
        }
    }

    if map[next_crate_row][next_crate_col] == '.' {
        for &(crate_row, crate_col) in crate_positions.iter().rev() {
            let (new_row, new_col) = match instruction {
                '^' => (crate_row - 1, crate_col),
                'v' => (crate_row + 1, crate_col),
                '<' => (crate_row, crate_col - 1),
                '>' => (crate_row, crate_col + 1),
                _ => unreachable!(),
            };
            map[new_row][new_col] = 'O';
            map[crate_row][crate_col] = '.';
        }

        map[row_index][col_index] = '@';
        map[robot_position.0][robot_position.1] = '.';
        robot_position.0 = row_index;
        robot_position.1 = col_index;
    }
}
