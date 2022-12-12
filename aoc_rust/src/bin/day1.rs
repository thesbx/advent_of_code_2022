use std::fs;

/// Parses a file and returns a vector of integers grouped into smaller vectors.
fn parse_file(filename: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut groups: Vec<Vec<i32>> = Vec::new();
    for group in contents.split("\n\n") {
        let mut items: Vec<i32> = Vec::new();
        for item in group.split_whitespace() {
            items.push(item.parse::<i32>().unwrap());
        }
        groups.push(items);
    }
    groups
}

fn main() {
    let items: Vec<Vec<i32>> = parse_file("inputs/day1/list.txt");

    let mut elves: Vec<i32> = Vec::new();

    for lines in items {
        elves.push(aoc_lib::sum(&lines));
    }

    let top_elves: Vec<i32> = aoc_lib::nlargest(&elves, 3);

    println!("Top 3 elves: {:?}", top_elves);
    println!("{:?}", aoc_lib::largest(&elves));
    println!("{:?}", aoc_lib::sum(&top_elves));
    
}
