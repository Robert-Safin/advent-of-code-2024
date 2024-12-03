pub fn solution() -> i32 {
    let input: String =
        std::fs::read_to_string("src/inputs/day3.txt").expect("failed to parse input");

    let mut sum = 0;
    let input_len = input.len();

    let mut valid: Option<bool> = None;

    for (i, char) in input.chars().enumerate() {
        if char == 'd' && &input[i..=i + 3] == "do()" {
            valid = Some(true);
        }

        if char == 'd' && &input[i..=i + 6] == "don't()" {
            valid = Some(false);
        }

        if char == '(' && i >= 3 && i < input_len - 9 {
            let mul_slice = &input[i - 3..i];
            if mul_slice == "mul" {
                let slice = &input[i..];
                if let Some(end_idx) = slice.find(')') {
                    let full_instr = &slice[..=end_idx];
                    if full_instr.contains(",") {
                        let split: Vec<&str> =
                            full_instr[1..full_instr.len() - 1].split(',').collect();
                        if split.len() == 2 {
                            let left = split[0].trim().parse::<i32>();
                            let right = split[1].trim().parse::<i32>();

                            if let (Ok(left), Ok(right)) = (left, right) {
                                match valid {
                                    Some(true) => sum += left * right,
                                    Some(false) => (),
                                    None => sum += left * right,
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", sum);
    sum
}
