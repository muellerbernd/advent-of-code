use std::fs::read_to_string;

fn get_euc_dist(a: &Vec<f64>, b: &Vec<f64>) -> Option<f64> {
    match a.len() == b.len() && a.len() == 3 {
        true => Some(
            (f64::powi(b[0] - a[0], 2) + f64::powi(b[1] - a[1], 2) + f64::powi(b[2] - a[2], 2))
                .sqrt(),
        ),
        false => None,
    }
}

fn parse_input(input: &str) -> Vec<Vec<f64>> {
    let junction_boxes: Vec<Vec<f64>> = input
        .lines()
        .map(|l| l.split(",").map(|c| c.parse::<f64>().unwrap()).collect())
        .collect();
    return junction_boxes;
}

fn task1(junction_boxes: &Vec<Vec<f64>>, n: usize) -> i64 {
    let mut closest_dist: f64 = -1.0;
    let mut closest_a: Vec<f64> = Vec::new();
    let mut closest_b: Vec<f64> = Vec::new();
    for (i, junctionbox) in junction_boxes.iter().enumerate() {
        for j in i + 1..junction_boxes.len() {
            let a = junctionbox.clone();
            let b = junction_boxes[j].clone();
            let dist = get_euc_dist(&a, &b).unwrap();
            if closest_dist == -1.0 {
                closest_dist = dist;
                closest_a = a.clone();
                closest_b = b.clone();
            } else if dist < closest_dist {
                closest_dist = dist;
                closest_a = a.clone();
                closest_b = b.clone();
            }
        }
    }
    println!(
        "dist {:?}, a {:?}, b {:?}",
        closest_dist, closest_a, closest_b
    );

    return 0;
}

fn main() {
    // let file_path = "../inputs/aoc_08.txt";
    let file_path = "test_input.txt";

    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");

    let junction_boxes = parse_input(&raw_input);
    let task1_solution = task1(&junction_boxes, 1000);
    println!("task1 solution is {}", task1_solution);
    // let task2_solution = task2(&raw_input);
    // println!("task2 solution is {}", task2_solution);
}

#[test]
fn test_input() {
    let file_path = "test_input.txt";
    let raw_input = read_to_string(file_path).expect("Should have been able to read the file");
    let junction_boxes = parse_input(&raw_input);
    let task1_solution = task1(&junction_boxes, 10);
    assert_eq!(task1_solution, 40);
    // let task2_solution = task2(&raw_input);
    // assert_eq!(task2_solution, 40);
}
