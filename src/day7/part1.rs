pub fn solution() -> i64 {
    let input: String =
        std::fs::read_to_string("src/inputs/day7.txt").expect("failed to parse input");

    let mut parsed_inputs: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(":").collect();
        let target: i64 = split[0].trim().parse().unwrap();
        let nums: Vec<i64> = split[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        parsed_inputs.push((target, nums));
    }

    let mut total_calibration_result = 0;

    fn evaluate_combinations(
        numbers: &[i64],
        operators: &mut Vec<char>,
        current_index: usize,
        target: i64,
    ) -> bool {
        if current_index == numbers.len() - 1 {
            let mut result = numbers[0];

            for (i, &op) in operators.iter().enumerate() {
                match op {
                    '+' => result += numbers[i + 1],
                    '*' => result *= numbers[i + 1],
                    _ => panic!("At The Disco"),
                }
            }

            return result == target;
        }

        for &op in ['+', '*'].iter() {
            operators[current_index] = op;
            if evaluate_combinations(numbers, operators, current_index + 1, target) {
                return true;
            }
        }

        false
    }

    for (target, numbers) in parsed_inputs {
        let num_count = numbers.len();

        let mut operators = vec![' '; num_count - 1];
        if evaluate_combinations(&numbers, &mut operators, 0, target) {
            total_calibration_result += target;
        }
    }
    println!("{}", total_calibration_result);
    total_calibration_result
}
