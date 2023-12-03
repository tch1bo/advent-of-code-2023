#![allow(dead_code)]
use std::{env, fs};

#[derive(Debug)]
struct Number {
    value: u32,
    start_index: usize,
    // `end_index` is non-inclusive (Python style)
    end_index: usize,
}

#[derive(Debug)]
struct Line {
    numbers: Vec<Number>,
    // stores indices of any symbols (used in part one)
    symbol_indices: Vec<usize>,

    // stores indices of '*' symbols (used in part two)
    star_indices: Vec<usize>,
}

fn symbol_matches_number(symbol_index: usize, number: &Number) -> bool {
    (symbol_index + 1 >= number.start_index) && (symbol_index <= number.end_index)
}

impl Line {
    fn has_matching_symbol(&self, number: &Number) -> bool {
        self.symbol_indices
            .iter()
            .any(|symbol_index| -> bool { symbol_matches_number(*symbol_index, number) })
    }
}

fn parse_line(line_str: &str) -> Line {
    let mut numbers = Vec::<Number>::new();
    let mut symbol_indices = Vec::<usize>::new();
    let mut star_indices = Vec::<usize>::new();

    let bytes = line_str.as_bytes();
    // println!("processing {line_str}");
    let mut i: usize = 0;
    while i < bytes.len() {
        if bytes[i] == b'.' || bytes[i] == b'\r' {
            i += 1;
            continue;
        };
        if bytes[i].is_ascii_digit() {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                j += 1;
            }
            let value: u32 = String::from_utf8(bytes[i..j].to_vec())
                .expect("invalid string")
                .parse()
                .expect("invalid number");
            numbers.push(Number {
                value,
                start_index: i,
                end_index: j,
            });
            i = j;
        } else {
            symbol_indices.push(i);
            if bytes[i] == b'*' {
                star_indices.push(i);
            }
            i += 1;
        }
    }

    Line {
        numbers,
        symbol_indices,
        star_indices,
    }
}

fn part_one(content: &str) -> u32 {
    fn check_line(line: &Line, prev_line: Option<&Line>, next_line: Option<&Line>) -> u32 {
        line.numbers
            .iter()
            .map(|number| -> u32 {
                if line.has_matching_symbol(number)
                    || (prev_line.is_some() && prev_line.unwrap().has_matching_symbol(number))
                    || (next_line.is_some() && next_line.unwrap().has_matching_symbol(number))
                {
                    return number.value;
                }
                0
            })
            .sum()
    }

    let lines: Vec<Line> = content.split("\n").map(parse_line).collect();
    lines
        .iter()
        .enumerate()
        .map(|(index, line)| -> u32 {
            check_line(
                line,
                if index > 0 {
                    lines.get(index - 1)
                } else {
                    None
                },
                // The overflow here is fine, `get` will take care of it.
                lines.get(index + 1),
            )
        })
        .sum()
}

fn part_two(content: &str) -> u32 {
    let lines: Vec<Line> = content.split("\n").map(parse_line).collect();

    fn push_values_matching_star(star_index: usize, line: &Line, matching_values: &mut Vec<u32>) {
        line.numbers.iter().for_each(|number| {
            if symbol_matches_number(star_index, number) {
                matching_values.push(number.value)
            }
        })
    }
    lines
        .iter()
        .enumerate()
        .map(|(line_index, line)| -> u32 {
            line.star_indices
                .iter()
                .map(|star_index| -> u32 {
                    let mut matching_values = Vec::<u32>::new();
                    if line_index > 0 {
                        push_values_matching_star(
                            *star_index,
                            &lines[line_index - 1],
                            &mut matching_values,
                        );
                    }
                    push_values_matching_star(*star_index, &line, &mut matching_values);
                    if line_index + 1 < lines.len() {
                        push_values_matching_star(
                            *star_index,
                            &lines[line_index + 1],
                            &mut matching_values,
                        )
                    }
                    if matching_values.len() == 2 {
                        matching_values[0] * matching_values[1]
                    } else {
                        0
                    }
                })
                .sum()
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
