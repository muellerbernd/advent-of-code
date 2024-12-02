use std::fs::read_to_string;

fn parse_input(raw_input: &String) -> Vec<Vec<Vec<usize>>> {
    raw_input
        .split("\n\n")
        .map(|l| {
            l.split_once(":")
                .unwrap()
                .1
                .split("\n")
                .filter(|x| !x.is_empty())
                .map(|l| l.split(" ").map(|e| e.parse::<usize>().unwrap()).collect())
                .collect()
        })
        .collect::<Vec<_>>()
}

fn task01(puzzle_input: &Vec<Vec<Vec<usize>>>) -> usize {
    let seeds: Vec<usize> = puzzle_input[0][0].clone();
    let maps = &puzzle_input[1..];

    let mut locations: Vec<usize> = Vec::new();
    for seed in seeds {
        let mut scratch_var = seed;
        for map in maps {
            for line in map {
                let (dest_start, src_start, range): (usize, usize, usize) =
                    (line[0], line[1], line[2]);
                if scratch_var >= src_start && scratch_var < (src_start + range) {
                    scratch_var = dest_start + (scratch_var - src_start);
                    break;
                }
            }
        }
        locations.push(scratch_var);
    }
    *locations.iter().min().unwrap()
}

fn task02(puzzle_input: &Vec<Vec<Vec<usize>>>) -> usize {
    let parsed_seeds: Vec<usize> = puzzle_input[0][0].clone();
    let mut scratchpad: Vec<(usize, usize)> = (0..(parsed_seeds.len() / 2))
        .map(|i| {
            (
                parsed_seeds[i * 2],
                parsed_seeds[(i * 2) + 1] + parsed_seeds[i * 2],
            )
        })
        .collect();
    dbg!(scratchpad.clone());
    let maps = &puzzle_input[1..];
    for map in maps {
        let mut counter = 0;
        while counter < scratchpad.len() {
            for line in map {
                let (dest_start, src_start, range): (usize, usize, usize) =
                    (line[0], line[1], line[2]);
                let (low_is_in, up_is_in, is_around) =
                    between(scratchpad[counter], src_start, range);
                if low_is_in && up_is_in {
                    scratchpad[counter] = (
                        (scratchpad[counter].0 - src_start) + dest_start,
                        (scratchpad[counter].1 - src_start) + dest_start,
                    );
                    break;
                } else if low_is_in {
                    scratchpad.push((src_start + range, scratchpad[counter].1));
                    scratchpad[counter] = (
                        (scratchpad[counter].0 - src_start) + dest_start,
                        dest_start + range,
                    );
                    break;
                } else if up_is_in {
                    scratchpad.push((scratchpad[counter].0, src_start));
                    scratchpad[counter] =
                        (dest_start, (scratchpad[counter].1 - src_start) + dest_start);
                    break;
                } else if is_around {
                    scratchpad.push((scratchpad[counter].0, src_start));
                    scratchpad.push((src_start + range, scratchpad[counter].1));
                    scratchpad[counter] = (dest_start, dest_start + range);
                    break;
                }
            }
            counter += 1;
        }
    }
    *scratchpad.iter().map(|(v, _)| v).min().unwrap()
}
fn between(v: (usize, usize), lower: usize, range: usize) -> (bool, bool, bool) {
    (
        v.0 >= lower && v.0 < lower + range,
        v.1 > lower && v.1 <= lower + range,
        v.0 < lower && v.1 > lower + range,
    )
}

fn main() {
    let file_path = "../inputs/aoc_05.txt";
    // let file_path = "sample_input.txt";

    let mut raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    raw_input = raw_input.replace(": ", ":");
    println!("{}", raw_input);
    let parsed_input = parse_input(&raw_input);
    let task01_solution = task01(&parsed_input);
    println!("task01 solution = {}", task01_solution);
    let task02_solution = task02(&parsed_input);
    println!("task02 solution = {}", task02_solution);
}
