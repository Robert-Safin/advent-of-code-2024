pub fn solution() -> i32 {
    let (towel_patterns, designs) = parse_input();

    let possible_count = designs
        .iter()
        .filter(|&design| is_design_possible(&towel_patterns, design))
        .count();

    println!("{}", possible_count);

    possible_count as i32
}

fn is_design_possible(towel_patterns: &Vec<String>, design: &str) -> bool {
    let mut dp: Vec<bool> = vec![false; design.len() + 1];
    dp[0] = true;

    for outer in 1..=design.len() {
        for inner in 0..outer {
            if dp[inner] && towel_patterns.contains(&design[inner..outer].to_owned()) {
                dp[outer] = true;
                break;
            }
        }
    }

    *dp.last().unwrap()
}

fn parse_input() -> (Vec<String>, Vec<String>) {
    let input: String = std::fs::read_to_string("src/inputs/day19.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let patterns = split[0]
        .split(", ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let designs = split[1]
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    (patterns, designs)
}
