use std::collections::HashMap;

pub fn solution() -> i32 {
    let input: String =
        std::fs::read_to_string("src/inputs/day5.txt").expect("failed to parse input");

    let split = input.split("\n\n").collect::<Vec<&str>>();

    let rules: Vec<Vec<i32>> = split[0]
        .lines()
        .map(|inst| inst.split("|").collect::<Vec<&str>>())
        .map(|inst| {
            inst.iter()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let instruction = split[1]
        .lines()
        .map(|inst| inst.split(",").collect::<Vec<&str>>())
        .map(|inst| {
            inst.iter()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut rules_hash: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        let first = rule[0];
        let last = rule[1];
        if rules_hash.contains_key(&first) {
            let temp = rules_hash.get_mut(&first).unwrap();
            temp.push(last);
        } else {
            rules_hash.insert(first, vec![last]);
        }
    }
    let mut sum = 0;
    for instruction_line in instruction {
        let mut valid = true;
        for (i, instruction) in instruction_line.iter().enumerate() {
            let left_side_slice = &instruction_line[0..i];
            if rules_hash.contains_key(&instruction) {
                let should_be_under = rules_hash.get(&instruction).unwrap();
                for v in left_side_slice {
                    if should_be_under.contains(v) {
                        valid = false;
                    }
                }
            }
        }
        if valid {
            let middle_value = instruction_line[instruction_line.len() / 2];
            sum += middle_value;
        }
    }

    println!("sum: {}", sum);
    sum
}
