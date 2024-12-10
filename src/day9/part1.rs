pub fn solution() -> i64 {
    let input: String = std::fs::read_to_string("src/inputs/day9.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();

    let mut vec: Vec<String> = Vec::new(); // Store the whole number as a string
    let mut current_id = 0;

    for (i, c) in input.chars().enumerate() {
        let num = c
            .to_string()
            .parse::<i32>()
            .expect("failed to parse char to int");

        if i % 2 == 0 {
            for _ in 0..num {
                vec.push(current_id.to_string());
            }
            current_id += 1;
        } else {
            for _ in 0..num {
                vec.push(".".to_string());
            }
        }
    }

    let mut left_p = 0 as usize;
    let mut right_p = vec.len() - 1;

    while left_p < right_p {
        if vec[left_p] != "." {
            left_p += 1;
            continue;
        }
        if vec[right_p] == "." {
            right_p -= 1;
            continue;
        }

        vec.swap(left_p, right_p);
    }

    let mut check_sum: i64 = 0;

    for (i, v) in vec.iter().enumerate() {
        if v != "." {
            let num = v.parse::<i64>().expect("failed to parse string to int");
            check_sum += i as i64 * num;
        }
    }
    // print vec as String
    println!("{:?}", vec);
    // println!("{}", check_sum);
    check_sum
}
