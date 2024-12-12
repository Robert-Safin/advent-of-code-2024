use std::collections::HashMap;

pub fn solution() -> i64 {
    let input: String = std::fs::read_to_string("src/inputs/day11.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();

    let stones: Vec<i64> = input.split(" ").map(|n| n.parse().unwrap()).collect();

    let mut stone_counts: HashMap<i64, i64> = HashMap::new();

    for stone in stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..75 {
        let mut new_stone_counts: HashMap<i64, i64> = HashMap::new();

        for (&stone, &count) in &stone_counts {
            if stone == 0 {
                *new_stone_counts.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                let digits = stone.to_string();
                let half_len = digits.len() / 2;
                let left: i64 = digits[..half_len].parse().unwrap();
                let right: i64 = digits[half_len..].parse().unwrap();
                *new_stone_counts.entry(left).or_insert(0) += count;
                *new_stone_counts.entry(right).or_insert(0) += count;
            } else {
                *new_stone_counts.entry(stone * 2024).or_insert(0) += count;
            }
        }

        stone_counts = new_stone_counts;
    }

    println!("Total stones: {}", stone_counts.values().sum::<i64>());
    stone_counts.values().sum()
}
