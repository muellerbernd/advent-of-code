use regex::Regex;
use std::fs::read_to_string;

fn task1(input: &str) -> i32 {
    let re =
        Regex::new(r"mul\(([1-9]|[1-9][0-9]|[1-9][0-9][0-9]),([1-9]|[1-9][0-9]|[1-9][0-9][0-9])\)")
            .unwrap();
    let mut result = 0;
    for cap in re.captures_iter(&input) {
        let m = cap.get(0).unwrap().as_str();
        let cleaned_input: String = m.replace(|c: char| !(c.is_digit(10) | (c == ',')), "");
        let nums: Vec<i32> = cleaned_input.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        result += nums.get(0).unwrap() * nums.get(1).unwrap();
    }
    result
}

fn task2(lines: &Vec<&str>) -> u32 {
    5
}

fn main() {
    let file_path = "../inputs/aoc_03.txt";
    //let file_path = "test_input.txt";

    let raw_input: String = read_to_string(file_path)
        .expect("Should have been able to read the file")
        .replace("\n", "");

    let task1_solution = task1(&raw_input);
    println!("task1 solution is {}", task1_solution);
    //let task2_solution = task2(&raw_input.lines().collect());
    //println!("task2 solution is {}", task2_solution);
}
