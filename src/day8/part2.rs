use std::collections::{HashMap, HashSet};
pub fn solution() -> i32 {
    let input: String =
        std::fs::read_to_string("src/inputs/day8.txt").expect("failed to parse input");

    let mut hash: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (row_i, line) in input.lines().enumerate() {
        for (col_i, char) in line.chars().enumerate() {
            if char != '.' {
                let vec = hash.entry(char).or_insert(vec![]);
                vec.push((row_i as i32, col_i as i32));
            }
        }
    }

    let (width, height) = (
        input.lines().count() as i32,
        input.lines().next().unwrap().len() as i32,
    );

    let mut set: HashSet<(i32, i32)> = HashSet::new();

    for (_, v) in hash.iter() {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                let (left, right) = (v[i], v[j]);
                let (col_diff, row_diff) = (right.0 - left.0, right.1 - left.1);

                let mut anti_node_coords_one = (left.0 - col_diff, left.1 - row_diff);
                while anti_node_coords_one.0 >= 0
                    && anti_node_coords_one.0 < width
                    && anti_node_coords_one.1 >= 0
                    && anti_node_coords_one.1 < height
                {
                    set.insert(anti_node_coords_one);
                    anti_node_coords_one = (
                        anti_node_coords_one.0 - col_diff,
                        anti_node_coords_one.1 - row_diff,
                    );
                }
                let mut anti_node_coords_two = (right.0 + col_diff, right.1 + row_diff);
                while anti_node_coords_two.0 >= 0
                    && anti_node_coords_two.0 < width
                    && anti_node_coords_two.1 >= 0
                    && anti_node_coords_two.1 < height
                {
                    set.insert(anti_node_coords_two);
                    anti_node_coords_two = (
                        anti_node_coords_two.0 + col_diff,
                        anti_node_coords_two.1 + row_diff,
                    );
                }
            }
        }
    }
    hash.iter().for_each(|(_, v)| {
        if v.len() > 1 {
            for (row, col) in v {
                set.insert((*row, *col));
            }
        }
    });
    println!("set length {:?}", set.len());
    set.len() as i32
}
