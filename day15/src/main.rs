#![allow(dead_code)]
use std::{env, fs};

fn hash(s: &str) -> u64 {
    let bytes = s.trim().bytes();
    let mut h = 0;
    for b in bytes {
        h += b as u64;
        h *= 17;
        h %= 256;
    }
    h
}

fn part_one(content: &str) -> u64 {
    content
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| hash(s))
        .sum()
}

fn part_two<'a>(content: &'a str) -> u64 {
    let mut boxes: Vec<Vec<(&str, u64)>> = vec![vec![]; 256];

    content
        .split(",")
        .filter(|s| !s.is_empty())
        .for_each(|s: &'a str| {
            let label = s.split(['-', '=']).next().unwrap();
            let h = hash(label);
            let cur_box = &mut boxes[h as usize];
            if s.contains("-") {
                if let Some(index) = cur_box.iter().position(|lens| lens.0 == label) {
                    cur_box.remove(index);
                }
            } else {
                let (label, focal_length) = s.split_once("=").unwrap();
                let focal_length: u64 = focal_length.trim().parse().unwrap();

                if let Some(index) = cur_box.iter().position(|lens| lens.0 == label) {
                    cur_box[index].1 = focal_length;
                } else {
                    cur_box.push((label, focal_length));
                }
            }
            ()
        });

    boxes
        .iter()
        .enumerate()
        .map(|(box_index, v)| {
            v.iter()
                .enumerate()
                .map(|(lens_index, lens)| (box_index as u64 + 1) * (lens_index as u64 + 1) * lens.1)
                .sum::<u64>()
        })
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
