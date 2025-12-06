use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
        .collect()
}

fn task1(problems: &Vec<Vec<String>>) -> i64 {
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

//
// fn task2(fresh_id_ranges: &Vec<(u64, u64)>) -> usize {
//     let mut fresh_count: usize = 0;
//     for (s, e) in fresh_id_ranges {
//         fresh_count += ((e - s) + 1) as usize;
//     }
//     return fresh_count;
// }

fn main() {
    let file_path = "../inputs/aoc_06.txt";
    // let file_path = "test_input.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let parsed_input = parse_input(&raw_input);

    let task1_solution = task1(&parsed_input);
    println!("task1 solution is {}", task1_solution);
    // let task2_solution = task2(&fresh_ids);
    // println!("task2 solution is {}", task2_solution);
}

#[test]
fn test_input() {
    let file_path = "test_input.txt";
    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let parsed_input = parse_input(&raw_input);
    let task1_solution = task1(&parsed_input);
    assert_eq!(task1_solution, 4277556);
    // let task2_solution = task2(&fresh_ids);
    // assert_eq!(task2_solution, 14);
}
