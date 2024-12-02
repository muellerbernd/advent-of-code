use std::collections::HashMap;
use std::fs::read_to_string;
use std::usize;

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}

fn parse_input(raw_input: &String) -> (String, HashMap<String, Vec<String>>) {
    let (navigation, parsed_network) = raw_input.split_once("\n\n").unwrap();
    let mut network: HashMap<String, Vec<String>> = HashMap::new();
    for n in parsed_network.lines() {
        let (k, v) = n.split_once(" = ").unwrap();
        let vals: Vec<String> = v.split(", ").map(|x| x.to_string()).collect();
        network.insert(k.to_string(), vals);
    }
    (navigation.to_string(), network)
}

fn task01(navigation: &String, network: &HashMap<String, Vec<String>>) {
    let nav: Vec<i32> = navigation
        .chars()
        .map(|c| match c {
            'R' => 1,
            'L' => 0,
            _ => -1,
        })
        .collect();
    let mut way_points: Vec<String> = Vec::new();
    let mut curr_waypoint: String = "AAA".to_string();
    let goal: String = "ZZZ".to_string();
    let mut goal_reached = false;
    let mut nav_counter = 0;
    while !goal_reached {
        let dir: usize = nav[nav_counter % nav.len()] as usize;
        way_points.push(curr_waypoint.to_string());
        let next_waypoint: String = network.get(&curr_waypoint).unwrap()[dir].to_string();
        if curr_waypoint == goal.to_string() {
            println!("goal_reached");
            goal_reached = true;
        } else {
            curr_waypoint = next_waypoint;
            nav_counter += 1;
        }
    }
    println!("task01 goal was reached after {} steps", nav_counter);
}
fn task02(navigation: &String, network: &HashMap<String, Vec<String>>) {
    let nav: Vec<i32> = navigation
        .chars()
        .map(|c| match c {
            'R' => 1,
            'L' => 0,
            _ => -1,
        })
        .collect();
    let start_points: Vec<String> = network
        .keys()
        .map(|k| k.to_owned())
        .filter(|k| k.ends_with("A"))
        .collect();
    let mut start_point_to_waypoints: HashMap<String, Vec<String>> = HashMap::new();
    for start_point in start_points {
        let mut way_points: Vec<String> = Vec::new();
        let mut curr_waypoint: String = start_point.clone();
        let mut goal_reached = false;
        let mut nav_counter = 0;
        while !goal_reached {
            let dir: usize = nav[nav_counter % nav.len()] as usize;
            way_points.push(curr_waypoint.to_string());
            let next_waypoint: String = network.get(&curr_waypoint).unwrap()[dir].to_string();
            if curr_waypoint.ends_with("Z") {
                println!("goal_reached");
                goal_reached = true;
                start_point_to_waypoints.insert(start_point.clone(), way_points.clone());
            } else {
                curr_waypoint = next_waypoint;
                nav_counter += 1;
            }
        }
    }
    let way_lengths: Vec<usize> = start_point_to_waypoints
        .values()
        .map(|v| v.len() - 1)
        .collect();
    println!("{:?}", way_lengths);
    let mut curr_lcm = way_lengths[0];
    for i in 1..way_lengths.len() {
        curr_lcm = lcm(curr_lcm, way_lengths[i]);
    }
    println!("task02 goal was reached after {} steps", curr_lcm);
}

fn main() {
    let file_path = "../inputs/aoc_08.txt";
    // let file_path = "sample_input.txt";

    let mut raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    raw_input = raw_input.replace("(", "").replace(")", "");
    let (navigation_task01, network_task01): (String, HashMap<String, Vec<String>>) =
        parse_input(&raw_input);
    task01(&navigation_task01, &network_task01);

    let file_path = "../inputs/aoc_08.txt";
    // let file_path = "sample_input2.txt";
    let mut raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    raw_input = raw_input.replace("(", "").replace(")", "");
    let (navigation_task02, network_task02): (String, HashMap<String, Vec<String>>) =
        parse_input(&raw_input);
    task02(&navigation_task02, &network_task02);
    // let camel_card_to_bid: Vec<(Vec<usize>, i32)> = parse_input(raw_input.clone());
    // let task01_solution = task01(camel_card_to_bid);
    // println!("task01 solution = {}", task01_solution);
    // let camel_card_to_bid_task02: Vec<(Vec<usize>, i32)> = parse_input_task02(raw_input.clone());
    // let task02_solution = task02(camel_card_to_bid_task02);
    // println!("task02 solution = {}", task02_solution);
}
