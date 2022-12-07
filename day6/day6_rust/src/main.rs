use std::fs;
use std::collections::HashSet;

fn parse_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let output = contents.chars().collect::<String>();
    output 
}

fn parse_file_to_vec(filename: &str) -> Vec<char>{
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let output = contents.chars().collect::<Vec<char>>();
    output 
}

/// First attempt at sliding window before knowing what the algorythm was.
fn sliding_window_no_overlap(slice: &Vec<char>, n: usize) -> Vec<String> {
    let mut items = Vec::new();
    for c in slice.chunks(n) {
        let mut unique = HashSet::new();
        let value = c.iter().all(|x| unique.insert(x));
        let string = c.iter().cloned().collect::<String>();
        if value {
            items.push(string);
        }
    }
    items
}

/// Set window of n length and scan string for first index where string does not have duplicates
fn sliding_window(str: &String, n: usize) -> usize { 
    let result = str 
    .as_bytes()
    .windows(n)
    .position(|c| {
        c.iter().collect::<HashSet<_>>().len() == n
    });
    let num = result.unwrap();
    num + n
}

fn main() {
    let string = parse_file("../input.txt");
    let vec = parse_file_to_vec("../input.txt");
    let results_part1 = sliding_window_no_overlap(&vec, 14);
    let results_part2 = sliding_window(&string, 14);
    let target = string.find(&results_part1[0]);
    println!("{:?}", target);
    println!("{:?}", results_part2);
}
