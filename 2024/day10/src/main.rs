use std::collections::BTreeMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct MapPoint {
    x: usize,
    y: usize,
    height: u32,
}

fn parse_input(input: &String) -> Vec<MapPoint> {
    let mut map_points: Vec<MapPoint> = Vec::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let map_point = MapPoint {
                x: (x),
                y: (y),
                height: (c.to_digit(10).unwrap()),
            };
            map_points.push(map_point);
        }
    }
    map_points
}

fn main() {
    //let file_path = "../inputs/aoc_10.txt";
    let file_path = "test_input.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let parsed_input: Vec<MapPoint> = parse_input(&raw_input);
    println!("parsed_input {:?}", parsed_input);

    //let task1_solution = task1(&parsed_input);
    //println!("task1 solution is {}", task1_solution);
    //let parsed_files: Vec<DiskChunk> = parse_files(&raw_input);
    //let task2_solution = task2(&parsed_files);
    //println!("task2 solution is {}", task2_solution);
    //let task2_solution = task2(&antenna_map, grid_height, grid_width);
    //println!("task2 solution is {}", task2_solution);
}
