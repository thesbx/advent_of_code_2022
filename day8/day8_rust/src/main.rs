use anyhow::Result;
use std::fs;

fn parse_file(filename: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(filename).expect("No file");
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    for line in contents.split("\n") {
        let mut row: Vec<u32> = Vec::new();
        for i in line.chars() {
            row.push(i.to_digit(10).unwrap()); 
        }
        matrix.push(row);
    }
    matrix
}

fn travers_matrix(grid: &Vec<Vec<u32>>) -> (u64, u64) {
    let mut count_visible = 0_u64;
    for r in 1..grid[0].len() - 1 {
        for c in 1..grid.len() - 1 { 
            let mut visible = true;

            for w in (0..c).rev() {
                if grid[r][w] >= grid[r][c] {
                    visible = false;
                    break;
                }
            }

            if visible {
                count_visible += 1;
                continue;
            } else {
                visible = true;
            }

            for e in c + 1..grid[0].len() {
                if grid[r][e] >= grid[r][c] {
                    visible = false;
                    break;
                }
            }

            if visible {
                count_visible += 1;
                continue;
            } else {
                visible = true;
            }

            for n in (0..r).rev() {
                if grid[n][c] >= grid[r][c] {
                    visible = false;
                    break;
                }
            }

            if visible {
                count_visible += 1;
                continue;
            } else {
                visible = true;
            }

            for s in r + 1..grid.len() {
                if grid[s][c] >= grid[r][c] {
                    visible = false;
                    break;
                }
            }
            if visible {
                count_visible += 1;
            }
        }
    }
    let mut vds = vec![];
    for r in 1..grid[0].len() - 1 {
        for c in 1..grid.len() - 1 {
            let mut viewing_distance = [0; 4];

            for w in (0..c).rev() {
                viewing_distance[0] += 1;
                if grid[r][w] >= grid[r][c] {
                    break;
                }
            }

            for e in c + 1..grid[0].len() {
                viewing_distance[1] += 1;
                if grid[r][e] >= grid[r][c] {
                    break;
                }
            }

            for n in (0..r).rev() {
                viewing_distance[2] += 1;
                if grid[n][c] >= grid[r][c] {
                    break;
                }
            }

            for s in r + 1..grid.len() {
                viewing_distance[3] += 1;
                if grid[s][c] >= grid[r][c] {
                    break;
                }
            }

            vds.push(viewing_distance.iter().product());
        }
    }
    (
        count_visible + 2 * (grid[0].len() as u64 + grid.len() as u64 - 2),
        *vds.iter().max().unwrap(),
    )
}
fn main() -> Result<()> {
    let input = parse_file("../prod.txt");
    let (p1, p2) = travers_matrix(&input);
    println!("{:?}", p1); 
    println!("{:?}", p2); 

    return Ok(());
}
