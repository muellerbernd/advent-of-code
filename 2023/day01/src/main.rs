use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::read_to_string;

fn task1(lines: &Vec<&str>) -> (Vec<i32>, i32) {
    let mut calibration_values: Vec<i32> = Vec::new();

    for line in lines {
        let collection: Vec<String> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_string())
            .collect();
        let two_digit_number: i32 = (collection.first().unwrap().to_owned().clone()
            + collection.last().unwrap())
        .parse::<i32>()
        .unwrap();
        calibration_values.push(two_digit_number);
    }
    (
        calibration_values.clone(),
        calibration_values.iter().sum::<i32>(),
    )
}

fn main() {
    let file_path = "../inputs/aoc_01.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    let (_, task1_solution) = task1(&raw_input.lines().collect());
    println!("task1 solution is {}", task1_solution);

    let mut remap = HashMap::new();
    remap.insert("one", "o1e");
    remap.insert("two", "t2o");
    remap.insert("three", "t3e");
    remap.insert("four", "f4r");
    remap.insert("five", "f5e");
    remap.insert("six", "s6x");
    remap.insert("seven", "s7n");
    remap.insert("eight", "e8t");
    remap.insert("nine", "n9e");
    let mut task2_input: Vec<String> = Vec::new();
    for line in raw_input.lines() {
        let mut spelled_out_digits_in_line = BTreeMap::new();
        for key in remap.keys().into_iter() {
            if line.contains(key) {
                spelled_out_digits_in_line.insert(line.find(key).unwrap(), key);
            }
        }
        let mut new_line = line.to_string();
        for (_k, v) in spelled_out_digits_in_line {
            // let digit = v.ke;
            new_line = new_line.replace(
                v,
                remap.get(v).unwrap(),
            );
        }
        task2_input.push(new_line.clone());
    }
    let (vals, task2_solution) = task1(&task2_input.iter().map(|s| s as &str).collect());
    // for (i, val) in raw_input.lines().enumerate() {
    //     println!("{} {} {}", val, task2_input[i], vals[i]);
    //     // dbg!(raw_input.lines().collect::<Vec<&str>>());
    //     // dbg!(task2_input.clone());
    // }
    println!("task2 solution is {}", task2_solution);
}
