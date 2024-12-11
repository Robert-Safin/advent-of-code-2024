use std::collections::HashMap;

pub fn solution() -> i64 {
    let input: String = std::fs::read_to_string("src/inputs/day11.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();

    let stones: Vec<i64> = input.split(" ").map(|x| x.parse().unwrap()).collect();

    let mut cache: HashMap<i64, i64> = stones.into_iter().fold(HashMap::new(), |mut acc, stone| {
        *acc.entry(stone).or_insert(0) += 1;
        acc
    });

    for _ in 0..75 {
        cache = cache
            .into_iter()
            .fold(HashMap::new(), |mut new_cache, (stone, count)| {
                match stone {
                    0 => {
                        *new_cache.entry(1).or_insert(0) += count;
                    }
                    _ if stone.to_string().len() % 2 == 0 => {
                        let digits = stone.to_string();
                        let mid = digits.len() / 2;
                        let left: i64 = digits[..mid].parse().unwrap();
                        let right: i64 = digits[mid..].parse().unwrap();
                        *new_cache.entry(left).or_insert(0) += count;
                        *new_cache.entry(right).or_insert(0) += count;
                    }
                    _ => {
                        *new_cache.entry(stone * 2024).or_insert(0) += count;
                    }
                }
                new_cache
            });
    }
    let out: i64 = cache.values().sum();
    println!("{}", out);
    return out;
}
