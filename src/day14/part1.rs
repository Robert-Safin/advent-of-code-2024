// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// #[derive(Debug)]
// struct Robot {
//     location: Point,
//     velocity: Point,
// }

// pub fn solution() -> i32 {
//     let input: String = std::fs::read_to_string("src/inputs/day14.txt")
//         .expect("failed to parse input")
//         .trim()
//         .to_string();

//     let mut robots: Vec<Robot> = Vec::new();
//     for line in input.lines() {
//         let parts: Vec<&str> = line.split_whitespace().collect();
//         let location_split = parts[0].split(",").collect::<Vec<&str>>();
//         let location_x = &location_split[0][2..]
//             .parse::<i32>()
//             .expect("failed to parse location x");
//         let location_y = &location_split[1]
//             .parse::<i32>()
//             .expect("failed to parse location x");
//         let location = Point {
//             x: *location_x,
//             y: *location_y,
//         };

//         let velocity_split = parts[1].split(",").collect::<Vec<&str>>();
//         let velocity_x = &velocity_split[0][2..]
//             .parse::<i32>()
//             .expect("failed to parse velocity x");
//         let velocity_y: &i32 = &velocity_split[1]
//             .parse::<i32>()
//             .expect("failed to parse velocity x");
//         let velocity = Point {
//             x: *velocity_x,
//             y: *velocity_y,
//         };
//         let robot = Robot {
//             location: location,
//             velocity: velocity,
//         };
//         robots.push(robot);
//     }
//     let grid_width = 101;
//     let grid_height = 103;
//     let mut map: Vec<Vec<i32>> = vec![vec![0; (grid_width) as usize]; (grid_height) as usize];
//     let mut end_positions: Vec<Point> = Vec::new();
//     for robot in &robots {
//         let time = 100;
//         let mut x = 0;
//         let mut y = 0;
//         let x_distance: i32 = robot.location.x + (robot.velocity.x * time);
//         let y_distance = robot.location.y + (robot.velocity.y * time);

//         if robot.velocity.x >= 0 {
//             x = x_distance % grid_width;
//         } else {
//             x = x_distance % grid_width;
//             if x < 0 {
//                 x = grid_width + x
//             }
//         }

//         if robot.velocity.y >= 0 {
//             y = y_distance % grid_height;
//         } else {
//             y = y_distance % grid_height;
//             if y < 0 {
//                 y = grid_height + y
//             }
//         }

//         let point = Point { x: x, y: y };
//         end_positions.push(point);
//     }
//     for point in &end_positions {
//         map[point.y as usize][point.x as usize] += 1;
//     }

//     let (a, b, c, d) = split_into_quadrants(map);

//     let (a_sum, b_sum, c_sum, d_sum) = (
//         sum_quadrant(a),
//         sum_quadrant(b),
//         sum_quadrant(c),
//         sum_quadrant(d),
//     );
//     println!("{}", a_sum * b_sum * c_sum * d_sum);
//     a_sum * b_sum * c_sum * d_sum
// }

// fn split_into_quadrants(
//     matrix: Vec<Vec<i32>>,
// ) -> (Vec<Vec<i32>>, Vec<Vec<i32>>, Vec<Vec<i32>>, Vec<Vec<i32>>) {
//     let height = matrix.len();
//     let width = matrix[0].len();

//     let mid_row = height / 2;
//     let mid_col = width / 2;

//     let mut quadrant1 = Vec::new();
//     let mut quadrant2 = Vec::new();
//     let mut quadrant3 = Vec::new();
//     let mut quadrant4 = Vec::new();

//     for (i, row) in matrix.iter().enumerate() {
//         if i == mid_row {
//             continue;
//         }

//         let (left, right) = row.split_at(mid_col);
//         let left_part: Vec<i32> = left.to_vec();
//         let right_part: Vec<i32> = right[1..].to_vec();

//         if i < mid_row {
//             quadrant1.push(left_part);
//             quadrant2.push(right_part);
//         } else if i > mid_row {
//             quadrant3.push(left_part);
//             quadrant4.push(right_part);
//         }
//     }

//     (quadrant1, quadrant2, quadrant3, quadrant4)
// }

// fn sum_quadrant(quadrant: Vec<Vec<i32>>) -> i32 {
//     let mut sum = 0;
//     quadrant.iter().flatten().for_each(|v| sum += v);
//     sum
// }
