use std::fs::read_to_string;

fn parse_input(input: &str) {
    let mut obstacle_map: Vec<Vec<bool>> = Vec::new();
    let mut start_point: (usize,usize) = (0,0);
    for (i, line) in input.lines().map(|l| l.to_string()).into_iter().enumerate() {
        let mut line_mask: Vec<bool> = Vec::new();
        //println!("{:?}", line);
        for (j, c) in line.chars().enumerate() {
            //println!("{:?}", c);
            match c {
                '.' => line_mask.push(false),
                '#' => line_mask.push(true),
                '^' => {start_point.0=i;start_point.1=j},
                _ => println!("unknown char {}",c)
            }
        }
        obstacle_map.push(line_mask);
    }
    println!("obstacle_map {:?}", obstacle_map);
}

//fn task1(updates: Vec<Vec<i32>>, rules_map: HashMap<i32, Vec<i32>>) -> i32 {
//}
//
//fn task2(updates: Vec<Vec<i32>>, rules_map: HashMap<i32, Vec<i32>>) -> i32 {
//}

fn main() {
    let file_path = "../inputs/aoc_06.txt";
    //let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    //let (updates, rules_map) =
    parse_input(&raw_input);
    //let task1_solution = task1(updates.clone(), rules_map.clone());
    //println!("task1 solution is {}", task1_solution);
    //let task2_solution = task2(updates.clone(), rules_map.clone());
    //println!("task2 solution is {}", task2_solution);
}
