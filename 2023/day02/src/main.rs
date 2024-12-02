use std::collections::HashMap;
use std::fs::read_to_string;

fn get_revealed_cubes(input: &String) -> Vec<Vec<String>> {
    let mut all_revealed_cubes = Vec::new();
    for line in input.lines().collect::<Vec<&str>>() {
        let game_substr: String = line.split(":").last().unwrap().to_string();
        let revealed_cubes: Vec<String> = game_substr.split(",").map(|s| s.to_string()).collect();
        all_revealed_cubes.push(revealed_cubes.clone());
    }
    all_revealed_cubes
}

fn task1(all_revealed_cubes: &Vec<Vec<String>>, max_allowed_cubes: &HashMap<String, usize>) -> i32 {
    let mut possible_run_idxs: Vec<usize> = vec![1; all_revealed_cubes.len()];
    for (i, reveal) in all_revealed_cubes.iter().enumerate() {
        for cubes in reveal {
            let run: Vec<_> = cubes.split(" ").collect();
            let k: String = run.last().unwrap().to_string();
            let v: usize = run.first().unwrap().parse::<usize>().unwrap();
            if v > *max_allowed_cubes.get(&k).unwrap() {
                possible_run_idxs[i] = 0;
            }
        }
    }
    let mut res: i32 = 0;
    for (i, v) in possible_run_idxs.iter().enumerate() {
        res += ((i + 1) * v) as i32;
    }
    res
}

fn task2(all_revealed_cubes: &Vec<Vec<String>>) -> i32 {
    let mut games: Vec<HashMap<String, usize>> = Vec::new();
    for reveal in all_revealed_cubes {
        let mut min_cubes: HashMap<String, usize> = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);
        for cubes in reveal {
            let run: Vec<_> = cubes.split(" ").collect();
            let k: String = run.last().unwrap().to_string();
            let v: usize = run.first().unwrap().parse::<usize>().unwrap();
            if v > *min_cubes.get(&k).unwrap() {
                *min_cubes.get_mut(&k).unwrap() = v;
            }
        }
        games.push(min_cubes);
    }
    let mut power: i32 = 0;
    for g in games {
        let mut game_power: i32 = 1;
        for v in g.values() {
            game_power *= *v as i32;
        }
        power += game_power;
    }
    power
}

fn main() {
    let max_allowed_cubes: HashMap<String, usize> = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);
    let file_path = "../inputs/aoc_02.txt";
    // let file_path = "test_input.txt";

    let mut raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    raw_input = raw_input.replace(": ", ":");
    raw_input = raw_input.replace(", ", ",");
    raw_input = raw_input.replace("; ", ",");
    let all_revealed_cubes = get_revealed_cubes(&raw_input);
    let task1_solution = task1(&all_revealed_cubes, &max_allowed_cubes);
    println!("task1 solution={}", task1_solution);
    let task2_solution = task2(&all_revealed_cubes);
    println!("task2 solution={}", task2_solution);
}
