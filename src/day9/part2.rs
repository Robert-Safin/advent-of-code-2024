// f
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
    // println!("{:?}", vec);
    let mut grouped: Vec<Vec<String>> = group_strings(vec);
    let mut left = 0 as usize;
    let mut right = grouped.len() - 1;

    for _ in 0..42 {
        //println!("{:?}", grouped);
        println!("\n{:?} {:?}", left, right);

        if left >= right {
            left = 0;
            right -= 1;
            continue;
        }

        if !grouped[left].contains(&".".to_owned()) {
            left += 1;
            continue;
        }
        if grouped[right].contains(&".".to_owned()) {
            right -= 1;
            continue;
        }

        if grouped[left].len() < grouped[right].len() {
            left += 1;
            continue;
        }

        if grouped[left].len() >= grouped[right].len() {
            let right_group = grouped[right].clone();
            for (i, v) in right_group.iter().enumerate() {
                grouped[left][i] = v.clone();
                grouped[right][i] = ".".to_owned();
            }
            left = 0;
        }
        flatten(&mut grouped);
        grouped = group_strings(grouped[0].clone());
    }

    0
}

fn flatten(groups: &mut Vec<Vec<String>>) {
    let mut result = Vec::new();
    for group in groups.iter() {
        for s in group {
            result.push(s.clone());
        }
    }
    *groups = vec![result];
}

fn group_strings(input: Vec<String>) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut current_group = Vec::new();

    for s in input {
        if current_group.is_empty() || s == current_group[0] {
            current_group.push(s);
        } else {
            result.push(current_group);
            current_group = vec![s];
        }
    }

    if !current_group.is_empty() {
        result.push(current_group);
    }

    result
}
