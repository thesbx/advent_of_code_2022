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

fn main() {


    let file: Vec<String> = fs::read_to_string("../list.txt")
        .expect("Should find file")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    
    let output: Vec<Vec<i32>> = file.into_iter().fold(Vec::new(), |mut acc, x| {
        
        if x == "" || acc.is_empty() {
            acc.push(Vec::new());
        }
        if x != "" {
            let int = x.parse::<i32>().unwrap();
            acc.last_mut().unwrap().push(int);
        }

        acc
        
    });

    let mut elves: Vec<i32> = Vec::new();

    for lines in output {
        elves.push(sum(&lines));
    }

    let top_elves: Vec<i32> = nlargest(&elves, 3);
    println!("Top 3 elves: {:?}", top_elves);
    println!("{:?}", largest(&elves));
    println!("{:?}", sum(&top_elves));
    
}
