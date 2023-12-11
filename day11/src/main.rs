#![allow(dead_code)]
use std::{env, fs};

fn run_with_multiplier(content: &str, empty_space_multiplier: i64) -> u64 {
    let grid: Vec<Vec<char>> = content
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let row_empty: Vec<bool> = grid.iter().map(|row| !row.contains(&'#')).collect();
    let col_empty: Vec<bool> = (0..grid.len())
        .into_iter()
        .map(|col_index| (0..grid[0].len()).all(|row_index| grid[row_index][col_index] != '#'))
        .collect();

    let mut galaxies: Vec<(i64, i64)> = vec![];
    let mut num_empty_rows = 0;
    for i in 0..grid.len() {
        num_empty_rows += row_empty[i] as i64;
        let mut num_empty_cols = 0;
        for j in 0..grid[0].len() {
            num_empty_cols += col_empty[j] as i64;
            if grid[i][j] == '#' {
                galaxies.push((
                    i as i64 + num_empty_rows * (empty_space_multiplier - 1),
                    j as i64 + num_empty_cols * (empty_space_multiplier - 1),
                ));
            }
        }
    }

    let mut sum_distances = 0;
    for (i, a) in galaxies.iter().enumerate() {
        for j in i + 1..galaxies.len() {
            let b = galaxies[j];
            let dist = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
            sum_distances += dist;
        }
    }

    sum_distances
}

fn part_one(content: &str) -> u64 {
    run_with_multiplier(content, 2)
}

fn part_two(content: &str) -> u64 {
    run_with_multiplier(content, 1000000)
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
