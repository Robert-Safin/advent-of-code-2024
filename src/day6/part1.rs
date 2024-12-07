pub fn solution() -> i32 {
    let input: String =
        std::fs::read_to_string("src/inputs/day6.txt").expect("failed to parse input");

    #[derive(Debug)]
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

    let mut visited: Vec<(i32, i32)> = Vec::new();

    loop {
        match guard_position.2 {
            Direction::Up => {
                if guard_position.0 > 0 {
                    let new_row = guard_position.0 - 1;
                    if let Some(cell_value) = matrix
                        .get(new_row as usize)
                        .and_then(|row| row.get(guard_position.1 as usize))
                    {
                        if *cell_value == '#' {
                            visited.push((guard_position.0, guard_position.1));

                            guard_position.2 = Direction::Right;
                        } else {
                            visited.push((guard_position.0, guard_position.1));

                            guard_position.0 -= 1;
                        }
                    }
                } else {
                    visited.push((guard_position.0, guard_position.1));
                    break;
                }
            }
            Direction::Right => {
                let new_col = guard_position.1 + 1;
                if new_col < matrix[0].len() as i32 {
                    let cell_value = matrix[guard_position.0 as usize][new_col as usize];
                    if cell_value == '#' {
                        visited.push((guard_position.0, guard_position.1));

                        guard_position.2 = Direction::Down;
                    } else {
                        visited.push((guard_position.0, guard_position.1));

                        guard_position.1 += 1;
                    }
                } else {
                    visited.push((guard_position.0, guard_position.1));
                    break;
                }
            }
            Direction::Down => {
                let new_row = guard_position.0 + 1;
                if new_row < matrix.len() as i32 {
                    let cell_value = matrix[new_row as usize][guard_position.1 as usize];
                    if cell_value == '#' {
                        visited.push((guard_position.0, guard_position.1));

                        guard_position.2 = Direction::Left;
                    } else {
                        visited.push((guard_position.0, guard_position.1));

                        guard_position.0 += 1;
                    }
                } else {
                    visited.push((guard_position.0, guard_position.1));
                    break;
                }
            }
            Direction::Left => {
                if guard_position.1 > 0 {
                    let new_col = guard_position.1 - 1;
                    if let Some(cell_value) = matrix
                        .get(guard_position.0 as usize)
                        .and_then(|row| row.get(new_col as usize))
                    {
                        if *cell_value == '#' {
                            visited.push((guard_position.0, guard_position.1));

                            guard_position.2 = Direction::Up;
                        } else {
                            visited.push((guard_position.0, guard_position.1));

                            guard_position.1 -= 1;
                        }
                    } else {
                        visited.push((guard_position.0, guard_position.1));
                        break;
                    }
                }
            }
            _ => (),
        }
    }

    visited.sort();
    visited.dedup();
    println!("{:?}", visited.len());
    return visited.len() as i32;
}
