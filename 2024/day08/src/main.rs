use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Antenna {
    x: i32,
    y: i32,
    name: char,
}

fn parse_input(input: &str) -> ((usize, usize), HashMap<char, Vec<Antenna>>) {
    let mut antenna_map: HashMap<char, Vec<Antenna>> = HashMap::new();
    let grid_height = input.lines().collect::<Vec<&str>>().len();
    let grid_width = input.lines().collect::<Vec<&str>>()[0]
        .chars()
        .collect::<Vec<char>>()
        .len();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let a: Antenna = Antenna {
                    x: (x as i32),
                    y: (y as i32),
                    name: (c),
                };
                antenna_map
                    .entry(c)
                    .and_modify(|v| v.push(a))
                    .or_insert(vec![a]);
            }
        }
    }
    ((grid_height, grid_width), antenna_map)
}

fn get_antenna_delta(a0: Antenna, a1: Antenna) -> (i32, i32) {
    let delta_x = a0.x as i32 - a1.x as i32;
    let delta_y = a0.y as i32 - a1.y as i32;
    (delta_x, delta_y)
}

fn task1(
    antenna_map: &HashMap<char, Vec<Antenna>>,
    grid_height: usize,
    grid_width: usize,
) -> usize {
    println!("{:?}", antenna_map);
    println!("height {:?} width {}", grid_height, grid_width);
    let antenna_grid_points: Vec<Antenna> = antenna_map.values().flatten().map(|a| *a).collect::<Vec<Antenna>>();
    let mut antinode_grid_points: Vec<Antenna> = Vec::new();
    let mut final_grid: Vec<Vec<char>> = vec![vec!['.'; grid_width]; grid_height];
    for (k, v) in antenna_map.iter() {
        println!("k:{} v:{:?}", k, v);
        for i in 0..v.iter().len() {
            let a0: Antenna = v[i];
            for j in i + 1..v.iter().len() {
                let a1: Antenna = v[j];
                println!("antenna0 {:?} antenna_1 {:?}", a0, a1);
                let (delta_x, delta_y) = get_antenna_delta(a0, a1);
                println!("delta_x {:?} delta_y {:?}", delta_x, delta_y);
                let antinode0: Antenna = Antenna {
                    x: (a0.x as i32 + delta_x),
                    y: (a0.y as i32 + delta_y),
                    name: ('#'),
                };
                let antinode1: Antenna = Antenna {
                    x: (a1.x as i32 - delta_x),
                    y: (a1.y as i32 - delta_y),
                    name: ('#'),
                };
                if antinode0.x >= 0
                    && antinode0.x < grid_width as i32
                    && antinode0.y >= 0
                    && antinode0.y < grid_height as i32
                    && !antenna_grid_points.contains(&antinode0)
                    && !antinode_grid_points.contains(&antinode0)
                {
                    final_grid[antinode0.y as usize][antinode0.x as usize] = '#';
                    antinode_grid_points.push(antinode0);
                }
                if antinode1.x >= 0
                    && antinode1.x < grid_width as i32
                    && antinode1.y >= 0
                    && antinode1.y < grid_height as i32
                    && !antenna_grid_points.contains(&antinode1)
                    && !antinode_grid_points.contains(&antinode1)
                {
                    final_grid[antinode1.y as usize][antinode1.x as usize] = '#';
                    antinode_grid_points.push(antinode1);
                }
            }
        }
    }
    //println!("final_grid");
    //for a in antenna_grid_points {
    //    final_grid[a.y as usize][a.x as usize] = a.name;
    //}
    //for l in final_grid.iter() {
    //    println!("{:?}", l);
    //}
    antinode_grid_points.len()
}

//fn task2(test_values: &Vec<u64>, numbrs: &Vec<Vec<u64>>) -> u64 {
//    5
//}

fn main() {
    let file_path = "../inputs/aoc_08.txt";
    //let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let ((grid_height, grid_width), antenna_map) = parse_input(&raw_input);

    let task1_solution = task1(&antenna_map, grid_height, grid_width);
    println!("task1 solution is {}", task1_solution);
    //let task2_solution = task2(&test_values, &numbrs);
    //println!("task2 solution is {}", task2_solution);
}
