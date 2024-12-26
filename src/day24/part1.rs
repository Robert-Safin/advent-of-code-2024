use std::collections::{HashMap, VecDeque};
pub fn solution() -> i64 {
    let (mut wires, mut connections) = parse_input();

    while connections.len() > 0 {
        let (a, op, b, target) = connections.pop_front().unwrap();

        let a_value = wires.get(&a);
        let b_value = wires.get(&b);

        if a_value.is_none() || b_value.is_none() {
            connections.push_back((a, op, b, target));
            continue;
        } else {
            let a_value = a_value.unwrap();
            let b_value = b_value.unwrap();
            match op.as_str() {
                "AND" => {
                    let value = and(*a_value, *b_value);
                    wires.insert(target, value);
                }
                "OR" => {
                    let value = or(*a_value, *b_value);
                    wires.insert(target, value);
                }
                "XOR" => {
                    let value = xor(*a_value, *b_value);
                    wires.insert(target, value);
                }
                _ => {
                    panic!("Unknown operator");
                }
            }
        }
    }
    get_decimal_value(&mut wires)
}
fn get_decimal_value(wires: &mut HashMap<String, i32>) -> i64 {
    let keys_to_remove: Vec<String> = wires
        .iter()
        .filter(|(k, _)| k.chars().next().unwrap() != 'z')
        .map(|(k, _)| k.clone())
        .collect();

    for k in keys_to_remove {
        wires.remove(&k);
    }

    let mut wires_ints: Vec<(i32, i32)> = Vec::new();

    for (k, v) in wires {
        let int = k
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        wires_ints.push((int, *v));
    }

    wires_ints.sort();
    let binary_str = wires_ints
        .iter()
        .rev()
        .map(|(_, v)| v.to_string())
        .collect::<String>();
    println!("Binary value: {}", binary_str);
    let decimal = i64::from_str_radix(&binary_str, 2).unwrap();
    println!("Decimal value: {}", decimal);
    return decimal;
}

fn and(a: i32, b: i32) -> i32 {
    if a == 1 && b == 1 {
        return 1;
    } else {
        return 0;
    }
}

fn or(a: i32, b: i32) -> i32 {
    if a == 1 || b == 1 {
        return 1;
    } else {
        return 0;
    }
}

fn xor(a: i32, b: i32) -> i32 {
    if a == 1 && b == 0 {
        return 1;
    } else if a == 0 && b == 1 {
        return 1;
    } else {
        return 0;
    }
}

fn parse_input() -> (
    HashMap<String, i32>,
    VecDeque<(String, String, String, String)>,
) {
    let input = std::fs::read_to_string("src/inputs/day24.txt")
        .unwrap()
        .trim()
        .to_owned();

    let split = input.split("\n\n").collect::<Vec<&str>>();

    let mut wires: HashMap<String, i32> = HashMap::new();
    for wire in split[0].lines() {
        let split = wire.split(": ").collect::<Vec<&str>>();
        wires.insert(split[0].to_owned(), split[1].parse::<i32>().unwrap());
    }

    let mut connections: VecDeque<(String, String, String, String)> = VecDeque::new();

    for connection in split[1].lines() {
        let split = connection.split(" ").collect::<Vec<&str>>();
        connections.push_back((
            split[0].to_owned(),
            split[1].to_owned(),
            split[2].to_owned(),
            split[4].to_owned(),
        ));
    }

    return (wires, connections);
}
