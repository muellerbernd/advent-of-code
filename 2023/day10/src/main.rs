use std::fs::read_to_string;

fn parse_input(raw_input: &String) -> (Vec<Vec<char>>, (usize, usize)) {
    let field: Vec<Vec<char>> = raw_input.lines().map(|l| l.chars().collect()).collect();
    let mut start_point: (usize, usize) = (0, 0);
    // let start_xy = grid.iter().enumerate().map(|(r,l)| l.iter().enumerate().);
    for (row, line) in field.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == 'S' {
                start_point = (col, row);
                break;
            }
        }
    }
    (field, start_point)
}

fn main() {
    // let file_path = "../inputs/aoc_10.txt";
    let file_path = "sample_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let (field, start_point): (Vec<Vec<char>>, (usize, usize)) = parse_input(&raw_input);
    println!("{:?} {:?}", field, start_point);
    let possible_startings: Vec<(usize,usize)> = [
            (start_point.0 as i32 + 1, start_point.1 as i32),
            (start_point.0 as i32 - 1, start_point.1 as i32),
            (start_point.0 as i32, start_point.1 as i32 + 1),
            (start_point.0 as i32, start_point.1 as i32 - 1),
        ]
        .to_vec()
        .iter()
        .filter(|(col, row)| col >= &0 && row >= &0).map(|(col,row)| (*col as usize, *row as usize)).collect()
;
    for (col, row) in possible_startings {
        let tile: char = match field.get(row) {
            Some(r) => match r.get(col) {
                Some(c) => *c,
                None => continue,
            },
            None => continue,
        };
        println!("tile {:?}", tile);
        // TODO: implement function that follows the pipe
    }
}
