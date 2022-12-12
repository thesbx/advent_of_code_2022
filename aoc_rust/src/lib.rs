/// Returns the sum of all elements in a vector
pub fn sum(arr: &[i32]) -> i32 {
    return arr.iter().fold(0, |acc, n| acc + n);
}

/// Find the largest element in a vector
pub fn largest(arr: &[i32]) -> i32 {
    let largest = arr.iter().reduce(|acc, i| {
        if acc >= i {acc} else {i} 
    });
    return *largest.unwrap();

}

/// Find n largest elements in a vector
pub fn nlargest(arr: &Vec<i32>, n: usize) -> Vec<i32> {
   let mut nlargest = Vec::new();
   let mut items = arr.clone();
   
   for _ in 0..n {
       let largest = largest(&items);
       nlargest.push(largest);
       items.retain(|&x| x != largest);
   }

   nlargest
}

/// Generates a numeric vector bases on the start and end size, incrementing the elements
/// from start to end.
/**
---
Examples:
```
generate_vec(30, 127) // -> [30, 31, 32...127]
```
*/
pub fn generate_vec(start: i32, end: i32) -> Vec<i32> {
    (start..end + 1).collect()
}

/// Returns true if needle fully exists in haystack if strict is true, otherwise it will
/// return true if any of the elements in needle exist in the haystack
pub fn find_sub<T: PartialEq>
(haystack: &[T], needle: &[T], strict: bool) -> bool 
{
    if strict {
        needle.iter().all(|x| haystack.contains(&x))
    } else {
        needle.iter().any(|x| haystack.contains(&x))
    }
}
