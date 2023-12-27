#![allow(dead_code)]
use itertools::Itertools;
use std::{collections::HashMap, collections::HashSet, env, fs};

fn part_one(content: &str) -> u64 {
    let grid: Vec<Vec<char>> = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().chars().collect())
        .collect();

    let w = grid[0].len();
    let h = grid.len();

    let mut reachable: Vec<(usize, usize)> = vec![];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 'S' {
                reachable.push((i, j));
                break;
            }
        }
        if !reachable.is_empty() {
            break;
        }
    }

    for _ in 0..64 {
        let mut new = vec![];
        for r in &reachable {
            let (i, j) = *r;
            if i > 0 && grid[i - 1][j] != '#' {
                new.push((i - 1, j));
            }
            if i + 1 < h && grid[i + 1][j] != '#' {
                new.push((i + 1, j));
            }
            if j > 0 && grid[i][j - 1] != '#' {
                new.push((i, j - 1));
            }
            if j + 1 < w && grid[i][j + 1] != '#' {
                new.push((i, j + 1));
            }
        }
        new.sort();
        reachable = new.into_iter().unique().collect();
    }

    reachable.len() as u64
}

fn part_two(content: &str) -> u64 {
    let grid: Vec<Vec<char>> = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().chars().collect())
        .collect();

    let w = grid[0].len();
    let h = grid.len();

    let mut reachable = vec![];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 'S' {
                reachable.push((i, j));
                break;
            }
        }
        if !reachable.is_empty() {
            break;
        }
    }
    const NUM_STEPS: usize = 500;

    for step in 1..(NUM_STEPS + 1) {
        let mut new = vec![];
        for r in &reachable {
            let (i, j) = *r;
            // Up.
            if i == 0 && grid[h - 1][j] != '#' {
                // new.insert((h - 1, j, mapi - 1, mapj));
            }
            if i > 0 && grid[i - 1][j] != '#' {
                new.push((i - 1, j));
            }

            // Down.
            if i == h - 1 && grid[0][j] != '#' {
                // new.insert((0, j, mapi + 1, mapj));
            }
            if i + 1 < h && grid[i + 1][j] != '#' {
                new.push((i + 1, j));
            }

            // Left.
            if j == 0 && grid[i][w - 1] != '#' {
                // new.insert((i, w - 1, mapi, mapj - 1));
            }
            if j > 0 && grid[i][j - 1] != '#' {
                new.push((i, j - 1));
            }

            // Right.
            if j == w - 1 && grid[i][0] != '#' {
                // new.insert((i, 0, mapi, mapj + 1));
            }
            if j + 1 < w && grid[i][j + 1] != '#' {
                new.push((i, j + 1));
            }
        }
        new.sort();
        reachable = new.into_iter().unique().collect();

        // let grid = grids[grid_id].clone();
        // if step % 2 == NUM_STEPS % 2 {
        //     if grid.last_computed.is_none() {
        //         grids.get_mut(grid_id).unwrap().last_computed =
        //             Some((reachable.len() as u64, step));
        //     } else {
        //         let cur_best = grids[grid_id].last_computed.unwrap().0;
        //         if cur_best == reachable.len() as u64 {
        //             // We reached the fixpoint for this tile.
        //             println!("breaking {grid_id} at step {step}");
        //             reached_fix_point = true;
        //         }

        //         grids.get_mut(grid_id).unwrap().last_computed =
        //             Some((reachable.len() as u64, step));
        //     }
        // }
        // }
    }

    0
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
