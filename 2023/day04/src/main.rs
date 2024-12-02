use std::collections::HashSet;
use std::fs::read_to_string;

fn get_matching_numbers_per_card(puzzle_input: &Vec<String>) -> Vec<Vec<i32>> {
    let mut matching_numbers_per_card: Vec<Vec<i32>> = Vec::new();
    for line in puzzle_input {
        let scratchcard: String = line
            .split(": ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string();
        let all_numbers: HashSet<i32> = HashSet::from_iter(
            scratchcard
                .split(" | ")
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .split(" ")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
        let winnig_numbers: HashSet<i32> = HashSet::from_iter(
            scratchcard
                .split(" | ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .split(" ")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
        let matching_numbers: Vec<i32> = all_numbers
            .intersection(&winnig_numbers)
            .map(|n| *n as i32)
            .collect();
        matching_numbers_per_card.push(matching_numbers);
    }
    matching_numbers_per_card
}

fn task01(puzzle_input: &Vec<String>) {
    let matching_numbers_per_card: Vec<Vec<i32>> = get_matching_numbers_per_card(puzzle_input);
    let points: u32 = matching_numbers_per_card
        .iter()
        .map(|e| e.len())
        .filter(|&x| x > 0)
        .map(|l| u32::pow(2, (l - 1) as u32))
        .sum();
    println!("task 1 solution: {}", points);
}

fn task02(puzzle_input: &Vec<String>) {
    let matching_numbers_per_card: Vec<Vec<i32>> = get_matching_numbers_per_card(puzzle_input);
    let points: Vec<usize> = matching_numbers_per_card.iter().map(|e| e.len()).collect();
    let mut nums_scratchcards: Vec<usize> = vec![1; points.len()];
    for i in 0..nums_scratchcards.len() {
        let p = points[i];
        let num = nums_scratchcards[i];
        for j in i + 1..=i + p {
            if j < points.len() {
                nums_scratchcards[j] += num;
            }
        }
    }
    println!("task 2 solution: {}", nums_scratchcards.iter().sum::<usize>());
}

fn main() {
    let file_path = "../inputs/aoc_04.txt";
    // let file_path = "sample_input.txt";

    let mut raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    raw_input = raw_input.replace("  ", " ");
    let puzzle_input: Vec<String> = raw_input.lines().map(|l| l.to_string()).collect();
    task01(&puzzle_input);
    task02(&puzzle_input);
}
