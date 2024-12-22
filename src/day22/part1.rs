pub fn solution() -> i64 {
    let input = std::fs::read_to_string("src/inputs/day22.txt")
        .expect("failed to parse input")
        .trim()
        .to_string()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut out = vec![];

    for number in input {
        let mut secret = number;
        for _ in 0..2000 {
            monkey_business(&mut secret);
        }
        out.push(secret);
    }
    let sum: i64 = out.iter().sum();
    println!("{:?}", sum);
    return sum;
}

fn monkey_business(secret_number: &mut i64) {
    // step 1
    let result = *secret_number * 64;
    mix(result, secret_number);
    prune(secret_number);

    // step 2
    let result = (*secret_number as f64 / 32.0).floor() as i64;
    mix(result, secret_number);
    prune(secret_number);

    // step 3
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
