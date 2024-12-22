use std::collections::HashMap;
pub fn solution() -> i64 {
    let input: Vec<i64> = std::fs::read_to_string("src/inputs/day22.txt")
        .expect("failed to parse input")
        .trim()
        .to_string()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let digit_vectors = create_digit_vectors(input);
    let diff_vectors = create_diff_vectors(&digit_vectors);

    let mut vec_hash: Vec<HashMap<[i64; 4], i64>> = Vec::new();

    for (seq_i, seq) in diff_vectors.iter().enumerate() {
        let mut hash: HashMap<[i64; 4], i64> = HashMap::new();
        for (window_i, window) in seq.windows(4).enumerate() {
            let original_index = window_i as i64 + 4;
            if hash.contains_key(window) {
                continue;
            } else {
                hash.insert(
                    [window[0], window[1], window[2], window[3]],
                    digit_vectors[seq_i][original_index as usize],
                );
            }
        }
        vec_hash.push(hash);
    }

    let bananas = max_bananas(vec_hash);
    println!("{:?}", bananas);

    bananas
}
fn max_bananas(vec_hash: Vec<HashMap<[i64; 4], i64>>) -> i64 {
    let mut reduced_hash: HashMap<[i64; 4], i64> = HashMap::new();

    for hash in vec_hash {
        for (k, v) in hash.iter() {
            if reduced_hash.contains_key(*&k) {
                *reduced_hash.get_mut(*&k).unwrap() += v;
            } else {
                reduced_hash.insert(*k, *v);
            }
        }
    }
    *reduced_hash.iter().max_by_key(|x| x.1).unwrap().1
}

fn create_digit_vectors(input: Vec<i64>) -> Vec<Vec<i64>> {
    let mut out: Vec<Vec<i64>> = vec![];
    for number in input {
        let mut vec = vec![];
        let mut secret = number;
        vec.push(
            secret
                .to_string()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as i64,
        );
        for _ in 0..=2000 {
            monkey_business(&mut secret);
            let last_digit = secret
                .to_string()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as i64;
            vec.push(last_digit);
        }
        out.push(vec);
    }
    out
}

fn create_diff_vectors(digit_vectors: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut out = vec![];
    for vec in digit_vectors {
        let mut diff_vec = vec![];
        for window in vec.windows(2) {
            diff_vec.push(window[1] - window[0]);
        }
        out.push(diff_vec);
    }
    out
}

fn monkey_business(secret_number: &mut i64) {
    let result = *secret_number * 64;
    mix(result, secret_number);
    prune(secret_number);

    let result = (*secret_number as f64 / 32.0).floor() as i64;
    mix(result, secret_number);

    prune(secret_number);
    let result = *secret_number * 2048;
    mix(result, secret_number);
    prune(secret_number);
}

fn mix(given_value: i64, secret_number: &mut i64) {
    *secret_number = given_value ^ *secret_number;
}

fn prune(secret_number: &mut i64) {
    *secret_number = *secret_number % 16777216;
}
