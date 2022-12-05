use std::fs;

fn parse_file(filename: &str) -> Vec<Vec<Vec<i32>>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut groups: Vec<Vec<Vec<i32>>> = Vec::new();
    for group in contents.split("\n") {
        let mut items: Vec<Vec<i32>> = Vec::new();
        for item in group.split(",") {
            let mut i: Vec<i32> = Vec::new();
            for x in item.split("-") {
                i.push(x.to_string().parse::<i32>().unwrap());
            }
            items.push(i);
        }
        groups.push(items);
    }
    groups
}
fn main() {
    println!("Hello, world!");
}
