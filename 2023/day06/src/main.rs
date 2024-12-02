use regex::Regex;
use std::fs::read_to_string;
use std::iter::zip;

fn parse_input(raw_input: &String) -> Vec<Vec<usize>> {
    let re = Regex::new(r"\s+").unwrap();
    let race_description: Vec<String> = raw_input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            re.replace_all(l.split_once(":").unwrap().1.trim(), " ")
                .to_string()
        })
        .collect::<Vec<String>>();
    let time_to_dist: Vec<Vec<usize>> = race_description
        .iter()
        .map(|l| l.split(" ").map(|v| v.parse::<usize>().unwrap()).collect())
        .collect();
    time_to_dist
}

fn task01(time_to_dist: &Vec<Vec<usize>>) -> i32 {
    println!("{:?}", time_to_dist);
    let mut nr_winning_races: i32 = 1;
    for (time, dist) in zip(&time_to_dist[0], &time_to_dist[1]) {
        println!("{} {}", time, dist);
        let mut nr_winning_tries = 0;
        for ts in 1..*time {
            let race_dist: usize = ts * (time - ts);
            if race_dist > *dist {
                nr_winning_tries += 1;
            }
        }
        nr_winning_races *= nr_winning_tries;
    }
    nr_winning_races
}

fn task02(time_to_dist: &Vec<Vec<usize>>) -> i32 {
    println!("{:?}", time_to_dist);
    let the_one_race: (i64, i64) = (
        time_to_dist[0]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i64>()
            .unwrap(),
        time_to_dist[1]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i64>()
            .unwrap(),
    );
    let time = the_one_race.0;
    let dist = the_one_race.1;
    let mut nr_winning_tries = 0;
    for ts in 1..time {
        let race_dist: i64 = ts * (time - ts);
        if race_dist > dist {
            nr_winning_tries += 1;
        }
    }
    nr_winning_tries
}
fn main() {
    let file_path = "../inputs/aoc_06.txt";
    // let file_path = "sample_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let time_to_dist: Vec<Vec<usize>> = parse_input(&raw_input);
    let task01_solution = task01(&time_to_dist);
    println!("task01 solution = {}", task01_solution);
    let task02_solution = task02(&time_to_dist);
    println!("task02 solution = {}", task02_solution);
}
