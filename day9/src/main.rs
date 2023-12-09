#![allow(dead_code)]
use std::{env, fs};

fn parse_space_separated_nums(line: &str) -> Vec<i64> {
    line.split(" ")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| -> i64 { chunk.parse().unwrap() })
        .collect()
}

fn part_one(content: &str) -> i64 {
    fn extrapolate_forwards(line: &str) -> i64 {
        let nums = parse_space_separated_nums(line);
        let mut difs: Vec<Vec<i64>> = vec![nums];
        loop {
            let cur_difs: Vec<i64> = difs
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect();
            if cur_difs.iter().all(|x| *x == 0) {
                break;
            }
            difs.push(cur_difs);
        }

        difs.iter().rev().map(|x| x.last().unwrap()).sum()
    }

    let lines: Vec<&str> = content.split("\n").filter(|l| !l.is_empty()).collect();
    lines.iter().map(|line| extrapolate_forwards(line)).sum()
}

fn part_two(content: &str) -> i64 {
    fn extrapolate_backwards(line: &str) -> i64 {
        let nums = parse_space_separated_nums(line);
        let mut difs: Vec<Vec<i64>> = vec![nums];
        loop {
            let cur_difs: Vec<i64> = difs
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[0] - w[1])
                .collect();
            if cur_difs.iter().all(|x| *x == 0) {
                break;
            }
            difs.push(cur_difs);
        }

        difs.iter().rev().map(|x| x.first().unwrap()).sum()
    }
    let lines: Vec<&str> = content.split("\n").filter(|l| !l.is_empty()).collect();
    lines.iter().map(|line| extrapolate_backwards(line)).sum()
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
