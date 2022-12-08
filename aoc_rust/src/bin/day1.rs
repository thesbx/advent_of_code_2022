use std::fs;

/// Returns the sum of all items in a vector.
fn sum(items: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for item in items {
        sum += item;
    }
    sum
}

/// Returns the largest number in a vector.
fn largest(items: &Vec<i32>) -> i32 {
    let mut largest: i32 = items[0];
    for item in items {
        if item > &largest {
            largest = *item;
        }
    }
    largest
}

/// Return N number of largest numbers in a vector.
fn nlargest(items: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut nlargest: Vec<i32> = Vec::new();
    let mut items: Vec<i32> = items.clone();
    for _ in 0..n {
        let largest = largest(&items);
        nlargest.push(largest);
        items.retain(|&x| x != largest);
    }
    nlargest
}

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
        elves.push(sum(&lines));
    }

    let top_elves: Vec<i32> = nlargest(&elves, 3);

    println!("Top 3 elves: {:?}", top_elves);
    println!("{:?}", largest(&elves));
    println!("{:?}", sum(&top_elves));
    
}
