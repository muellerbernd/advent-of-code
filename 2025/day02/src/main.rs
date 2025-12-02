use std::fs::read_to_string;

fn parse_input(input: &String) -> Vec<Vec<String>> {
    let ids: Vec<Vec<String>> = input
        .replace("\n", "")
        .split(",")
        .map(|x| x.split("-").map(|y| y.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    return ids;
}

fn has_repeating_digits(n: i64) -> bool {
    let s = n.to_string();
    let mut seen = [false; 10];
    for ch in s.chars() {
        if let Some(d) = ch.to_digit(10) {
            let idx = d as usize;
            if seen[idx] {
                return true;
            }
            seen[idx] = true;
        }
    }
    return false;
}

fn has_repeating_substr(s: String) -> (bool, String) {
    let mid = s.chars().count() / 2;
    let left = &s[..mid];
    let right = &s[mid..];
    if left == right {
        return (true, left.to_string());
    } else {
        return (false, "".to_string());
    }
}

fn task1(ids: &Vec<Vec<String>>) -> i64 {
    let mut acc: Vec<i64> = Vec::new();
    for range in ids.iter() {
        let start_id = range[0].parse::<i64>().unwrap();
        let stop_id = range[1].parse::<i64>().unwrap();
        for n in start_id..=stop_id {
            if has_repeating_substr(n.to_string()).0 {
                acc.push(n);
            }
        }
    }
    return acc.iter().sum();
}

// fn task2(ops: &Vec<i64>) -> i64 {
// }

fn main() {
    let file_path = "../inputs/aoc_02.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let parsed_input = parse_input(&raw_input);

    let task1_solution = task1(&parsed_input);
    println!("task1 solution is {}", task1_solution);
    // let task2_solution = task2(&parsed_operations);
    // println!("task2 solution is {}", task2_solution);
}

#[test]
fn test_has_repeating_substr() {
    assert_eq!(
        has_repeating_substr("112112".to_string()),
        (true, "112".to_string())
    );
}
