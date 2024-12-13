use std::collections::HashMap;
use std::fmt::format;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct Antenna {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> HashMap<char, Vec<Antenna>> {
    let mut antenna_map: HashMap<char, Vec<Antenna>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let a: Antenna = Antenna { x, y };
                antenna_map
                    .entry(c)
                    .and_modify(|v| v.push(a))
                    .or_insert(vec![a]);
            }
        }
    }
    antenna_map
}

fn task1(test_values: &Vec<u64>, numbrs: &Vec<Vec<u64>>) -> u64 {
    5
}

fn task2(test_values: &Vec<u64>, numbrs: &Vec<Vec<u64>>) -> u64 {
    5
}

fn main() {
    //let file_path = "../inputs/aoc_07.txt";
    let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let antenna_map: HashMap<char, Vec<Antenna>> = parse_input(&raw_input);
    println!("{:?}", antenna_map);

    //let (test_values, numbrs) = parse_input(&raw_input);
    //let task1_solution = task1(&test_values, &numbrs);
    //println!("task1 solution is {}", task1_solution);
    //let task2_solution = task2(&test_values, &numbrs);
    //println!("task2 solution is {}", task2_solution);
}
