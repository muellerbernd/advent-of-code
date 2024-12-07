//use regex::Regex;
use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn task1(char_matr: Vec<Vec<char>>) -> i32 {
    let mut x_idxs: Vec<(usize, usize)> = Vec::new();
    for (i, l) in char_matr.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            //println!("{:#?} {:#?} {:#?}", i, j, c);
            if *c == 'X' {
                x_idxs.push((i, j))
            }
        }
    }
    let cols: usize = char_matr.get(0).unwrap().len();
    let rows: usize = char_matr.len();
    println!("matr_height {}, matr_width {}", rows, cols);
    //let transposed: Vec<Vec<_>> = (0..cols)
    //    .map(|col| (0..rows).map(|row| char_matr[row][col]).collect())
    //    .collect();
    //println!("transposed {:?}", transposed);

    let mut xmas_counter = 0;
    for (row, col) in x_idxs {
        //println!("row {}, col {}", row, col);
        //println!("row {:?}", char_matr[row]);
        // right in row
        if col + 3 < cols {
            let s: String = char_matr[row][col..=col + 3]
                .iter()
                .cloned()
                .collect::<String>();
            if s == "XMAS" {
                xmas_counter += 1;
                //println!("right in row {:?}", s);
            }
        }
        // left in row
        if col >= 3 {
            let s: String = char_matr[row][col - 3..=col]
                .iter()
                .cloned()
                .rev()
                .collect::<String>();
            if s == "XMAS" {
                xmas_counter += 1;
                //println!("left in row {:?}", s);
            }
        }
        // down in col
        if row + 3 < rows {
            let s: String = format!(
                "{}{}{}{}",
                char_matr[row][col],
                char_matr[row + 1][col],
                char_matr[row + 2][col],
                char_matr[row + 3][col]
            );
            if s == "XMAS" {
                xmas_counter += 1;
                //println!("down in col {:?}", s);
            }
        }
        // up in col
        if row >= 3 {
            let s: String = format!(
                "{}{}{}{}",
                char_matr[row][col],
                char_matr[row - 1][col],
                char_matr[row - 2][col],
                char_matr[row - 3][col],
            );
            if s == "XMAS" {
                xmas_counter += 1;
                //println!("up in col {:?}", s);
            }
        }
        // up left
        if row >= 3 && col >= 3 {
            let s: String = format!(
                "{}{}{}{}",
                char_matr[row][col],
                char_matr[row - 1][col - 1],
                char_matr[row - 2][col - 2],
                char_matr[row - 3][col - 3],
            );
            if s == "XMAS" {
                xmas_counter += 1;
                //println!("diag up left {:?}", s);
            }
        }
        // up right
        if row >= 3 && col + 3 < cols {
            let s: String = format!(
                "{}{}{}{}",
                char_matr[row][col],
                char_matr[row - 1][col + 1],
                char_matr[row - 2][col + 2],
                char_matr[row - 3][col + 3],
            );
            if s == "XMAS" {
                xmas_counter += 1;
                //println!("diag up right {:?}", s);
            }
        }
        // down right
        if row + 3 < rows && col + 3 < cols {
            let s: String = format!(
                "{}{}{}{}",
                char_matr[row][col],
                char_matr[row + 1][col + 1],
                char_matr[row + 2][col + 2],
                char_matr[row + 3][col + 3],
            );
            if s == "XMAS" {
                xmas_counter += 1;
                //println!("diag down right {:?}", s);
            }
        }
        // down left
        if row + 3 < rows && col >= 3 {
            let s: String = format!(
                "{}{}{}{}",
                char_matr[row][col],
                char_matr[row + 1][col - 1],
                char_matr[row + 2][col - 2],
                char_matr[row + 3][col - 3],
            );
            if s == "XMAS" {
                xmas_counter += 1;
                //println!("diag down left {:?}", s);
            }
        }
    }
    xmas_counter
}
fn task2(char_matr: Vec<Vec<char>>) -> i32 {
    let mut a_idxs: Vec<(usize, usize)> = Vec::new();
    for (i, l) in char_matr.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            //println!("{:#?} {:#?} {:#?}", i, j, c);
            if *c == 'A' {
                a_idxs.push((i, j))
            }
        }
    }
    let cols: usize = char_matr.get(0).unwrap().len();
    let rows: usize = char_matr.len();
    println!("matr_height {}, matr_width {}", rows, cols);

    let mut result = 0;
    for (row, col) in a_idxs {
        let mut mas_counter = 0;
        // upper left to lower right
        if row >= 1 && col + 1 < cols && row + 1 < rows && col >= 1 {
            let s: String = format!(
                "{}{}{}",
                char_matr[row - 1][col - 1],
                char_matr[row][col],
                char_matr[row + 1][col + 1],
            );
            if s == "MAS" || s == "SAM" {
                mas_counter += 1;
            }
        }
        // upper right to lower left
        if row >= 1 && col + 1 < cols && row + 1 < rows && col >= 1 {
            let s: String = format!(
                "{}{}{}",
                char_matr[row - 1][col + 1],
                char_matr[row][col],
                char_matr[row + 1][col - 1],
            );
            if s == "MAS" || s == "SAM" {
                mas_counter += 1;
                //println!("diag up right {:?}", s);
            }
        }
        if mas_counter % 2 == 0 && mas_counter > 0 {
            result += 1;
        }
    }
    result
}

fn main() {
    let file_path = "../inputs/aoc_04.txt";
    //let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    let input = parse_input(&raw_input);
    let task1_solution = task1(input.clone());
    println!("task1 solution is {}", task1_solution);
    let task2_solution = task2(input.clone());
    println!("task2 solution is {}", task2_solution);
}
