use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn parse_input(input: &str) -> (Vec<Vec<i32>>, HashMap<i32, Vec<i32>>) {
    let rules_and_updates: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();
    //println!("{:?}", rules_and_updates);
    let rules: Vec<Vec<i32>> = rules_and_updates
        .get(0)
        .unwrap()
        .lines()
        .map(|l| l.split('|').map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in &rules {
        let left = rule.get(0).unwrap();
        let right = rule.get(1).unwrap();
        rules_map
            .entry(*left)
            .and_modify(|v| v.push(*right))
            .or_insert(vec![*right]);
    }
    //println!("rules_map {:?}", rules_map);

    let updates: Vec<Vec<i32>> = rules_and_updates
        .get(1)
        .unwrap()
        .lines()
        .map(|l| l.split(',').map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();
    //let mut num_set: HashSet<i32> = HashSet::new();
    //for update in updates {
    //    for num in update {
    //        num_set.insert(num);
    //    }
    //}
    ////println!("num set {:?}", num_set);
    //let num_vec: Vec<i32> = num_set.clone().into_iter().collect();
    //let mut num_count_map: HashMap<i32, i32> = HashMap::new();
    //for i in 0..num_vec.len() {
    //    let k = num_vec.get(i).unwrap();
    //    println!("k {:?}", k);
    //    num_count_map.entry(*k).or_insert(0);
    //    for (_, value) in rules_map.clone().into_iter() {
    //        if value.contains(k) {
    //            num_count_map.entry(*k).and_modify(|v| *v += 1);
    //        };
    //    }
    //}
    //println!("num_count_map {:?}", num_count_map);
    ////let mut sorted_counts: Vec<_> = num_count_map.into_values().collect::<Vec<_>>();
    ////sorted_counts.sort();
    ////println!("sorted_counts {:?}", sorted_counts);
    ////for v in sorted_counts {
    ////    num_count_map.iter()
    ////        .filter_map(|(key, &val)| if val == v { Some(key) } else { None })
    ////        .collect();
    ////}
    //let mut sort_nums: BTreeMap<i32, i32> = BTreeMap::new();
    //for (k,v) in num_count_map {
    //    sort_nums.insert(v, k);
    //
    //}
    //println!("sort_nums {:?}", sort_nums);
    (updates, rules_map)
}

fn task1(updates: Vec<Vec<i32>>, rules_map: HashMap<i32, Vec<i32>>) -> i32 {
    let mut middle_pages_sum = 0;
    let mut ordered_counter = 0;
    for update in &updates {
        let mut is_in_invalid_order: bool = false;
        for i in 0..update.len() {
            let lhs = update.get(i).unwrap();
            for j in (i + 1)..update.len() {
                let rhs = update.get(j).unwrap();
                if rules_map.contains_key(rhs) && rules_map.get(rhs).unwrap().contains(lhs) {
                    is_in_invalid_order = true;
                }
            }
        }
        //println!("{:?} is_ordered {}", update, is_in_invalid_order);
        if !is_in_invalid_order {
            //println!("{:?} is_ordered {}", update, is_ordered);
            //println!("middle page {:?}", update.get(update.len() / 2).unwrap());
            middle_pages_sum += update.get(update.len() / 2).unwrap();
            ordered_counter += 1;
        }
    }
    println!(
        "ordered_counter {:?} vs {:?}",
        ordered_counter,
        updates.len()
    );
    middle_pages_sum
}

fn task2(updates: Vec<Vec<i32>>, rules_map: HashMap<i32, Vec<i32>>) -> i32 {
    let mut middle_pages_sum = 0;
    let mut ordered_counter = 0;
    for u in 0..updates.len() {
        let mut update: Vec<i32> = updates.get(u).unwrap().to_vec();
        let mut is_in_invalid_order: bool = false;
        for i in 0..update.len() {
            let mut lhs = update.get(i).unwrap().clone();
            for j in (i + 1)..update.len() {
                let rhs = update.get(j).unwrap().clone();
                if rules_map.contains_key(&rhs) && rules_map.get(&rhs).unwrap().contains(&lhs) {
                    is_in_invalid_order = true;
                    update.swap(i, j);
                    lhs = rhs.clone();
                }
            }
        }
        //println!("{:?} is_ordered {}", update, is_in_invalid_order);
        if is_in_invalid_order {
            //println!("{:?} is_ordered {}", update, is_ordered);
            //println!("middle page {:?}", update.get(update.len() / 2).unwrap());
            middle_pages_sum += update.get(update.len() / 2).unwrap();
            ordered_counter += 1;
        }
    }
    println!(
        "ordered_counter {:?} vs {:?}",
        ordered_counter,
        updates.len()
    );
    middle_pages_sum
}

fn main() {
    let file_path = "../inputs/aoc_05.txt";
    //let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    let (updates, rules_map) = parse_input(&raw_input);
    let task1_solution = task1(updates.clone(), rules_map.clone());
    println!("task1 solution is {}", task1_solution);
    let task2_solution = task2(updates.clone(), rules_map.clone());
    println!("task2 solution is {}", task2_solution);
}
