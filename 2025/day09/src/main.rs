use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|l| {
            let t = l
                .split(",")
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (t[0], t[1])
        })
        .collect()
}

fn task1(red_tiles: &Vec<(i64, i64)>) -> i64 {
    let mut areas: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            let tile_a = red_tiles[i];
            let tile_b = red_tiles[j];

            let area = (tile_a.0 - tile_b.0 + 1).abs() * (tile_a.1 - tile_b.1 + 1).abs();
            areas.push((area, i, j));
        }
    }
    areas.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));
    let (d, _, _) = areas.last().unwrap();
    return *d;
}

fn main() {
    let file_path = "../inputs/aoc_09.txt";
    // let file_path = "test_input.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");

    let red_tiles = parse_input(&raw_input);
    let task1_solution = task1(&red_tiles);
    println!("task1 solution is {}", task1_solution);
    // let task2_solution = task2(&junction_boxes);
    // println!("task2 solution is {}", task2_solution);
}

#[test]
fn task01() {
    let file_path = "test_input.txt";
    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let red_tiles = parse_input(&raw_input);
    let task1_solution = task1(&red_tiles);
    assert_eq!(task1_solution, 50);
}
//
// #[test]
// fn task02() {
//     let file_path = "test_input.txt";
//     let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
//     let junction_boxes = parse_input(&raw_input);
//     let task2_solution = task2(&junction_boxes);
//     assert_eq!(task2_solution, 25272);
// }
