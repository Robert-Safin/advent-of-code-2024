pub fn solution() -> i32 {
    let input: String = std::fs::read_to_string("src/inputs/day12.txt")
        .expect("failed to parse input")
        .trim()
        .to_string();

    let mut matrix: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        matrix.push(line.chars().collect());
    }

    let mut total_cost = 0;

    while has_regions_left(&matrix) {
        let start = find_region_start(&matrix);
        let neighbors = get_neighbors(&matrix, start);
        let isolated_region = isolate_region(&neighbors);
        total_cost += calculate_cost(isolated_region);
        remove_region(neighbors, &mut matrix);
    }

    println!("{:?}", total_cost);
    total_cost
}

fn has_regions_left(matrix: &Vec<Vec<char>>) -> bool {
    for row in matrix {
        for cell in row {
            if *cell != '.' {
                return true;
            }
        }
    }
    false
}
fn find_region_start(matrix: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell != &'.' {
                return Some((i, j)).unwrap();
            }
        }
    }

    panic!("No region found");
}
fn get_neighbors(matrix: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut current_region_coords: Vec<(usize, usize)> = vec![];
    let mut visited: Vec<(usize, usize)> = vec![];
    let mut to_visit: Vec<(usize, usize)> = vec![];
    to_visit.push(start);

    while !to_visit.is_empty() {
        let current = to_visit.pop().unwrap();
        let (i, j) = current;
        if visited.contains(&current) {
            continue;
        }
        visited.push(current);
        current_region_coords.push(current);

        if i > 0 && matrix[i - 1][j] == matrix[i][j] {
            to_visit.push((i - 1, j));
        }

        if i < matrix.len() - 1 && matrix[i + 1][j] == matrix[i][j] {
            to_visit.push((i + 1, j));
        }

        if j > 0 && matrix[i][j - 1] == matrix[i][j] {
            to_visit.push((i, j - 1));
        }

        if j < matrix[i].len() - 1 && matrix[i][j + 1] == matrix[i][j] {
            to_visit.push((i, j + 1));
        }
    }

    current_region_coords
}
fn isolate_region(region: &Vec<(usize, usize)>) -> Vec<Vec<char>> {
    let max_height = region.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_width = region.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let mut region_matrix: Vec<Vec<char>> = vec![vec!['.'; max_width + 1]; max_height + 1];
    for (i, j) in region {
        region_matrix[*i][*j] = 'X';
    }
    return region_matrix;
}
fn calculate_cost(isolated_region: Vec<Vec<char>>) -> i32 {
    let mut area = 0;
    let mut perimeter = 0;
    let height = isolated_region.len();
    let width = isolated_region[0].len();

    for i in 0..height {
        for j in 0..width {
            if isolated_region[i][j] == 'X' {
                area += 1;

                if i == 0 || isolated_region[i - 1][j] == '.' {
                    perimeter += 1;
                }

                if i == height - 1 || isolated_region[i + 1][j] == '.' {
                    perimeter += 1;
                }

                if j == 0 || isolated_region[i][j - 1] == '.' {
                    perimeter += 1;
                }

                if j == width - 1 || isolated_region[i][j + 1] == '.' {
                    perimeter += 1;
                }
            }
        }
    }

    area * perimeter
}
fn remove_region(region: Vec<(usize, usize)>, matrix: &mut Vec<Vec<char>>) {
    for (i, j) in region {
        matrix[i][j] = '.';
    }
}
