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
    let mut wrong_instructions = Vec::new();
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
        if !valid {
            wrong_instructions.push(instruction_line);
        }
    }
    let mut sum = 0;

    fn valid_instruction(instruction_line: &Vec<i32>, rules_hash: &HashMap<i32, Vec<i32>>) -> bool {
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
        valid
    }

    fn swap_elements(vec: &mut Vec<i32>, idx1: usize, idx2: usize) {
        vec.swap(idx1, idx2);
    }

    for instruction_line in &mut wrong_instructions {
        while !valid_instruction(&instruction_line, &rules_hash) {
            let mut swaps = Vec::new();
            for (i, instruction) in instruction_line.iter().enumerate() {
                let left_side_slice = &instruction_line[0..i];
                if rules_hash.contains_key(&instruction) {
                    let should_be_under = rules_hash.get(&instruction).unwrap();
                    for v in left_side_slice {
                        if should_be_under.contains(v) {
                            let v_idx = instruction_line.iter().position(|&x| x == *v).unwrap();
                            let inst_idx = instruction_line
                                .iter()
                                .position(|&x| x == *instruction)
                                .unwrap();
                            swaps.push((v_idx, inst_idx));
                        }
                    }
                }
            }
            for (v_idx, inst_idx) in swaps {
                swap_elements(instruction_line, v_idx, inst_idx);
            }
        }
    }
    for instr in wrong_instructions {
        let middle_value = instr[instr.len() / 2];
        sum += middle_value;
    }
    println!("sum: {}", sum);
    sum
}
