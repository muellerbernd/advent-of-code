use std::fs::read_to_string;

fn parse_input(input: &str) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut test_values: Vec<i32> = Vec::new();
    let mut numbrs: Vec<Vec<i32>> = Vec::new();
    for (i, line) in input.lines().map(|l| l.to_string()).into_iter().enumerate() {
        println!("i {} line {:?}", i, line);
        let test_value: i32 = line.split(": ").collect::<Vec<&str>>()[0]
            .parse::<i32>()
            .unwrap();
        let nums: Vec<i32> = line.split(": ").collect::<Vec<&str>>()[1]
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        test_values.push(test_value);
        numbrs.push(nums)
    }
    (test_values, numbrs)
}

//fn task1(
//    start_point: (usize, usize),
//    obstacle_list: Vec<(i32, i32)>,
//    length: i32,
//) -> (i32, Vec<(usize, usize)>) {
//}
//
//fn task2(
//    start_point: (usize, usize),
//    obstacle_list: Vec<(i32, i32)>,
//    length: i32,
//    traversed_indices: Vec<(usize, usize)>,
//) -> i32 {
//}

fn main() {
    //let file_path = "../inputs/aoc_07.txt";
    let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    let (test_values, numbrs) = parse_input(&raw_input);
    println!("test_values {:?}", test_values);
    println!("numbrs {:?}", numbrs);
    //let (task1_solution, traversed_indices) = task1(
    //    start_point,
    //    obstacle_list.clone(),
    //    raw_input.lines().count() as i32,
    //);
    //println!("task1 solution is {}", task1_solution);
    //let task2_solution = task2(
    //    start_point,
    //    obstacle_list.clone(),
    //    raw_input.lines().count() as i32,
    //    traversed_indices,
    //);
    //println!("task2 solution is {}", task2_solution);
}
