use std::fs::read_to_string;

fn task1(input: &str) -> i64 {
    let start_line = input.lines().collect::<Vec<_>>()[0];
    let start_y: usize = start_line.find("S").unwrap();
    let mut cur_beams: Vec<bool> = vec![false; start_line.len()];
    cur_beams[start_y] = true;
    let mut split_counter = 0;
    for (i, line) in input.lines().skip(1).enumerate() {
        let splitters: Vec<_> = line.match_indices("^").collect();
        for (idx, _) in splitters {
            if cur_beams[idx] {
                // split
                cur_beams[idx] = false;
                cur_beams[idx + 1] = true;
                cur_beams[idx - 1] = true;
                split_counter += 1;
            }
        }
    }
    return split_counter;
}

fn task2(input: &str) -> usize {
    let start_line = input.lines().collect::<Vec<_>>()[0];
    let start_y: usize = start_line.find("S").unwrap();
    let mut cur_beams: Vec<usize> = vec![0; start_line.len()];
    cur_beams[start_y] = 1;
    for (i, line) in input.lines().skip(1).enumerate() {
        let splitters: Vec<_> = line.match_indices("^").collect();
        for (idx, v) in splitters {
            if cur_beams[idx] > 0 {
                // split
                cur_beams[idx + 1] += cur_beams[idx];
                cur_beams[idx - 1] += cur_beams[idx];
                cur_beams[idx] = 0;
            }
        }
    }
    return cur_beams.iter().sum();
}

fn main() {
    let file_path = "../inputs/aoc_07.txt";
    // let file_path = "test_input.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");

    // parse_input(&raw_input);
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
    assert_eq!(task1_solution, 21);
    let task2_solution = task2(&raw_input);
    assert_eq!(task2_solution, 40);
}
