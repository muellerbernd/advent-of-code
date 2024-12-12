use std::fs::read_to_string;

fn parse_input(input: &str) -> ((usize, usize), Vec<(i32, i32)>) {
    let mut obstacle_list: Vec<(i32, i32)> = Vec::new();
    let mut start_point: (usize, usize) = (0, 0);
    for (i, line) in input.lines().map(|l| l.to_string()).into_iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => obstacle_list.push((i as i32, j as i32)),
                '^' => {
                    start_point.0 = i;
                    start_point.1 = j
                }
                _ => continue,
            }
        }
    }
    (start_point, obstacle_list)
}

fn task1(start_point: (usize, usize), obstacle_list: Vec<(i32, i32)>, length: i32) -> i32 {
    let (mut x, mut y): (i32, i32) = (start_point.0 as i32, start_point.1 as i32);
    let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir_ind = 0;
    let mut traversed_grid: Vec<Vec<bool>> = vec![vec![false; length as usize]; length as usize];
    loop {
        traversed_grid[x as usize][y as usize] = true;
        let (next_x, next_y) = (x + directions[dir_ind].0, y + directions[dir_ind].1);
        if next_x >= length || next_x < 0 || next_y >= length || next_y < 0 {
            break;
        }
        if obstacle_list.contains(&(next_x, next_y)) {
            dir_ind = (dir_ind + 1) % 4
        } else {
            (x, y) = (next_x, next_y)
        }
    }
    traversed_grid
        .iter()
        .map(|v| v.iter().map(|v| *v as i32).sum::<i32>())
        .sum()
}

fn task2(start_point: (usize, usize), obstacle_list: Vec<(i32, i32)>, length: i32) -> i32 {
    let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut loop_counter: i32 = 0;
    for i in 0..length {
        for j in 0..length {
            let (mut x, mut y): (i32, i32) = (start_point.0 as i32, start_point.1 as i32);
            let mut new_obstacle_list = obstacle_list.clone();
            new_obstacle_list.push((i, j));
            let mut dir_ind = 0;
            let mut traversed_grid_directions: Vec<Vec<Vec<bool>>> =
                vec![vec![vec![false; directions.len()]; length as usize]; length as usize];
            loop {
                if traversed_grid_directions[x as usize][y as usize][dir_ind as usize] {
                    loop_counter += 1;
                    break;
                } else {
                    traversed_grid_directions[x as usize][y as usize][dir_ind as usize] = true;
                }
                let (next_x, next_y) = (x + directions[dir_ind].0, y + directions[dir_ind].1);
                if next_x >= length || next_x < 0 || next_y >= length || next_y < 0 {
                    break;
                }
                if new_obstacle_list.contains(&(next_x, next_y)) {
                    dir_ind = (dir_ind + 1) % 4
                } else {
                    (x, y) = (next_x, next_y)
                }
            }
        }
    }
    loop_counter
}

fn main() {
    let file_path = "../inputs/aoc_06.txt";
    //let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    let (start_point, obstacle_list) = parse_input(&raw_input);
    let task1_solution = task1(
        start_point,
        obstacle_list.clone(),
        raw_input.lines().count() as i32,
    );
    println!("task1 solution is {}", task1_solution);
    let task2_solution = task2(
        start_point,
        obstacle_list.clone(),
        raw_input.lines().count() as i32,
    );
    println!("task2 solution is {}", task2_solution);
}
