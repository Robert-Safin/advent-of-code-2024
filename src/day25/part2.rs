pub fn solution() -> i32 {
    let (keys, locks) = parse_input();

    let mut count = 0;

    for key in keys {
        let sub_count = try_key(key, locks.clone());
        count += sub_count;
    }
    println!("{}", count);
    count
}

fn try_key(key: Vec<Vec<i8>>, locks: Vec<Vec<Vec<i8>>>) -> i32 {
    let mut count = 0;

    for lock in locks {
        let sum_matrix = sum_matrices(&key, &lock);

        if sum_matrix.iter().flatten().any(|v| *v == 2) {
            continue;
        } else {
            count += 1;
        }
    }
    count
}

fn sum_matrices(key: &Vec<Vec<i8>>, lock: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    key.iter()
        .zip(lock.iter())
        .map(|(row_key, row_lock)| {
            row_key
                .iter()
                .zip(row_lock.iter())
                .map(|(cell_key, cell_lock)| cell_key + cell_lock)
                .collect()
        })
        .collect()
}

fn parse_input() -> (Vec<Vec<Vec<i8>>>, Vec<Vec<Vec<i8>>>) {
    let input = std::fs::read_to_string("src/inputs/day25.txt")
        .unwrap()
        .trim()
        .to_owned();

    let mut things: Vec<Vec<Vec<i8>>> = vec![];

    for bit in input.split("\n\n") {
        let mut num_bit: Vec<Vec<i8>> = vec![];
        for line in bit.lines() {
            let mut num_line: Vec<i8> = vec![];
            for c in line.chars() {
                if c == '#' {
                    num_line.push(1);
                } else {
                    num_line.push(0);
                }
            }
            num_bit.push(num_line);
        }
        things.push(num_bit);
    }

    let mut keys: Vec<Vec<Vec<i8>>> = vec![];
    let mut locks: Vec<Vec<Vec<i8>>> = vec![];

    for thing in things {
        if thing[0].iter().all(|v| *v == 1) {
            locks.push(thing);
        } else {
            keys.push(thing);
        }
    }

    return (keys, locks);
}
