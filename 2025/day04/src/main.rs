use std::collections::VecDeque;
use std::fs::read_to_string;

fn parse_input(input: &str) -> VecDeque<VecDeque<u32>> {
    let dq = VecDeque::from(
        input
            .lines()
            .map(|l| {
                VecDeque::from(
                    l.replace(".", "0")
                        .replace("@", "1")
                        .chars()
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

fn task1(matr: &VecDeque<VecDeque<u32>>) -> u32 {
    return 0;
}

// fn task2(banks: &Vec<Vec<char>>) -> u128 {
//     let mut joltages = Vec::new();
//     for bank in banks {
//         let best = largest_subseq_of_len(bank, 12);
//         joltages.push(best.parse::<u128>().unwrap())
//     }
//     return joltages.iter().sum()
// }

fn main() {
    // let file_path = "../inputs/aoc_04.txt";
    let file_path = "test_input.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let mut parsed_input = parse_input(&raw_input);
    println!("parsed input {:?}", parsed_input);

    let padded_input = pad_matr(&mut parsed_input);
    println!("padded input {:?}", padded_input);

    let task1_solution = task1(&padded_input);
    println!("task1 solution is {}", task1_solution);
    // let task2_solution = task2(&parsed_input);
    // println!("task2 solution is {}", task2_solution);
}

#[test]
fn test_input() {
    // let file_path = "test_input.txt";
    // let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    // let parsed_input = parse_input(&raw_input);
    // println!("parsed_input {:?}", parsed_input);
    // let task1_solution = task1(&parsed_input);
    // assert_eq!(task1_solution, 13);
    // let task2_solution = task2(&parsed_input);
    // assert_eq!(task2_solution, 3121910778619);
}
