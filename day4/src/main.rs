#![allow(dead_code)]
use std::{env, fs};

fn get_num_winners(line: &str) -> u32 {
    let (winning_nums_str, our_nums_str) = line
        .split_once(":")
        .expect("line should contain :")
        .1
        .split_once("|")
        .expect("line should contain |");

    let mut winning_nums: Vec<u32> = winning_nums_str
        .split(" ")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| -> u32 { chunk.parse().unwrap() })
        .collect();
    winning_nums.sort();

    our_nums_str
        .split(" ")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| -> u32 { chunk.parse().unwrap() })
        .map(|num| -> u32 { winning_nums.binary_search(&num).is_ok() as u32 })
        .sum()
}

fn part_one(content: &str) -> u32 {
    content
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line: &str| -> u32 {
            let num_winners = get_num_winners(line);

            if num_winners == 0 {
                0
            } else {
                (2 as u32).pow(num_winners - 1)
            }
        })
        .sum()
}

fn part_two(content: &str) -> u32 {
    let num_winners: Vec<u32> = content
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(get_num_winners)
        .collect();

    let mut num_cards: Vec<u32> = vec![1; num_winners.len()];
    for i in 0..num_cards.len() {
        for j in 0..(num_winners[i] as usize) {
            num_cards[i + j + 1] += num_cards[i];
        }
    }

    num_cards.iter().sum()
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
