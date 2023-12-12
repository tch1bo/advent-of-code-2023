#![allow(dead_code)]
use more_asserts::assert_le;
use std::{env, fs};

fn parse_comma_separated_nums(line: &str) -> Vec<usize> {
    line.split(",")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| -> usize { chunk.parse().unwrap() })
        .collect()
}

fn calc_for_line(line: &str, num_repeats: usize) -> u64 {
    let (springs_str, nums_str) = line.split_once(" ").unwrap();
    let springs_str = vec![springs_str; num_repeats].join("?");
    let nums_str = vec![nums_str; num_repeats].join(",");
    let nums = parse_comma_separated_nums(&nums_str);

    let bytes = springs_str.as_bytes();
    let can_be_dot =
        |pos: usize| -> bool { pos < bytes.len() && (bytes[pos] == b'.' || bytes[pos] == b'?') };
    let can_be_spring =
        |pos: usize| -> bool { pos < bytes.len() && (bytes[pos] == b'#' || bytes[pos] == b'?') };

    // The +1 is a neat trick to avoid checking the indices vs `bytes.len()`.
    let mut dp_table: Vec<Vec<u64>> = vec![vec![0; bytes.len() + 1]; nums.len()];
    for i in (0..nums.len()).rev() {
        for j in (0..bytes.len()).rev() {
            if bytes[j] == b'.' || bytes[j] == b'?' {
                dp_table[i][j] = dp_table[i][j + 1];
            }
            let this_is_the_last_number = i + 1 == nums.len();
            if bytes[j] == b'#' || bytes[j] == b'?' {
                let k = j + nums[i];
                if (j..k).all(can_be_spring) {
                    assert_le!(k, bytes.len()); // otherwise `all(can_be_spring)` wouldn't match
                    dp_table[i][j] += if k == bytes.len() {
                        // Stopped at exactly the end of string.
                        // If this was the last number to be satisfied, then we're good.
                        // Otherwise there are more numbers to satisfy, but no chars left => UNSAT.
                        this_is_the_last_number as u64
                    } else if can_be_dot(k) {
                        // Stopped at a dot.
                        if this_is_the_last_number {
                            // The rest should be dots or question marks.
                            (k + 1..bytes.len()).all(can_be_dot) as u64
                        } else {
                            // There are more numbers to go through.
                            dp_table[i + 1][k + 1]
                        }
                    } else {
                        0
                    }
                }
            }
        }
    }
    dp_table[0][0]
}

fn part_one(content: &str) -> u64 {
    content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| calc_for_line(l, 1))
        .sum()
}

fn part_two(content: &str) -> u64 {
    content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| calc_for_line(l, 5))
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
