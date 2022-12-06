use std::fs;
use std::collections::HashSet;

fn parse_file(filename: &str) -> Vec<char> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let output = contents.chars().collect::<Vec<char>>();
    output 
}

fn sliding_window(string: &String, n: usize) -> usize { 
    let result = string
    .as_bytes()
    .windows(n)
    .position(|c| c.iter().collect::<HashSet<_>>().len() == n);
    let num = result.unwrap();
    num + n
}

fn collect_string(col: &Vec<char>) -> String{
    let string = col.iter().cloned().collect::<String>();
    string
}

fn main() {
    let data = parse_file("../input.txt");
    let string = collect_string(&data);
    let results_part1 = sliding_window(&string, 4);
    let results_part2 = sliding_window(&string, 14);
    println!("{:?}", results_part1);
    println!("{:?}", results_part2);
}
