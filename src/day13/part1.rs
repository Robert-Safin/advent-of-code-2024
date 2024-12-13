#[derive(Debug)]
struct Machine {
    a: (i32, i32),
    b: (i32, i32),
    prize: (i32, i32),
}

pub fn solution() -> u32 {
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
        let a = (a0.parse::<i32>().unwrap(), a1.parse::<i32>().unwrap());
        let b_chunk = chunk[1].split(", ").collect::<Vec<&str>>();
        let b0 = &b_chunk[0].to_string()[12..];
        let b1 = &b_chunk[1].to_string()[2..];
        let b = (b0.parse::<i32>().unwrap(), b1.parse::<i32>().unwrap());
        let prize_chunk = chunk[2].split(", ").collect::<Vec<&str>>();
        let prize0 = &prize_chunk[0].to_string()[9..];
        let prize1 = &prize_chunk[1].to_string()[2..];
        let prize = (
            prize0.parse::<i32>().unwrap(),
            prize1.parse::<i32>().unwrap(),
        );
        machines.push(Machine { a, b, prize });
    }

    let mut total_cost = 0;

    for machine in machines {
        total_cost += min_cost_to_win(machine).unwrap_or(0);
    }
    println!("{}", total_cost);
    total_cost as u32
}

fn min_cost_to_win(machine: Machine) -> Option<u32> {
    let p0 = machine.prize.0 as i32;
    let p1 = machine.prize.1 as i32;
    let a0 = machine.a.0 as i32;
    let a1 = machine.a.1 as i32;
    let b0 = machine.b.0 as i32;
    let b1 = machine.b.1 as i32;

    let mut min_cost = i32::MAX;
    let mut solvable = false;

    for n_a in 0..=100 {
        for n_b in 0..=100 {
            if n_a * a0 + n_b * b0 == p0 && n_a * a1 + n_b * b1 == p1 {
                let cost = 3 * n_a + 1 * n_b;
                if cost < min_cost {
                    min_cost = cost;
                    solvable = true;
                }
            }
        }
    }

    if solvable {
        Some(min_cost as u32)
    } else {
        None
    }
}
