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

fn task2(ops: &Vec<i32>) -> i32 {
    let mut dial: i32 = 50;
    let mut passwd: i32 = 0;
    // determine direction and number of steps
    for op in ops.into_iter() {
        let dir = if *op >= 0 { 1 } else { -1 };
        let steps = op.abs() as i32;

        for _ in 0..steps {
            dial = (dial + dir).rem_euclid(100);
            if dial == 0 {
                passwd += 1;
            }
        }
    }
    return passwd;
}

fn main() {
    let file_path = "../inputs/aoc_01.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let parsed_operations = parse_operations(&raw_input.lines().collect());

    let task1_solution = task1(&parsed_operations);
    println!("task1 solution is {}", task1_solution);
    let task2_solution = task2(&parsed_operations);
    println!("task2 solution is {}", task2_solution);
}
