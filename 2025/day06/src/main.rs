use std::fs::read_to_string;

fn task1(input: &str) -> i64 {
    let problems: Vec<Vec<String>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
        .collect();
    let num_rows = problems.len() - 1;
    let operators = problems.last().unwrap();
    let mut running_vals: Vec<i64> = problems
        .first()
        .unwrap()
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    for row in 1..num_rows {
        let vals: Vec<i64> = problems[row]
            .clone()
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        running_vals = running_vals
            .iter()
            .zip(vals.iter())
            .enumerate()
            .map(|(_i, (x, y))| match operators[_i].as_str() {
                "*" => x * y,
                "+" => x + y,
                _ => 0,
            })
            .collect();
    }

    return running_vals.iter().sum();
}

fn task2(input: &str) -> i64 {
    let problems: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let operators = problems.last().unwrap();
    let mut cols: Vec<(usize, usize)> = Vec::new();
    let mut last_col_start = 0;
    for (idx, v) in operators.iter().enumerate() {
        match *v == '*' || *v == '+' {
            true => {
                if idx == 0 {
                    last_col_start = idx;
                } else {
                    cols.push((last_col_start, idx - 2));
                    last_col_start = idx;
                }
            }
            false => (),
        }
    }
    println!("cols {:?}", cols);
    return 0;
}

fn main() {
    let file_path = "../inputs/aoc_06.txt";
    let file_path = "test_input.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");

    let task1_solution = task1(&raw_input);
    println!("task1 solution is {}", task1_solution);
    let task2_solution = task2(&raw_input);
    println!("task2 solution is {}", task2_solution);
}

#[test]
fn test_input() {
    let file_path = "test_input.txt";
    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let task1_solution = task1(&raw_input);
    assert_eq!(task1_solution, 4277556);
    let task2_solution = task2(&raw_input);
    assert_eq!(task2_solution, 3263827);
}
