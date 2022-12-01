use std::fs;

// function to find sum of items in a vector
fn sum(items: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in items {
        sum += item;
    }
    sum
}

// function to find the largest item in a vector
fn largest(items: &Vec<i32>) -> i32 {
    let mut largest = items[0];
    for item in items {
        if item > &largest {
            largest = *item;
        }
    }
    largest
}

// function to find n largest items in a vector
fn nlargest(items: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut nlargest = Vec::new();
    let mut items = items.clone();
    for _ in 0..n {
        let largest = largest(&items);
        nlargest.push(largest);
        items.retain(|&x| x != largest);
    }
    nlargest
}

// function to parse file into vector of integers grouped by \n\n
fn parse_file(filename: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut groups = Vec::new();
    for group in contents.split("\n\n") {
        let mut items = Vec::new();
        for item in group.split_whitespace() {
            items.push(item.parse::<i32>().unwrap());
        }
        groups.push(items);
    }
    groups
}

fn main() {
    let items = parse_file("../list.txt");

    let mut elves: Vec<i32> = Vec::new();

    for lines in items {
        elves.push(sum(&lines));
    }

    let top_elves: Vec<i32> = nlargest(&elves, 3);

    println!("Top 3 elves: {:?}", top_elves);
    println!("{:?}", largest(&elves));
    println!("{:?}", sum(&top_elves));
    
}
