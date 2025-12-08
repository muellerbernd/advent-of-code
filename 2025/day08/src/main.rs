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
    let mut dists: Vec<(f64, usize, usize)> = Vec::new();
    for (i, junctionbox) in junction_boxes.iter().enumerate() {
        let a = junctionbox; // borrow; avoid clone if possible
        for j in i + 1..junction_boxes.len() {
            let b = &junction_boxes[j];
            if let Some(dist) = get_euc_dist(a, b) {
                dists.push((dist, i, j));
            }
        }
    }

    dists.sort_by(|(a, _, _), (b, _, _)| a.partial_cmp(&b).unwrap());
    let mut circuits: Vec<usize> = (0..junction_boxes.len()).collect();
    for (d, a, b) in &dists[..n] {
        circuits = circuits
            .iter()
            .map(|n| if *n == circuits[*a] { circuits[*b] } else { *n })
            .collect();
    }
    let mut circuit_counts = vec![0; junction_boxes.len()];
    for c in circuits {
        circuit_counts[c] += 1;
    }
    circuit_counts.sort();

    return circuit_counts
        .iter()
        .rev()
        .take(3)
        .fold(1, |acc, v| acc * v);
}

fn main() {
    let file_path = "../inputs/aoc_08.txt";

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
