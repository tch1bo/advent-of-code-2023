#![allow(dead_code)]
use std::{env, fs};

fn parse_block(block: &str) -> Vec<Vec<u8>> {
    block
        .split("\n")
        .filter(|r| !r.is_empty())
        .map(|l| l.bytes().collect())
        .collect()
}

fn process_block(block: &str, expected_diff: i64) -> i64 {
    let chars = parse_block(block);
    let num_rows = chars.len();
    let num_cols = chars[0].len();

    let check_row = |r: usize| -> i64 {
        let mut i: i64 = r as i64;
        let mut j = r + 1;
        let mut num_diff = 0;
        while i >= 0 && j < num_rows {
            for k in 0..num_cols {
                if chars[i as usize][k] != chars[j][k] {
                    num_diff += 1;
                }
            }

            i -= 1;
            j += 1;
        }
        if num_diff == expected_diff {
            (r as i64 + 1) * 100
        } else {
            0
        }
    };

    let check_col = |c: usize| -> i64 {
        let mut i: i64 = c as i64;
        let mut j = c + 1;
        let mut num_diff = 0;
        while i >= 0 && j < num_cols {
            for k in 0..num_rows {
                if chars[k][i as usize] != chars[k][j] {
                    num_diff += 1;
                }
            }

            i -= 1;
            j += 1;
        }
        if num_diff == expected_diff {
            c as i64 + 1
        } else {
            0
        }
    };

    (0..num_rows - 1).map(check_row).sum::<i64>() + (0..num_cols - 1).map(check_col).sum::<i64>()
}

fn part_one(content: &str) -> i64 {
    content
        .split("\n\n")
        .filter(|block| !block.is_empty())
        .map(|block| process_block(block, 0))
        .sum()
}

fn part_two(content: &str) -> i64 {
    content
        .split("\n\n")
        .filter(|block| !block.is_empty())
        .map(|block| process_block(block, 1))
        .sum()
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
