use std::fs::read_to_string;

fn task1(input: &str) -> i64 {
    let problems: Vec<Vec<String>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
        .collect();
    let num_rows = problems.len() - 1;
    let operators = problems.last().unwrap();
    let mut running_vals: Vec<i64> = problems
        .first()
        .unwrap()
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    for row in 1..num_rows {
        let vals: Vec<i64> = problems[row]
            .clone()
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        running_vals = running_vals
            .iter()
            .zip(vals.iter())
            .enumerate()
            .map(|(_i, (x, y))| match operators[_i].as_str() {
                "*" => x * y,
                "+" => x + y,
                _ => 0,
            })
            .collect();
    }

    return running_vals.iter().sum();
}

fn task2(input: &str) -> i64 {
    let problems: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let operators = problems.last().unwrap();
    let mut cols: Vec<(usize, usize)> = Vec::new();
    let mut last_col_start = 0;
    for (idx, v) in operators.iter().enumerate() {
        match *v == '*' || *v == '+' {
            true => {
                if idx == 0 {
                    last_col_start = idx;
                } else {
                    cols.push((last_col_start, idx - 2));
                    last_col_start = idx;
                }
            }
            false => (),
        }
    }
    println!("cols {:?}", cols);
    return 0;
}

// fn task2(input: &str) -> i64 {
//     // Read all lines
//     let mut lines: Vec<&str> = input.lines().collect();
//     if lines.is_empty() {
//         return 0;
//     }
//
//     // The last line is the operator row
//     let op_row = lines.pop().unwrap_or("");
//
//     // Remaining lines are digit rows
//     let digit_rows = lines.len();
//
//     // Determine the maximum width to pad lines
//     let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
//
//     // Build grid of chars, padding with spaces
//     let mut grid: Vec<Vec<char>> = Vec::with_capacity(digit_rows);
//     for &ln in &lines {
//         let mut row: Vec<char> = ln.chars().collect();
//         if row.len() < width {
//             row.resize(width, ' ');
//         }
//         grid.push(row);
//     }
//
//     // If there are no digit rows, nothing to do
//     if grid.is_empty() || op_row.chars().count() == 0 {
//         return 0;
//     }
//
//     // Helper to check if a column is a separator (all spaces in digit rows)
//     let is_separator = |col: usize| -> bool {
//         for r in 0..digit_rows {
//             if grid[r][col] != ' ' {
//                 return false;
//             }
//         }
//         true
//     };
//
//     // Determine blocks (start..end) of contiguous non-separator columns
//     let mut blocks: Vec<(usize, usize)> = Vec::new();
//     let mut c = 0usize;
//     while c < width {
//         if is_separator(c) {
//             c += 1;
//             continue;
//         }
//         // start of a block
//         let start = c;
//         while c < width && !is_separator(c) {
//             c += 1;
//         }
//         let end = if c > 0 { c - 1 } else { 0 };
//         blocks.push((start, end));
//     }
//
//     let mut grand_total: i64 = 0;
//
//     for (start, end) in blocks {
//         // Gather numbers for this block by reading digits per column within the block
//         let mut numbers: Vec<i64> = Vec::new();
//         for col in start..=end {
//             let mut s = String::new();
//             for r in 0..digit_rows {
//                 let ch = grid[r][col];
//                 if ch.is_ascii_digit() {
//                     s.push(ch);
//                 }
//             }
//             if !s.is_empty() {
//                 if let Ok(n) = s.parse::<i64>() {
//                     numbers.push(n);
//                 }
//             }
//         }
//
//         if numbers.is_empty() {
//             continue;
//         }
//
//         // Determine operator for this block from the op_row within [start..=end]
//         let mut op = '+';
//         for ch in op_row.chars().skip(start).take(end - start + 1) {
//             if ch == '+' || ch == '*' {
//                 op = ch;
//                 break;
//             }
//         }
//
//         // Compute the per-block value left-to-right
//         let mut val = numbers[0];
//         for &num in numbers.iter().skip(1) {
//             match op {
//                 '+' => val += num,
//                 '*' => val *= num,
//                 _ => {}
//             }
//         }
//
//         grand_total += val;
//     }
//
//     grand_total
// }

fn main() {
    let file_path = "../inputs/aoc_06.txt";
    let file_path = "test_input.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");

    let task1_solution = task1(&raw_input);
    println!("task1 solution is {}", task1_solution);
    let task2_solution = task2(&raw_input);
    println!("task2 solution is {}", task2_solution);
}

#[test]
fn test_input() {
    let file_path = "test_input.txt";
    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let task1_solution = task1(&raw_input);
    assert_eq!(task1_solution, 4277556);
    let task2_solution = task2(&raw_input);
    assert_eq!(task2_solution, 3263827);
}
