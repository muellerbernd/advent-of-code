use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::read_to_string;

const RELATIVE_STRENGTHS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const RELATIVE_STRENGTHS_TASK2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn parse_input(raw_input: String) -> Vec<(Vec<usize>, i32)> {
    let v: Vec<(Vec<usize>, i32)> = raw_input
        .lines()
        .map(|l| {
            let camel_cards_raw: Vec<&str> = l.split(" ").collect();
            (
                camel_cards_raw[0]
                    .chars()
                    .map(|c| RELATIVE_STRENGTHS.iter().position(|&r| r == c).unwrap() + 1)
                    .collect(),
                camel_cards_raw[1].parse::<i32>().unwrap(),
            )
        })
        .collect();
    v
}

fn parse_input_task02(raw_input: String) -> Vec<(Vec<usize>, i32)> {
    let v: Vec<(Vec<usize>, i32)> = raw_input
        .lines()
        .map(|l| {
            let camel_cards_raw: Vec<&str> = l.split(" ").collect();
            (
                camel_cards_raw[0]
                    .chars()
                    .map(|c| {
                        RELATIVE_STRENGTHS_TASK2
                            .iter()
                            .position(|&r| r == c)
                            .unwrap()
                            + 1
                    })
                    .collect(),
                camel_cards_raw[1].parse::<i32>().unwrap(),
            )
        })
        .collect();
    v
}

fn task01(camel_card_to_bid: Vec<(Vec<usize>, i32)>) -> i32 {
    let mut all_hands: BTreeMap<i32, Vec<(Vec<usize>, i32)>> = BTreeMap::new();
    for (hand, bid) in camel_card_to_bid {
        let letter_counts: HashMap<usize, i32> = hand.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(*c).or_insert(0) += 1;
            map
        });
        let hand_type: i32 = match letter_counts.len() {
            5 => 0,
            4 => 1,
            3 => match letter_counts.values().max().unwrap() {
                3 => 3,
                _ => 2,
            },
            2 => match letter_counts.values().max().unwrap() {
                4 => 5,
                _ => 4,
            },
            1 => 6,
            _ => {
                println!("default");
                -1
            }
        };
        match all_hands.get_mut(&hand_type) {
            Some(v) => v.push((hand.to_vec(), bid)),
            // _=> println!("not found")
            _ => {
                let mut v: Vec<(Vec<usize>, i32)> = Vec::new();
                v.push((hand.to_vec(), bid));
                all_hands.insert(hand_type, v);
            }
        };
    }
    for (_, v) in all_hands.iter_mut().rev() {
        v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }
    all_hands
        .values()
        .flatten()
        .enumerate()
        .map(|(i, v)| v.1 * (i + 1) as i32)
        .sum()
}

fn task02(camel_card_to_bid: Vec<(Vec<usize>, i32)>) -> i32 {
    let mut all_hands: BTreeMap<i32, Vec<(Vec<usize>, i32)>> = BTreeMap::new();
    for (hand, bid) in camel_card_to_bid {
        let mut letter_counts: BTreeMap<usize, i32> =
            hand.iter().fold(BTreeMap::new(), |mut map, c| {
                *map.entry(*c).or_insert(0) += 1;
                map
            });
        let nr_js: i32 = match letter_counts.remove(&1) {
            Some(x) => x,
            _ => 0,
        };

        if letter_counts.len() == 0 {
            letter_counts.insert(1, 0);
        }
        let max_nr_card: usize = *letter_counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0;
        let max_val = letter_counts.get_mut(&max_nr_card).unwrap();
        *max_val += nr_js;

        let mut mod_letter_counts: Vec<i32> = letter_counts.values().map(|x| *x).collect();

        mod_letter_counts.sort();
        mod_letter_counts.reverse();
        let hand_type: i32 = match mod_letter_counts[0] {
            5 => 6,
            4 => 5,
            3 => match mod_letter_counts[1] {
                2 => 4,
                _ => 3,
            },
            2 => match mod_letter_counts[1] {
                2 => 2,
                _ => 1,
            },
            _ => 0,
        };
        match all_hands.get_mut(&hand_type) {
            Some(v) => v.push((hand, bid)),
            _ => {
                all_hands.insert(hand_type, vec![(hand, bid)]);
            }
        };
    }
    for (_, v) in all_hands.iter_mut().rev() {
        v.sort_by(|a, b| a.0.cmp(&b.0));
    }
    all_hands
        .values()
        .flatten()
        .enumerate()
        .map(|(i, v)| v.1 * (i + 1) as i32)
        .sum()
}

fn main() {
    let file_path = "../inputs/aoc_07.txt";
    // let file_path = "sample_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let camel_card_to_bid: Vec<(Vec<usize>, i32)> = parse_input(raw_input.clone());
    let task01_solution = task01(camel_card_to_bid);
    println!("task01 solution = {}", task01_solution);
    let camel_card_to_bid_task02: Vec<(Vec<usize>, i32)> = parse_input_task02(raw_input.clone());
    let task02_solution = task02(camel_card_to_bid_task02);
    println!("task02 solution = {}", task02_solution);
}
