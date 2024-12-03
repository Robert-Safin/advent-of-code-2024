pub fn solution() -> i32 {
    let input: String =
        std::fs::read_to_string("src/inputs/day2.txt").expect("failed to parse input");

    let mut matrix: Vec<Vec<i32>> = Vec::new();

    // Parse the input into a matrix of integers
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

    for row in matrix {
        let mut direction = Direction::None;
        let mut is_safe = true;

        for window in row.windows(2) {
            let first = window[0];
            let second = window[1];
            let diff = second - first;

            if diff.abs() < 1 || diff.abs() > 3 {
                is_safe = false;
                break;
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
                is_safe = false;
                break;
            }
        }

        if is_safe {
            count += 1;
        }
    }

    println!("{}", count);
    count
}
