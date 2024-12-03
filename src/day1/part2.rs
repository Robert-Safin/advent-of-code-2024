pub fn solution() -> i32 {
    let input: String =
        std::fs::read_to_string("src/inputs/day1.txt").expect("failed to parse input");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let split = line.split_ascii_whitespace().collect::<Vec<&str>>();
        left.push(
            split
                .first()
                .expect("failed to chunk")
                .parse::<i32>()
                .expect("failed to parse split into i32"),
        );
        right.push(
            split
                .last()
                .expect("failed to chunk")
                .parse::<i32>()
                .expect("failed to parse split into i32"),
        );
    }

    let mut similarity_score = 0;

    for value in left.iter() {
        let right_occurrence = right.iter().filter(|v| *v == value).count() as i32;
        similarity_score += right_occurrence * value;
    }
    println!("Similarity score: {}", similarity_score);
    return similarity_score;
}
