use std::collections::BTreeMap;
use std::fs::read_to_string;

fn parse_input(input: &String) -> Vec<i32> {
    let mut parsed_input: Vec<i32> = Vec::new();
    let mut file_id = 0;
    for (i, c) in input.chars().enumerate() {
        let repetitions: usize = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            parsed_input.extend(vec![file_id; repetitions]);
            file_id += 1;
        } else {
            parsed_input.extend(vec![-1; repetitions]);
        }
    }
    parsed_input
}

fn get_checksum(defrag_input: Vec<i32>) -> i64 {
    let mut result: i64 = 0;
    for (pos, file_id) in defrag_input.iter().enumerate() {
        if file_id.is_positive() {
            result += pos as i64 * (*file_id as i64);
        }
    }
    result
}

fn task1(parsed_input: &Vec<i32>) -> i64 {
    let mut defrag_input = parsed_input.clone();
    let mut curr_swap_idx = parsed_input.len() - 1;
    let mut curr_start_idx = 0;
    while curr_start_idx < curr_swap_idx {
        if defrag_input[curr_swap_idx] == -1 {
            curr_swap_idx -= 1;
            continue;
        }
        if defrag_input[curr_start_idx] == -1 {
            defrag_input.swap(curr_start_idx, curr_swap_idx);
            curr_swap_idx -= 1;
        }
        curr_start_idx += 1;
    }

    get_checksum(defrag_input)
}

#[derive(Debug, Clone, Copy)]
struct DiskChunk {
    file_id: usize,
    length: usize,
    is_file: bool,
}

fn parse_files(input: &String) -> Vec<DiskChunk> {
    let mut parsed_input: Vec<DiskChunk> = Vec::new();
    for (i, c) in input.chars().enumerate() {
        let repetitions: usize = c.to_digit(10).unwrap() as usize;
        let disk_chunk: DiskChunk = DiskChunk {
            file_id: (i / 2),
            length: (repetitions),
            is_file: (i % 2 == 0),
        };
        parsed_input.push(disk_chunk);
    }
    parsed_input
}

fn task2(parsed_input: &Vec<DiskChunk>) -> i64 {
    let mut disk_chunks: Vec<DiskChunk> = parsed_input.clone();
    let mut curr_file_idx = disk_chunks.len() - (disk_chunks.len() % 2);
    while curr_file_idx > 1 {
        // skip if not a file
        if !disk_chunks[curr_file_idx].is_file {
            curr_file_idx -= 1;
            continue;
        }
        // if we come here, we can try to move file
        let l = disk_chunks[curr_file_idx].length;
        // now search for empty places
        for idx in 1..curr_file_idx {
            // skip file chunks
            if disk_chunks[idx].is_file {
                continue;
            }
            // free chunk with exactly enough space for current file
            if disk_chunks[idx].length == l {
                disk_chunks[idx] = disk_chunks[curr_file_idx];
                disk_chunks[curr_file_idx].is_file = false;
                break;
            }
            // free chunk with more than enough space
            if disk_chunks[idx].length > l {
                disk_chunks[idx].length -= l;
                disk_chunks.insert(idx, disk_chunks[curr_file_idx]);
                disk_chunks[curr_file_idx + 1].is_file = false;
                break;
            }
        }
        curr_file_idx -= 1;
    }
    let mut defrag_input: Vec<i32> = Vec::new();
    for chunk in disk_chunks {
        //println!("{:?}", chunk);
        let inserted: i32 = match chunk.is_file {
            true => chunk.file_id as i32,
            false => -1,
        };
        //println!("{:?}", inserted);
        defrag_input.extend(vec![inserted; chunk.length]);
    }
    get_checksum(defrag_input)
}

fn main() {
    let file_path = "../inputs/aoc_09.txt";
    //let file_path = "test_input.txt";

    let raw_input: String = read_to_string(file_path)
        .expect("Should have been able to read the file")
        .lines()
        .collect::<Vec<_>>()[0]
        .to_string();
    let parsed_input: Vec<i32> = parse_input(&raw_input);
    //println!("parsed_input {:?}", parsed_input);

    let task1_solution = task1(&parsed_input);
    println!("task1 solution is {}", task1_solution);
    let parsed_files: Vec<DiskChunk> = parse_files(&raw_input);
    let task2_solution = task2(&parsed_files);
    println!("task2 solution is {}", task2_solution);
    //let task2_solution = task2(&antenna_map, grid_height, grid_width);
    //println!("task2 solution is {}", task2_solution);
}
