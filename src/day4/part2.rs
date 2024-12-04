pub fn solution() -> i32 {
    let input: String =
        std::fs::read_to_string("src/inputs/day4.txt").expect("failed to parse input");

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        matrix.push(row);
    }

    fn is_in_bounds(matrix: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
        row >= 0 && row < matrix.len() as i32 && col >= 0 && col < matrix[0].len() as i32
    }

    let mut sum = 0;

    for (row_i, row) in matrix.iter().enumerate() {
        for (col_i, char) in row.iter().enumerate() {
            if char == &'A' {
                let top_left_index = (row_i as i32 - 1, col_i as i32 - 1);
                let top_right_index = (row_i as i32 - 1, col_i as i32 + 1);
                let bottom_left_index = (row_i as i32 + 1, col_i as i32 - 1);
                let bottom_right_index = (row_i as i32 + 1, col_i as i32 + 1);
                if is_in_bounds(&matrix, top_left_index.0, top_left_index.1)
                    && is_in_bounds(&matrix, top_right_index.0, top_right_index.1)
                    && is_in_bounds(&matrix, bottom_left_index.0, bottom_left_index.1)
                    && is_in_bounds(&matrix, bottom_right_index.0, bottom_right_index.1)
                {
                    let mut positioned_values: Vec<char> = Vec::new();
                    positioned_values
                        .push(matrix[top_left_index.0 as usize][top_left_index.1 as usize]);
                    positioned_values
                        .push(matrix[top_right_index.0 as usize][top_right_index.1 as usize]);
                    positioned_values
                        .push(matrix[bottom_left_index.0 as usize][bottom_left_index.1 as usize]);
                    positioned_values
                        .push(matrix[bottom_right_index.0 as usize][bottom_right_index.1 as usize]);
                    let variations = vec![
                        vec!['M', 'M', 'S', 'S'],
                        vec!['S', 'S', 'M', 'M'],
                        vec!['M', 'S', 'M', 'S'],
                        vec!['S', 'M', 'S', 'M'],
                    ];

                    if variations.contains(&positioned_values) {
                        sum += 1;
                    }
                }
            }
        }
    }

    println!("sum: {}", sum);
    sum
}
