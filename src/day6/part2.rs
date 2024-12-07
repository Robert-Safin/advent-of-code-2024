use std::collections::HashSet;

pub fn solution() -> usize {
    let input: String =
        std::fs::read_to_string("src/inputs/day6.txt").expect("failed to parse input");

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum Direction {
        Up,
        Right,
        Down,
        Left,
        None,
    }

    let mut guard_position: (i32, i32, Direction) = (0, 0, Direction::None);

    let mut matrix: Vec<Vec<char>> = Vec::new();
    input.lines().enumerate().for_each(|(row_i, line)| {
        let mut row: Vec<char> = Vec::new();
        line.chars().enumerate().for_each(|(col_i, char)| {
            if char == '^' {
                guard_position = (row_i as i32, col_i as i32, Direction::Up)
            }
            row.push(char);
        });
        matrix.push(row);
    });

    let mut possible_positions: Vec<(usize, usize)> = vec![];

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == '.' {
                matrix[row][col] = '#';

                let mut visited: HashSet<(i32, i32, Direction)> = HashSet::new();
                let mut current_position = guard_position.clone();
                let mut is_loop = false;

                loop {
                    if !visited.insert((
                        current_position.0,
                        current_position.1,
                        current_position.2.clone(),
                    )) {
                        is_loop = true;
                        break;
                    }

                    match current_position.2 {
                        Direction::Up => {
                            if current_position.0 > 0 {
                                let new_row = current_position.0 - 1;
                                if matrix[new_row as usize][current_position.1 as usize] == '#' {
                                    current_position.2 = Direction::Right;
                                } else {
                                    current_position.0 -= 1;
                                }
                            } else {
                                break;
                            }
                        }
                        Direction::Right => {
                            let new_col = current_position.1 + 1;
                            if new_col < matrix[0].len() as i32 {
                                if matrix[current_position.0 as usize][new_col as usize] == '#' {
                                    current_position.2 = Direction::Down;
                                } else {
                                    current_position.1 += 1;
                                }
                            } else {
                                break;
                            }
                        }
                        Direction::Down => {
                            let new_row = current_position.0 + 1;
                            if new_row < matrix.len() as i32 {
                                if matrix[new_row as usize][current_position.1 as usize] == '#' {
                                    current_position.2 = Direction::Left;
                                } else {
                                    current_position.0 += 1;
                                }
                            } else {
                                break;
                            }
                        }
                        Direction::Left => {
                            if current_position.1 > 0 {
                                let new_col = current_position.1 - 1;
                                if matrix[current_position.0 as usize][new_col as usize] == '#' {
                                    current_position.2 = Direction::Up;
                                } else {
                                    current_position.1 -= 1;
                                }
                            } else {
                                break;
                            }
                        }
                        _ => (),
                    }
                }

                if is_loop {
                    possible_positions.push((row, col));
                }

                matrix[row][col] = '.';
            }
        }
    }

    println!("{:?}", possible_positions.len());
    possible_positions.len()
}
