use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let t = l.split(",").collect::<Vec<&str>>();
            (
                t[0].parse::<usize>().unwrap(),
                t[1].parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn task1(red_tiles: &Vec<(usize, usize)>) -> i64 {
    println!("{:?}", red_tiles);
    return 0;
}

fn main() {
    // let file_path = "../inputs/aoc_08.txt";
    let file_path = "test_input.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");

    let red_tiles = parse_input(&raw_input);
    let task1_solution = task1(&red_tiles);
    println!("task1 solution is {}", task1_solution);
    // let task2_solution = task2(&junction_boxes);
    // println!("task2 solution is {}", task2_solution);
}

// #[test]
// fn task01() {
//     let file_path = "test_input.txt";
//     let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
//     let junction_boxes = parse_input(&raw_input);
//     let task1_solution = task1(&junction_boxes, 10);
//     assert_eq!(task1_solution, 40);
//     // let task2_solution = task2(&junction_boxes);
//     // assert_eq!(task2_solution, 25272);
// }
//
// #[test]
// fn task02() {
//     let file_path = "test_input.txt";
//     let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
//     let junction_boxes = parse_input(&raw_input);
//     let task2_solution = task2(&junction_boxes);
//     assert_eq!(task2_solution, 25272);
// }
