pub fn solution() -> i32 {
    let input: String =
        std::fs::read_to_string("src/inputs/day1.txt").expect("failed to parse input");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut lines: Vec<&str> = input.split("\n").collect();
    lines.pop();
    for line in lines {
        let split = line.split_ascii_whitespace().collect::<Vec<&str>>();
        left.push(
            split[0]
                .parse::<i32>()
                .expect("failed to parse split into i32"),
        );
        right.push(
            split[1]
                .parse::<i32>()
                .expect("failed to parse split into i32"),
        );
    }

    left.sort();
    right.sort();

    let mut sum: i32 = 0;

    for (index, value) in left.iter().enumerate() {
        let diff = right[index] - value;
        let abs_diff = diff.abs();
        sum += abs_diff;
    }
    println!("{}", sum);
    return sum;
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test() {
//         assert_eq!(solution(), 54916)
//     }
// }
