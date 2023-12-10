#![allow(dead_code)]
use std::{collections::HashMap, collections::HashSet, env, fs};

fn parse_space_separated_nums(line: &str) -> Vec<i64> {
    line.split(" ")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| -> i64 { chunk.parse().unwrap() })
        .collect()
}

fn part_one(content: &str) -> i64 {
    0
}

fn part_two(content: &str) -> i64 {
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
