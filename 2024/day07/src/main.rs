use std::fmt::format;
use std::fs::read_to_string;

fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<u64>>) {
    let mut test_values: Vec<u64> = Vec::new();
    let mut numbrs: Vec<Vec<u64>> = Vec::new();
    for (i, line) in input.lines().map(|l| l.to_string()).into_iter().enumerate() {
        let split: Vec<&str> = line.split(": ").collect();
        let test_value: u64 = split[0].parse::<u64>().unwrap();
        let nums: Vec<u64> = split[1]
            .split(" ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        test_values.push(test_value);
        numbrs.push(nums)
    }
    (test_values, numbrs)
}

fn add_vals(v1: u64, v2: u64, op: char) -> u64 {
    match op {
        '+' => return v1 + v2,
        '*' => return v1 * v2,
        '|' => return format!("{v1}{v2}").parse::<u64>().unwrap(),
        _ => panic!(),
    }
}

fn eval_left_to_right(nums: &Vec<u64>, ops: Vec<char>) -> u64 {
    let mut result = nums[0];
    for (idx, &op) in ops.iter().enumerate() {
        //println!("idx {} op {}", idx, op);
        result = add_vals(result, nums[idx + 1], op);
    }
    result
}

fn generate_permutations(char1: char, char2: char, n: usize) -> Vec<String> {
    let chars = vec![char1, char2];
    let mut result = Vec::new();

    // Iterate through all combinations of length n
    for i in 0..(1 << n) {
        // (1 << n) is equivalent to 2^n
        let mut permutation = String::new();
        for j in 0..n {
            if i & (1 << j) != 0 {
                permutation.push(chars[1]);
            } else {
                permutation.push(chars[0]);
            }
        }
        result.push(permutation);
    }

    result
}

fn generate_permutations_task2(char1: char, char2: char, char3: char, n: usize) -> Vec<String> {
    let chars = vec![char1, char2, char3];
    let mut result = Vec::new();

    // Iterate through all combinations of length n
    for i in 0..(3_usize.pow(n as u32)) {  // 3^n combinations
        let mut permutation = String::new();
        let mut current = i;

        // For each position in the string
        for _ in 0..n {
            let index = current % 3;  // Get the current character index (0, 1, or 2)
            permutation.push(chars[index]);
            current /= 3;  // Move to the next character position
        }

        result.push(permutation);
    }

    result
}

fn task1(test_values: &Vec<u64>, numbrs: &Vec<Vec<u64>>) -> u64 {
    let mut total_calibration_sum: u64 = 0;
    for (i, &test_val) in test_values.iter().enumerate() {
        let nums = &numbrs[i];
        let n = nums.len() - 1; // The number of combinations

        let op_combinations = generate_permutations('+', '*', n);
        for combination in op_combinations {
            let cal_sum = eval_left_to_right(nums, combination.chars().collect());

            if cal_sum == test_val {
                total_calibration_sum += test_val;
                break;
            }
        }
    }

    total_calibration_sum
}

fn task2(test_values: &Vec<u64>, numbrs: &Vec<Vec<u64>>) -> u64 {
    let mut total_calibration_sum: u64 = 0;
    for (i, &test_val) in test_values.iter().enumerate() {
        let nums = &numbrs[i];
        let n = nums.len() - 1; // The number of combinations

        let op_combinations = generate_permutations_task2('+', '*','|', n);
        for combination in op_combinations {
            let cal_sum = eval_left_to_right(nums, combination.chars().collect());

            if cal_sum == test_val {
                total_calibration_sum += test_val;
                break;
            }
        }
    }

    total_calibration_sum
}

fn main() {
    //let file_path = "../inputs/aoc_07.txt";
    let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    let (test_values, numbrs) = parse_input(&raw_input);
    let task1_solution = task1(&test_values, &numbrs);
    println!("task1 solution is {}", task1_solution);
    let task2_solution = task2(&test_values, &numbrs);
    println!("task2 solution is {}", task2_solution);
}
