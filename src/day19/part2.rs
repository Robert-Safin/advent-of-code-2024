pub fn solution() -> i32 {
    let (towel_patterns, designs) = parse_input();

    let total_ways: usize = designs
        .iter()
        .map(|design| count_ways(&towel_patterns, design))
        .sum();

    println!("{}", total_ways);
    return total_ways as i32;
}
fn count_ways(towel_patterns: &Vec<String>, design: &str) -> usize {
    let mut dp = vec![0; design.len() + 1];
    dp[0] = 1;

    for outer in 1..=design.len() {
        for inner in 0..outer {
            if towel_patterns.contains(&design[inner..outer].to_owned()) {
                dp[outer] += dp[inner];
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
