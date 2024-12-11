use std::fs::read_to_string;

fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<bool>>) {
    let mut obstacle_map: Vec<Vec<bool>> = Vec::new();
    let mut start_point: (usize, usize) = (0, 0);
    for (i, line) in input.lines().map(|l| l.to_string()).into_iter().enumerate() {
        let mut line_mask: Vec<bool> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => line_mask.push(false),
                '#' => line_mask.push(true),
                '^' => {
                    start_point.0 = i;
                    start_point.1 = j
                }
                _ => println!("unknown char {}", c),
            }
        }
        obstacle_map.push(line_mask);
    }
    (start_point, obstacle_map)
}

fn task1(start_point: (usize, usize), mut obstacle_map: Vec<Vec<bool>>) -> i32 {
    println!("{:?}", start_point);
    println!("{:?}", obstacle_map);
    let mut curr_point: (usize, usize) = start_point;
    let directions: Vec<(i8, i8)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let grid_width: usize = obstacle_map[0].len();
    let grid_height: usize = obstacle_map.len();
    while curr_point.0 > 0
        && curr_point.0 < grid_height
        && curr_point.1 > 0
        && curr_point.1 < grid_width
    {}
    5
}
//fn task2(updates: Vec<Vec<i32>>, rules_map: HashMap<i32, Vec<i32>>) -> i32 {
//}

fn main() {
    //let file_path = "../inputs/aoc_06.txt";
    let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    let (start_point, obstacle_map) = parse_input(&raw_input);
    let task1_solution = task1(start_point, obstacle_map);
    println!("task1 solution is {}", task1_solution);
    //let task2_solution = task2(updates.clone(), rules_map.clone());
    //println!("task2 solution is {}", task2_solution);
}
