use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn task1(banks: &Vec<Vec<char>>) -> u32 {
    let mut joltages = Vec::new();
    for bank in banks {
        let mut max_joltage = 0;
        for i in 0..bank.len() {
            for j in i+1..bank.len() {
                let joltage: u32 = format!("{}{}", bank[i], bank[j]).parse::<u32>().unwrap();
                if joltage > max_joltage {
                    max_joltage = joltage;
                }
            }
        }
        joltages.push(max_joltage);
    }
    return joltages.iter().sum()
}
//
// fn task2(ranges: &[(i64, i64)]) -> i64 {
// }

fn main() {
    let file_path = "../inputs/aoc_03.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let parsed_input = parse_input(&raw_input);
    let task1_solution = task1(&parsed_input);
    println!("task1 solution is {}", task1_solution);
    // let task2_solution = task2(&parsed_input);
    // println!("task2 solution is {}", task2_solution);
}

#[test]
fn test_input() {
    let file_path = "test_input.txt";
    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let parsed_input = parse_input(&raw_input);
    println!("parsed_input {:?}", parsed_input);
    let task1_solution = task1(&parsed_input);
    assert_eq!(task1_solution, 357);
    // let task2_solution = task2(&parsed_input);
    // println!("task2 solution is {}", task2_solution);
    // assert_eq!(task2_solution, 4174379265);
}
