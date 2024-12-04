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
            if char == &'X' {
                // check right
                let right_end_index = col_i + 3;
                if is_in_bounds(&matrix, row_i as i32, right_end_index as i32) {
                    let str = &matrix[row_i][col_i..right_end_index + 1]
                        .iter()
                        .collect::<String>();
                    if str == "XMAS" || str == "SAMX" {
                        sum += 1;
                    }
                }
                // check down
                let down_end_index = row_i + 3;
                if is_in_bounds(&matrix, down_end_index as i32, col_i as i32) {
                    let mut str = String::new();
                    for i in row_i..down_end_index + 1 {
                        str.push(matrix[i][col_i]);
                    }
                    if str == "XMAS" || str == "SAMX" {
                        sum += 1;
                    }
                }
                // check left
                let left_end_index = (col_i as i32) - 3;
                if is_in_bounds(&matrix, row_i as i32, left_end_index as i32) {
                    let str = &matrix[row_i][(left_end_index as usize)..col_i + 1]
                        .iter()
                        .collect::<String>();
                    if str == "XMAS" || str == "SAMX" {
                        sum += 1;
                    }
                }

                // check up
                let up_end_index = (row_i as i32) - 3;
                if is_in_bounds(&matrix, up_end_index as i32, col_i as i32) {
                    let mut str = String::new();
                    for i in (up_end_index..(row_i as i32) + 1).rev() {
                        str.push(matrix[i as usize][col_i]);
                    }
                    if str == "XMAS" || str == "SAMX" {
                        sum += 1;
                    }
                }
                // check diagonal up-right
                let up_end_index = (row_i as i32) - 3;
                let right_end_index = (col_i as i32) + 3;
                if is_in_bounds(&matrix, up_end_index, right_end_index) {
                    let str = [
                        matrix[row_i][col_i],
                        matrix[row_i - 1][col_i + 1],
                        matrix[row_i - 2][col_i + 2],
                        matrix[row_i - 3][col_i + 3],
                    ]
                    .iter()
                    .collect::<String>();
                    if str == "XMAS" || str == "SAMX" {
                        sum += 1;
                    }
                }

                // check diagonal down-right
                let down_end_index = (row_i as i32) + 3;
                let right_end_index = (col_i as i32) + 3;
                if is_in_bounds(&matrix, down_end_index, right_end_index) {
                    let str = [
                        matrix[row_i][col_i],
                        matrix[row_i + 1][col_i + 1],
                        matrix[row_i + 2][col_i + 2],
                        matrix[row_i + 3][col_i + 3],
                    ]
                    .iter()
                    .collect::<String>();
                    if str == "XMAS" || str == "SAMX" {
                        sum += 1;
                    }
                }

                // check diagonal down-left
                let down_end_index = (row_i as i32) + 3;
                let left_end_index = (col_i as i32) - 3;
                if is_in_bounds(&matrix, down_end_index, left_end_index) {
                    let str = [
                        matrix[row_i][col_i],
                        matrix[row_i + 1][col_i - 1],
                        matrix[row_i + 2][col_i - 2],
                        matrix[row_i + 3][col_i - 3],
                    ]
                    .iter()
                    .collect::<String>();

                    if str == "XMAS" || str == "SAMX" {
                        sum += 1;
                    }
                }

                // check diagonal up-left
                let up_end_index = (row_i as i32) - 3;
                let left_end_index = (col_i as i32) - 3;
                if is_in_bounds(&matrix, up_end_index, left_end_index) {
                    let str = [
                        matrix[row_i][col_i],
                        matrix[row_i - 1][col_i - 1],
                        matrix[row_i - 2][col_i - 2],
                        matrix[row_i - 3][col_i - 3],
                    ]
                    .iter()
                    .collect::<String>();
                    if str == "XMAS" || str == "SAMX" {
                        sum += 1;
                    }
                }
            }
        }
    }

    println!("sum: {}", sum);
    sum
}
