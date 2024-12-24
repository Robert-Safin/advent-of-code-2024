use std::collections::{HashMap, HashSet};
pub fn solution() -> i32 {
    let connections: Vec<(String, String)> = std::fs::read_to_string("src/inputs/day23.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|x| {
            let mut parts = x.split("-");
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().to_string(),
            )
        })
        .collect();

    let adj_list: HashMap<String, Vec<String>> = adjacency_list(connections);

    let triangles: HashSet<Vec<String>> = find_triangles(adj_list);

    let score = count_t(triangles);
    println!("Score: {}", score);

    score
}
fn count_t(triangles: HashSet<Vec<String>>) -> i32 {
    let mut score = 0;
    for triangle in triangles {
        for node in triangle {
            if node.starts_with("t") {
                score += 1;
                break;
            }
        }
    }
    score
}

fn find_triangles(adj_list: HashMap<String, Vec<String>>) -> HashSet<Vec<String>> {
    let mut triangles: HashSet<Vec<String>> = HashSet::new();

    for x in adj_list.keys() {
        for y in adj_list.get(x).unwrap() {
            for z in adj_list.get(y).unwrap() {
                if x != z && adj_list.get(z).unwrap().contains(x) {
                    let mut triangle: Vec<String> = vec![x.to_string(), y.to_owned(), z.to_owned()];
                    triangle.sort();
                    triangles.insert(triangle);
                }
            }
        }
    }

    triangles
}

fn adjacency_list(connections: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

    for c in &connections {
        adj_list
            .entry(c.0.clone())
            .or_insert(Vec::new())
            .push(c.1.clone());
        adj_list
            .entry(c.1.clone())
            .or_insert(Vec::new())
            .push(c.0.clone());
    }

    adj_list
}
