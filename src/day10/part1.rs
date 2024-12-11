use std::collections::HashSet;

pub fn solution() -> i32 {
    let input: String = std::fs::read_to_string("src/inputs/day10.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();

    let mut trails_heads: Vec<(i32, i32)> = vec![];
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for (row_i, row) in input.lines().enumerate() {
        let mut vec_row: Vec<i32> = Vec::new();
        for (col_i, col) in row.chars().enumerate() {
            let number = col.to_string().parse::<i32>().unwrap();
            if number == 0 {
                trails_heads.push((row_i as i32, col_i as i32));
            }
            vec_row.push(number);
        }
        matrix.push(vec_row);
    }

    let mut found: HashSet<(i32, i32)> = HashSet::new();
    let mut score = 0;
    for head in trails_heads.iter() {
        walk(&matrix, *head, &mut found);
        score += found.len();
        found.clear();
    }
    println!("{:?}", score);
    return score as i32;
}

fn walk(matrix: &Vec<Vec<i32>>, position: (i32, i32), found: &mut HashSet<(i32, i32)>) {
    let current_step = matrix[position.0 as usize][position.1 as usize];
    if current_step == 9 {
        found.insert(position);
    }
    let neighbors = vec![
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
        (position.0, position.1 + 1),
    ];
    for neighbor in neighbors.iter() {
        if is_in_bounds(matrix, *neighbor)
            && matrix[neighbor.0 as usize][neighbor.1 as usize] == current_step + 1
        {
            walk(matrix, *neighbor, found);
        }
    }
}

fn is_in_bounds(matrix: &Vec<Vec<i32>>, position: (i32, i32)) -> bool {
    position.0 >= 0
        && position.1 >= 0
        && position.0 < matrix.len() as i32
        && position.1 < matrix[0].len() as i32
}
