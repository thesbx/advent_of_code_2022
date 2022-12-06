use std::fs;
use std::collections::HashSet;

fn parse_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let output = contents.chars().collect::<String>();
    output 
}

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
    let results_part1 = sliding_window(&string, 4);
    let results_part2 = sliding_window(&string, 14);
    println!("{:?}", results_part1);
    println!("{:?}", results_part2);
}
