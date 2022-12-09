use anyhow::Result;
use std::fs;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day9.test")?;
    let lines = input
        .split("\n")
        .collect::<Vec<_>>();
    
    let mut head = Pos::default();
    let mut tails = [Pos::default(); 9];

    for instruction in lines {
        let (direction, amount) = instruction.split_once(' ').unwrap();
        for _ in 0..amount.parse().unwrap() {
            match direction {
                "U" => head.y += 1,
                "D" => head.y -= 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => panic!("your fucking matching failed bitch!"),
            }

            follow(&head, &mut tails[0]);

            
        }

    }

    return Ok(());
}

fn follow(lead: &Pos, t: &mut Pos) {
    let dx = lead.x - t.x;
    let dy = lead.x - t.y;

    if (dx.abs() + dy.abs() > 1) && (dx.abs() > 1 || dy.abs() > 1) {
        t.x += dx.signum();
        t.y += dy.signum();
    }
}
