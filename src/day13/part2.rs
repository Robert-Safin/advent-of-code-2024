#[derive(Debug)]
struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

pub fn solution() -> u64 {
    let input: String = std::fs::read_to_string("src/inputs/day13.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();

    let mut machines: Vec<Machine> = Vec::new();
    for machine in input.split("\n\n") {
        let chunk = machine.split("\n").collect::<Vec<&str>>();
        let a_chunk = chunk[0].split(", ").collect::<Vec<&str>>();
        let a0 = &a_chunk[0].to_string()[12..];
        let a1 = &a_chunk[1].to_string()[2..];
        let a = (a0.parse::<u64>().unwrap(), a1.parse::<u64>().unwrap());
        let b_chunk = chunk[1].split(", ").collect::<Vec<&str>>();
        let b0 = &b_chunk[0].to_string()[12..];
        let b1 = &b_chunk[1].to_string()[2..];
        let b = (b0.parse::<u64>().unwrap(), b1.parse::<u64>().unwrap());
        let prize_chunk = chunk[2].split(", ").collect::<Vec<&str>>();
        let prize0 = &prize_chunk[0].to_string()[9..];
        let prize1 = &prize_chunk[1].to_string()[2..];
        let prize = (
            prize0.parse::<u64>().unwrap() + 10000000000000,
            prize1.parse::<u64>().unwrap() + 10000000000000,
        );
        machines.push(Machine { a, b, prize });
    }

    let mut total_cost = 0;

    for machine in machines {
        total_cost += min_cost_to_win(machine).unwrap_or(0);
    }
    println!("{}", total_cost);
    total_cost
}
fn min_cost_to_win(machine: Machine) -> Option<u64> {
    let (p0, p1) = (machine.prize.0 as i64, machine.prize.1 as i64);
    let (a0, a1) = (machine.a.0 as i64, machine.a.1 as i64);
    let (b0, b1) = (machine.b.0 as i64, machine.b.1 as i64);

    let denominator = a1 * b0 - a0 * b1;
    let numerator_b = a1 * p0 - a0 * p1;

    if denominator == 0 || numerator_b % denominator != 0 {
        return None;
    }

    let b_count = numerator_b / denominator;
    if b_count < 0 {
        return None;
    }

    let numerator_a = p1 - b1 * b_count;
    if numerator_a % a1 != 0 {
        return None;
    }

    let a_count = numerator_a / a1;
    if a_count < 0 {
        return None;
    }

    Some((a_count as u64 * 3) + (b_count as u64 * 1))
}
