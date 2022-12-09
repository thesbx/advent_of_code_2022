use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("src/bin/inputs/day9.test")?;
    let lines = input
        .split("\n")
        .collect::<Vec<_>>();

    println!("{:?}", lines);
    return Ok(());
}
