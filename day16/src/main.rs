#![allow(dead_code)]
use std::{collections::HashSet, env, fs};

type C = (i64, i64, i64, i64);
fn count_tiles(grid: &Vec<Vec<u8>>, start: C) -> u64 {
    let w = grid[0].len() as i64;
    let h = grid.len() as i64;

    let mut stack: Vec<C> = vec![start];
    let mut v: HashSet<C> = HashSet::new();
    let mut e: HashSet<(i64, i64)> = HashSet::new();

    while !stack.is_empty() {
        let c = stack.pop().unwrap();
        if !v.insert(c) {
            continue;
        }
        e.insert((c.0, c.1));
        assert!(c.2 != 0 || c.3 != 0);

        let new_c: Vec<C> = match grid[c.0 as usize][c.1 as usize] {
            b'.' => vec![(c.0 + c.2, c.1 + c.3, c.2, c.3)],
            b'|' => {
                if c.2 == 0 {
                    // split vertically
                    vec![(c.0 - 1, c.1, -1, 0), (c.0 + 1, c.1, 1, 0)]
                } else {
                    // don't split
                    vec![(c.0 + c.2, c.1 + c.3, c.2, c.3)]
                }
            }
            b'-' => {
                if c.3 == 0 {
                    // split horizontally
                    vec![(c.0, c.1 - 1, 0, -1), (c.0, c.1 + 1, 0, 1)]
                } else {
                    // don't split
                    vec![(c.0 + c.2, c.1 + c.3, c.2, c.3)]
                }
            }
            b'\\' => {
                if c.2 == 0 {
                    if c.3 == 1 {
                        // bounce down
                        vec![(c.0 + 1, c.1, 1, 0)]
                    } else {
                        // bounce up
                        vec![(c.0 - 1, c.1, -1, 0)]
                    }
                } else if c.2 == 1 {
                    // bounce right
                    vec![(c.0, c.1 + 1, 0, 1)]
                } else {
                    // bounce left
                    assert_eq!(c.2, -1);
                    vec![(c.0, c.1 - 1, 0, -1)]
                }
            }
            b'/' => {
                if c.2 == 0 {
                    if c.3 == 1 {
                        // bounce up
                        vec![(c.0 - 1, c.1, -1, 0)]
                    } else {
                        // bounce down
                        vec![(c.0 + 1, c.1, 1, 0)]
                    }
                } else if c.2 == 1 {
                    // bounce left
                    vec![(c.0, c.1 - 1, 0, -1)]
                } else {
                    // bounce right
                    assert_eq!(c.2, -1);
                    vec![(c.0, c.1 + 1, 0, 1)]
                }
            }
            _ => panic!(""),
        };

        for nc in new_c {
            if nc.0 >= 0 && nc.0 < h && nc.1 >= 0 && nc.1 < w {
                stack.push(nc);
            }
        }
    }

    e.len() as u64
}

fn part_one(content: &str) -> u64 {
    let grid: Vec<Vec<u8>> = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().bytes().collect())
        .collect();
    count_tiles(&grid, (0, 0, 0, 1))
}

fn part_two(content: &str) -> u64 {
    let grid: Vec<Vec<u8>> = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().bytes().collect())
        .collect();
    let w = grid[0].len() as i64;
    let h = grid.len() as i64;
    let mut starts: Vec<C> = vec![];
    for i in 0..h {
        starts.push((i, 0, 0, 1));
        starts.push((i, w - 1, 0, -1));
    }
    for j in 0..w {
        starts.push((0, j, 1, 0));
        starts.push((h - 1, j, -1, 0));
    }

    starts.iter().map(|c| count_tiles(&grid, *c)).max().unwrap()
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
