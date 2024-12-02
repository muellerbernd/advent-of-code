use std::fs::read_to_string;

fn parse_input(input: &String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split(" ").map(|c| c.parse::<i32>().unwrap()).collect())
        .collect()
}

fn task01(histories: &Vec<Vec<i32>>) {
    let mut extrapolated_values: Vec<i32> = Vec::new();
    for i in 0..histories.len() {
        let mut temp_sequences: Vec<Vec<i32>> = Vec::new();
        let mut temp_sequence: Vec<i32> = histories[i].to_owned();
        while !temp_sequence.iter().all(|&x| x == 0) {
            temp_sequences.push(temp_sequence.clone());
            let mut scratchpad: Vec<i32> = Vec::new();
            for j in 1..temp_sequence.len() {
                scratchpad.push(temp_sequence[j] - temp_sequence[j - 1]);
            }
            temp_sequence = scratchpad;
        }
        temp_sequences.push(temp_sequence.clone());
        let extrapolated_value: i32 = temp_sequences
            .iter()
            .fold(0, |acc, v| acc + v.last().unwrap());

        extrapolated_values.push(extrapolated_value);
    }
    println!(
        "Solution for Task 1 = {}",
        extrapolated_values.iter().sum::<i32>()
    );
}

fn task02(histories: &Vec<Vec<i32>>) {
    let mut extrapolated_values: Vec<i32> = Vec::new();
    for i in 0..histories.len() {
        let mut temp_sequences: Vec<Vec<i32>> = Vec::new();
        let mut temp_sequence: Vec<i32> = histories[i].to_owned();
        while !temp_sequence.iter().all(|&x| x == 0) {
            temp_sequences.push(temp_sequence.clone());
            let mut scratchpad: Vec<i32> = Vec::new();
            for j in 1..temp_sequence.len() {
                scratchpad.push(temp_sequence[j] - temp_sequence[j - 1]);
            }
            temp_sequence = scratchpad;
        }
        temp_sequences.push(temp_sequence.clone());
        let mut extrapolated_value: i32 = 0;
        for seq in temp_sequences.iter().rev() {
            let val: i32 = *seq.first().unwrap();
            extrapolated_value = val - extrapolated_value;
        }

        extrapolated_values.push(extrapolated_value);
    }
    println!(
        "Solution for Task 2 = {}",
        extrapolated_values.iter().sum::<i32>()
    );
}
fn main() {
    let file_path = "../inputs/aoc_09.txt";
    // let file_path = "sample_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    println!("{:?}", raw_input);
    let histories: Vec<Vec<i32>> = parse_input(&raw_input);
    println!("{:?}", histories);
    task01(&histories);
    task02(&histories);
}
