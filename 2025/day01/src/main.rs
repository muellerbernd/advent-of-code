use std::fs::read_to_string;

fn parse_operations(lines: &Vec<&str>) -> Vec<i32> {
    let mut ops: Vec<i32> = Vec::new();
    for line in lines {
        ops.push(
            line.replace("L", "-")
                .replace("R", "")
                .parse::<i32>()
                .unwrap(),
        );
    }
    return ops;
}

fn task1(ops: &Vec<i32>) -> i32 {
    let mut dial: i32 = 50;
    let mut passwd: i32 = 0;
    for op in ops.iter() {
        dial = (dial + op).rem_euclid(100);
        if dial == 0 {
            passwd += 1;
        }
    }
    return passwd;
}
//
// fn task2(lines: &Vec<&str>) -> usize {
//     // let mut left: Vec<usize> = Vec::new();
//     // let mut right: Vec<usize> = Vec::new();
//     // for line in lines {
//     //     let nums: Vec<usize> = line
//     //         .split("   ")
//     //         .map(|n| n.parse::<usize>().unwrap())
//     //         .collect();
//     //     left.push(*nums.get(0).unwrap());
//     //     right.push(*nums.get(1).unwrap());
//     // }
//     // let similarity: Vec<usize> = left
//     //     .iter()
//     //     .map(|&e| e * right.iter().filter(|x| **x == e).count())
//     //     .collect();
//     // similarity.iter().sum()
// }

fn main() {
    let file_path = "../inputs/aoc_01.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    println!("input is {}", raw_input);
    let parsed_operations = parse_operations(&raw_input.lines().collect());
    println!("parsed input is {:?}", parsed_operations);

    let task1_solution = task1(&parsed_operations);
    println!("task1 solution is {}", task1_solution);
    // let task2_solution = task2(&raw_input.lines().collect());
    // println!("task2 solution is {}", task2_solution);
}
