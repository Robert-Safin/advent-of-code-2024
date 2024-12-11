pub fn solution() -> i32 {
    let input: String = std::fs::read_to_string("src/inputs/day11.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();

    let mut stones: Vec<i64> = input.split(" ").map(|x| x.parse().unwrap()).collect();

    for _ in 0..25 {
        stones = blink(stones);
    }

    println!("vec: {:?}", stones.len());

    return stones.len() as i32;
}

fn blink(stones: Vec<i64>) -> Vec<i64> {
    let mut new_stones: Vec<i64> = vec![];
    for stone in stones.iter() {
        let digits_str = stone.to_string();
        if *stone == 0 {
            new_stones.push(1);
        } else if digits_str.len() % 2 == 0 {
            let mid = digits_str.len() / 2;
            let (left_str, right_str) = digits_str.split_at(mid);
            let left = left_str.parse::<i64>().unwrap();
            let right = right_str.parse::<i64>().unwrap();
            new_stones.push(left);
            new_stones.push(right);
        } else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
}
