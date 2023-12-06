#![allow(dead_code)]
use std::{env, fs, iter::zip};

fn parse_space_separated_nums(line: &str) -> Vec<u64> {
    line.split(" ")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| -> u64 { chunk.parse().unwrap() })
        .collect()
}

fn num_ways_to_beat(time: u64, distance: u64) -> u64 {
    let discriminant = (time * time - 4 * distance) as f64;
    assert!(discriminant > 0.0);
    let first_root = (((time as f64) - discriminant.sqrt()) / 2.0).floor() as u64 + 1;
    let second_root = ((time as f64) + discriminant.sqrt()) / 2.0;
    let second_root = if second_root.fract() == 0.0 {
        second_root.floor() as u64 - 1
    } else {
        second_root.floor() as u64
    };
    second_root - first_root + 1
}

fn part_one(content: &str) -> u64 {
    let (time_str, distance_str) = content.split_once("\n").unwrap();
    let times = parse_space_separated_nums(time_str.split_once(":").unwrap().1);
    let distances = parse_space_separated_nums(
        distance_str
            .strip_suffix("\n")
            .unwrap()
            .split_once(":")
            .unwrap()
            .1,
    );

    zip(times, distances)
        .map(|(time, distance)| num_ways_to_beat(time, distance))
        .reduce(|a, b| a * b)
        .unwrap()
}

fn part_two(content: &str) -> u64 {
    let (time_str, distance_str) = content.split_once("\n").unwrap();
    let time: u64 = time_str
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: u64 = distance_str
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();

    num_ways_to_beat(time, distance)

}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
