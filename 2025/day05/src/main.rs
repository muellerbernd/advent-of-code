use std::fs::read_to_string;

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let split_input = input.split("\n\n").collect::<Vec<_>>();
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in split_input[0].lines() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() != 2 {
            continue;
        }
        let a: u64 = parts[0].trim().parse().unwrap();
        let b: u64 = parts[1].trim().parse().unwrap();
        let (start, end) = if a <= b { (a, b) } else { (b, a) };
        ranges.push((start, end));
    }

    // Merge overlapping ranges
    ranges.sort_by_key(|r| r.0);
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (s, e) in ranges {
        if let Some(last) = merged.last_mut() {
            if s <= last.1 + 1 {
                // overlapping or touching ranges -> extend
                last.1 = last.1.max(e);
            } else {
                merged.push((s, e));
            }
        } else {
            merged.push((s, e));
        }
    }

    let available_ids: Vec<u64> = split_input[1]
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    return (merged, available_ids);
}

fn task1(fresh_id_ranges: &Vec<(u64, u64)>, available_ids: &Vec<u64>) -> usize {
    let mut fresh_count: usize = 0;
    for id in available_ids {
        if fresh_id_ranges.iter().any(|&(s, t)| *id >= s && *id <= t) {
            fresh_count += 1;
        }
    }
    return fresh_count;
}

fn task2(fresh_id_ranges: &Vec<(u64, u64)>) -> usize {
    let mut fresh_count: usize = 0;
    for (s, e) in fresh_id_ranges {
        fresh_count += ((e - s) + 1) as usize;
    }
    return fresh_count;
}

fn main() {
    let file_path = "../inputs/aoc_05.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let (fresh_ids, available_ids) = parse_input(&raw_input);

    let task1_solution = task1(&fresh_ids, &available_ids);
    println!("task1 solution is {}", task1_solution);
    let task2_solution = task2(&fresh_ids);
    println!("task2 solution is {}", task2_solution);
}

#[test]
fn test_input() {
    let file_path = "test_input.txt";
    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let (fresh_ids, available_ids) = parse_input(&raw_input);
    let task1_solution = task1(&fresh_ids, &available_ids);
    assert_eq!(task1_solution, 3);
    let task2_solution = task2(&fresh_ids);
    assert_eq!(task2_solution, 14);
}
