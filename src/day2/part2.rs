pub fn solution() -> i32 {
    let input: String =
        std::fs::read_to_string("src/inputs/day2.txt").expect("failed to parse input");

    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<i32> = Vec::new();
        for num in line.split_whitespace() {
            row.push(num.parse::<i32>().unwrap());
        }
        matrix.push(row);
    }

    #[derive(PartialEq)]
    enum Direction {
        Increasing,
        Decreasing,
        None,
    }

    let mut count = 0;

    fn is_safe(row: &[i32]) -> bool {
        let mut direction = Direction::None;

        for window in row.windows(2) {
            let first = window[0];
            let second = window[1];
            let diff = second - first;

            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }

            if direction == Direction::None {
                if diff > 0 {
                    direction = Direction::Increasing;
                } else if diff < 0 {
                    direction = Direction::Decreasing;
                }
            }

            if (direction == Direction::Increasing && diff < 0)
                || (direction == Direction::Decreasing && diff > 0)
            {
                return false;
            }
        }

        true
    }

    for row in matrix {
        if is_safe(&row) {
            count += 1;
        } else {
            let mut dampened_safe = false;
            for i in 0..row.len() {
                let mut temp_row = row.clone();
                temp_row.remove(i);
                if is_safe(&temp_row) {
                    dampened_safe = true;
                    break;
                }
            }
            if dampened_safe {
                count += 1;
            }
        }
    }

    println!("{}", count);
    count
}
