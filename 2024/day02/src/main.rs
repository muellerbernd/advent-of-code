use std::fs::read_to_string;

fn task1(lines: &Vec<&str>) -> u32 {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let report: Vec<i32> = line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
        reports.push(report);
    }
    println!("{:?}", reports);
    let mut diff_reports: Vec<Vec<i32>> = Vec::new();
    for report in reports {
        let mut diffs: Vec<i32> = Vec::new();
        for (i, l) in report.iter().skip(1).enumerate() {
            diffs.push(report.get(i).unwrap() - l);
        }
        diff_reports.push(diffs);
    }
    println!("{:?}", diff_reports);
    let mut safe_reports_counter: u32 = 0;
    for diffs in diff_reports {
        let min: i32 = *diffs.iter().min().unwrap();
        let max: i32 = *diffs.iter().max().unwrap();
        if min.signum() == max.signum() && min > -4 && max < 4 && min != 0 && max != 0 {
            safe_reports_counter += 1;
        }
    }
    safe_reports_counter
}

//fn task2(lines: &Vec<&str>) -> usize {
//    let mut left: Vec<usize> = Vec::new();
//    let mut right: Vec<usize> = Vec::new();
//    for line in lines {
//        let nums: Vec<usize> = line
//            .split("   ")
//            .map(|n| n.parse::<usize>().unwrap())
//            .collect();
//        left.push(*nums.get(0).unwrap());
//        right.push(*nums.get(1).unwrap());
//    }
//    let similarity: Vec<usize> = left
//        .iter()
//        .map(|&e| e * right.iter().filter(|x| **x == e).count())
//        .collect();
//    similarity.iter().sum()
//}

fn main() {
    //let file_path = "../inputs/aoc_02.txt";
    let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    let task1_solution = task1(&raw_input.lines().collect());
    println!("task1 solution is {}", task1_solution);
    //let task2_solution = task2(&raw_input.lines().collect());
    //println!("task2 solution is {}", task2_solution);
}
