use std::fs::read_to_string;

fn task1(lines: &Vec<&str>) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines {
        let nums: Vec<i32> = line
            .split("   ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        left.push(*nums.get(0).unwrap());
        right.push(*nums.get(1).unwrap());
    }
    left.sort();
    right.sort();
    let diffs: Vec<i32> = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| (l - r).abs())
        .collect();
    diffs.iter().sum()
}

fn task2(lines: &Vec<&str>) -> usize {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    for line in lines {
        let nums: Vec<usize> = line
            .split("   ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        left.push(*nums.get(0).unwrap());
        right.push(*nums.get(1).unwrap());
    }
    let similarity: Vec<usize> = left
        .iter()
        .map(|&e| e * right.iter().filter(|x| **x == e).count())
        .collect();
    similarity.iter().sum()
}

fn main() {
    let file_path = "../inputs/aoc_01.txt";
    //let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    let task1_solution = task1(&raw_input.lines().collect());
    println!("task1 solution is {}", task1_solution);
    let task2_solution = task2(&raw_input.lines().collect());
    println!("task2 solution is {}", task2_solution);
}
