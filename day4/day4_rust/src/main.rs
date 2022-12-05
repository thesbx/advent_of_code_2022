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

/// Generate a vector with a start and end size, where the end is included in the final vector.
/**
---
Examples:
```
generate_vec(30, 127) // -> [30, 31, 32...127]
```
*/
fn generate_vec(start: i32 , end: i32) -> Vec<i32> {
    let vec = (start..end + 1).collect();
    return vec;
}

/// Compare elements of vectors.
/**
---
Examples:
```
find([1, 2, 3, 4], [2, 3], true) // -> true | 2, 3 are both in the haystack
find([1, 2, 3, 4], [2, 9], true) // -> false | 2 is, but 9 isn not.
find([1, 2, 3, 4], [2, 9], false) // -> true | One of the elements is in the haystack.
```
*/
fn find<T: PartialEq>(haystack: &[T], needle: &[T], strict: bool) -> bool {
    if strict {
        return needle.iter().all(|x| haystack.contains(&x));
    } else {
        return needle.iter().any(|x| haystack.contains(&x));
    }
    
}

/// Uses the needle in a haystack algorithm to compare items in a vector. Returns the number of true results.
fn compare_ranges(arr: &Vec<Vec<Vec<i32>>>, strict: bool) -> usize {
    let mut ranges = Vec::new();
    for items in arr {
        let mut contains = Vec::new();
        let left: Vec<i32> = generate_vec(items[0][0], items[0][1]);
        let right: Vec<i32> = generate_vec(items[1][0],items[1][1]);
        
        if find(&right, &left, strict) || find(&left, &right, strict) {
            contains.push(true);
        }

        if contains.len() >= 1 {
            ranges.push(contains)
        }

        
    }
    return ranges.len();
}

fn main() {
    let items = parse_file("../list.txt");
    let part1 = compare_ranges(&items, true);
    let part2 = compare_ranges(&items, false);
    println!("{:?}", part1);
    println!("{:?}", part2);
}
