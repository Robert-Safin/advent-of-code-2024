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
//     let mut time_count = 0;
//     loop {
//         let mut map: Vec<Vec<i32>> = vec![vec![0; (grid_width) as usize]; (grid_height) as usize];
//         let mut end_positions: Vec<Point> = Vec::new();
//         for robot in &robots {
//             let time = time_count;
//             let mut x = 0;
//             let mut y = 0;
//             let x_distance: i32 = robot.location.x + (robot.velocity.x * time);
//             let y_distance: i32 = robot.location.y + (robot.velocity.y * time);

//             if robot.velocity.x >= 0 {
//                 x = x_distance % grid_width;
//             } else {
//                 x = x_distance % grid_width;
//                 if x < 0 {
//                     x = grid_width + x
//                 }
//             }

//             if robot.velocity.y >= 0 {
//                 y = y_distance % grid_height;
//             } else {
//                 y = y_distance % grid_height;
//                 if y < 0 {
//                     y = grid_height + y
//                 }
//             }

//             let point = Point { x: x, y: y };
//             end_positions.push(point);
//         }

//         for point in &end_positions {
//             map[point.y as usize][point.x as usize] += 1;
//         }

//         let robot_count = count_robots_on_map(&map);

//         if robot_count == robots.len() as i32 {
//             for line in map {
//                 println!("{:?}", line);
//             }
//             println!("Found at time: {}", time_count);
//             return time_count;
//         }
//         time_count += 1;
//     }
// }

// fn count_robots_on_map(map: &Vec<Vec<i32>>) -> i32 {
//     let mut count = 0;
//     for row in map {
//         for col in row {
//             if col > &0 {
//                 count += 1;
//             }
//         }
//     }
//     count
// }
