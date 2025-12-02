use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .split(',')
        .filter_map(|seg| {
            let mut it = seg.split('-');
            let a = it.next()?.trim().parse::<i64>().ok()?;
            let b = it.next()?.trim().parse::<i64>().ok()?;
            Some((a, b))
        })
        .collect()
}

fn has_repeating_pattern(s: &str) -> bool {
    let n = s.len();
    if n == 0 {
        return false;
    }

    let bytes = s.as_bytes();
    for d in 1..=n / 2 {
        if n % d != 0 {
            continue;
        }
        let block = &bytes[..d];
        if (d..n).step_by(d).all(|i| &bytes[i..i + d] == block) {
            return true;
        }
    }
    false
}

fn task1(ranges: &[(i64, i64)]) -> i64 {
    let mut acc = Vec::new();
    for &(start, end) in ranges {
        for num in start..=end {
            // Convert to string once per number, then check pattern
            let s = num.to_string();
            let n = s.len();

            if &s[..n / 2] == &s[n / 2..] {
                acc.push(num);
            }
        }
    }
    acc.iter().sum()
}

fn task2(ranges: &[(i64, i64)]) -> i64 {
    let mut acc = Vec::new();
    for &(start, end) in ranges {
        for n in start..=end {
            if has_repeating_pattern(n.to_string().as_str()) {
                acc.push(n);
            }
        }
    }
    acc.iter().sum()
}

fn main() {
    let file_path = "../inputs/aoc_02.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let parsed_input = parse_input(&raw_input);

    let task1_solution = task1(&parsed_input);
    println!("task1 solution is {}", task1_solution);
    let task2_solution = task2(&parsed_input);
    println!("task2 solution is {}", task2_solution);
}

#[test]
fn test_input() {
    let file_path = "test_input.txt";
    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let parsed_input = parse_input(&raw_input);
    let task1_solution = task1(&parsed_input);
    assert_eq!(task1_solution, 1227775554);
    let task2_solution = task2(&parsed_input);
    println!("task2 solution is {}", task2_solution);
    assert_eq!(task2_solution, 4174379265);
}
