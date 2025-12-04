use std::collections::VecDeque;
use std::fs::read_to_string;

fn parse_input(input: &str) -> VecDeque<VecDeque<u32>> {
    let dq = VecDeque::from(
        input
            .replace(".", "0")
            .replace("@", "1")
            .lines()
            .map(|l| {
                VecDeque::from(
                    l.chars()
                        .map(|v| v.to_digit(10).unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>(),
    );
    return dq;
}

fn pad_matr(matr: &VecDeque<VecDeque<u32>>) -> VecDeque<VecDeque<u32>> {
    let w = matr.get(0).unwrap().len();
    let mut pad_matr = matr.clone();
    for row in pad_matr.iter_mut() {
        row.push_front(0);
        row.push_back(0);
    }
    pad_matr.push_front(VecDeque::from(vec![0 as u32; w + 2]));
    pad_matr.push_back(VecDeque::from(vec![0 as u32; w + 2]));
    return pad_matr;
}

fn get_removeable_rolls(matr: &VecDeque<VecDeque<u32>>) -> Vec<(usize, usize)> {
    let height = matr.len();
    let width = matr.get(0).unwrap().len();
    let mut removeable_rolls: Vec<(usize, usize)> = Vec::new();
    for row_idx in 1..height - 1 {
        for col_idx in 1..width - 1 {
            if matr[row_idx][col_idx] == 0 {
                continue;
            };
            let mut v = Vec::new();
            v.push(matr[row_idx - 1][col_idx - 1]);
            v.push(matr[row_idx - 1][col_idx]);
            v.push(matr[row_idx - 1][col_idx + 1]);
            v.push(matr[row_idx][col_idx - 1]);
            v.push(matr[row_idx][col_idx + 1]);
            v.push(matr[row_idx + 1][col_idx - 1]);
            v.push(matr[row_idx + 1][col_idx]);
            v.push(matr[row_idx + 1][col_idx + 1]);
            let sum: u32 = v.iter().sum();
            if sum < 4 {
                removeable_rolls.push((row_idx, col_idx));
            }
        }
    }
    return removeable_rolls;
}

fn task1(matr: &VecDeque<VecDeque<u32>>) -> usize {
    let removeable_rolls = get_removeable_rolls(matr);
    return removeable_rolls.len();
}

fn task2(matr: &VecDeque<VecDeque<u32>>) -> usize {
    let mut total_remove_rolls = 0;
    let mut edit_matr = matr.clone();
    loop {
        let removable_rolls = get_removeable_rolls(&edit_matr);
        if removable_rolls.len() == 0 {
            break;
        }
        for (row, col) in removable_rolls.clone().into_iter() {
            edit_matr[row][col] = 0;
        }
        total_remove_rolls += removable_rolls.len();
    }
    return total_remove_rolls;
}

fn main() {
    let file_path = "../inputs/aoc_04.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let mut parsed_input = parse_input(&raw_input);

    let padded_input = pad_matr(&mut parsed_input);

    let task1_solution = task1(&padded_input);
    println!("task1 solution is {}", task1_solution);
    let task2_solution = task2(&padded_input);
    println!("task2 solution is {}", task2_solution);
}

#[test]
fn test_input() {
    let file_path = "test_input.txt";
    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let mut parsed_input = parse_input(&raw_input);
    let padded_input = pad_matr(&mut parsed_input);
    let task1_solution = task1(&padded_input);
    assert_eq!(task1_solution, 13);
    let task2_solution = task2(&padded_input);
    assert_eq!(task2_solution, 43);
}
