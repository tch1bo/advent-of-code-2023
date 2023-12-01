use std::env;
use std::fs;

fn part_one(content: &str) -> u32 {
    content
        .split("\n")
        .map(|line: &str| -> u32 {
            if line.is_empty() {
                return 0;
            };
            let f = line
                .chars()
                .find(char::is_ascii_digit)
                .expect("the line should have a digit");
            let l = line
                .chars()
                .rfind(char::is_ascii_digit)
                .expect("the line should have a digit");
            let r = 10 * f.to_digit(10).expect("") + l.to_digit(10).expect("");
            println!("{line} -> {r}");
            r
        })
        .sum()
}

fn part_two(content: &str) -> u32 {
    let needles = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    content
        .split("\n")
        .map(|line: &str| -> u32 {
            if line.is_empty() {
                return 0;
            };
            let (_, first_needle_index) = needles
                .iter()
                .enumerate()
                .filter_map(|(needle_index, needle)| match line.find(needle) {
                    Some(index_in_string) => Some((index_in_string, needle_index)),
                    None => None,
                })
                .min()
                .expect("at least one needle should match");

            let (_, last_needle_index) = needles
                .iter()
                .enumerate()
                .filter_map(|(needle_index, needle)| match line.rfind(needle) {
                    Some(index_in_string) => Some((index_in_string, needle_index)),
                    None => None,
                })
                .max()
                .expect("at least one needle should match");

            let first_needle = needles[first_needle_index];
            let last_needle = needles[last_needle_index];
            let first_value: u32 = (first_needle_index as u32) % 9 + 1;
            let last_value: u32 = (last_needle_index as u32) % 9 + 1;
            println!("{line} -> {first_needle} ({first_value}) {last_needle} ({last_value})");

            first_value * 10 + last_value
        })
        .sum()
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("please provide one argument: the file path");

    let content = fs::read_to_string(file_path).expect("file should be readable");
    // let r = part_one(&content);
    let r = part_two(&content);

    println!("final answer: {r}");
}
