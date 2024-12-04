use std::fs::read_to_string;

fn parse_reports(lines: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let report: Vec<i32> = line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
        reports.push(report);
    }
    reports
}

fn get_diffs(report: &Vec<i32>) -> Vec<i32> {
    let mut diffs: Vec<i32> = Vec::new();
    for (i, l) in report.iter().skip(1).enumerate() {
        diffs.push(report.get(i).unwrap() - l);
    }
    diffs
}

fn check_report(report: &Vec<i32>) -> bool {
    let diffs = get_diffs(report);
    let min: i32 = *diffs.iter().min().unwrap();
    let max: i32 = *diffs.iter().max().unwrap();
    min.signum() == max.signum() && min > -4 && max < 4 && min != 0 && max != 0
}

fn task1(lines: &Vec<&str>) -> u32 {
    let reports: Vec<Vec<i32>> = parse_reports(lines);
    let mut safe_reports_counter: u32 = 0;
    for report in reports {
        if check_report(&report) {
            safe_reports_counter += 1;
        }
    }
    safe_reports_counter
}

fn task2(lines: &Vec<&str>) -> u32 {
    let reports: Vec<Vec<i32>> = parse_reports(lines);
    let mut safe_reports_counter: u32 = 0;
    for report in reports {
        if check_report(&report) {
            safe_reports_counter += 1;
        } else {
            for i in 0..report.len() {
                let mut new_report: Vec<i32> = report.clone();
                let _ = new_report.remove(i);
                if check_report(&new_report) {
                    safe_reports_counter += 1;
                    break;
                }
            }
        }
    }
    safe_reports_counter
}

fn main() {
    let file_path = "../inputs/aoc_02.txt";
    //let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    let task1_solution = task1(&raw_input.lines().collect());
    println!("task1 solution is {}", task1_solution);
    let task2_solution = task2(&raw_input.lines().collect());
    println!("task2 solution is {}", task2_solution);
}
