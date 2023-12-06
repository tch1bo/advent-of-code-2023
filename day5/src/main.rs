#![allow(dead_code)]
use std::{cmp::max, cmp::min, collections::HashMap, env, fs, ops::Range};

fn parse_space_separated_nums(line: &str) -> Vec<u64> {
    line.split(" ")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| -> u64 { chunk.parse().unwrap() })
        .collect()
}

#[derive(Debug)]
struct RangePair {
    source_range: Range<u64>,
    dest_range: Range<u64>,
}

#[derive(Debug)]
struct NamedRanges {
    dest_name: String,
    ranges: Vec<RangePair>,
}

impl NamedRanges {
    fn map_number(&self, num: u64) -> u64 {
        for range in &self.ranges {
            if range.source_range.contains(&num) {
                return num - range.source_range.start + range.dest_range.start;
            }
        }
        num
    }
}

fn parse_number_map_lines(lines: &Vec<&str>) -> HashMap<String, NamedRanges> {
    let mut i: usize = 1;
    let mut ranges_map = HashMap::<String, NamedRanges>::new();
    while i < lines.len() {
        let chunks: Vec<&str> = lines[i].split(['-', ' ']).collect();
        let (source, dest) = (chunks[0], chunks[2]);

        i += 1;
        let mut ranges = Vec::<RangePair>::new();
        while i < lines.len() && lines[i].as_bytes()[0].is_ascii_digit() {
            let nums = parse_space_separated_nums(lines[i]);

            ranges.push(RangePair {
                source_range: (nums[1]..nums[1] + nums[2]),
                dest_range: (nums[0]..nums[0] + nums[2]),
            });
            i += 1;
        }
        ranges_map.insert(
            source.to_string(),
            NamedRanges {
                dest_name: dest.to_string(),
                ranges,
            },
        );
    }
    ranges_map
}

fn part_one(content: &str) -> u64 {
    let lines: Vec<&str> = content
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect();

    let seeds = parse_space_separated_nums(lines[0].split_once(":").unwrap().1);
    let source_to_number_map = parse_number_map_lines(&lines);
    seeds
        .iter()
        .map(|seed| {
            let mut source = "seed";
            let mut cur_num = *seed;
            loop {
                let number_map = source_to_number_map.get(source).unwrap();
                cur_num = number_map.map_number(cur_num);

                source = &number_map.dest_name;

                if source == "location" {
                    return cur_num;
                }
            }
        })
        .min()
        .unwrap()
}

fn range_intersection(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
    if a.start <= b.end && a.end >= b.start {
        Some(Range {
            start: max(a.start, b.start),
            end: min(a.end, b.end),
        })
    } else {
        None
    }
}

fn range_difference(a: &Range<u64>, b: &Range<u64>) -> Vec<Range<u64>> {
    if a.end < b.start || a.start > b.end {
        // no intersection
        return vec![a.clone()];
    }

    let mut r = vec![];
    if a.start < b.start {
        // left intersection, if present
        r.push(Range {
            start: a.start,
            end: b.start,
        });
    }
    if b.end < a.end {
        // right intersection, if present
        r.push(Range {
            start: b.end + 1,
            end: a.end,
        });
    }

    r
}

impl NamedRanges {
    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        // Subranges of `range` that were mapped to the new ids.
        let mut mapped_intersections: Vec<Range<u64>> = vec![];
        // Ranges with the new ids.
        let mut new_ranges: Vec<Range<u64>> = vec![];
        for rp in &self.ranges {
            let i = range_intersection(range, &rp.source_range);
            if i.is_some() {
                // map the intersection to the destination range
                let i = i.unwrap();
                let new_start = i.start - rp.source_range.start + rp.dest_range.start;
                new_ranges.push(Range {
                    start: new_start,
                    end: new_start + (i.end - i.start),
                });
                mapped_intersections.push(i);
            }
        }

        let mut unmapped_subranges: Vec<Range<u64>> = vec![range.clone()];
        for mi in &mapped_intersections {
            let mut new_unmapped_subranges: Vec<Range<u64>> = vec![];
            for us in &unmapped_subranges {
                new_unmapped_subranges.append(&mut range_difference(us, mi));
            }
            unmapped_subranges = new_unmapped_subranges;
        }
        new_ranges.append(&mut unmapped_subranges);

        new_ranges
    }
}

fn part_two(content: &str) -> u64 {
    let lines: Vec<&str> = content
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect();

    let seed_ranges: Vec<Range<u64>> =
        parse_space_separated_nums(lines[0].split_once(":").unwrap().1)
            .chunks(2)
            .map(|w| w[0]..w[0] + w[1])
            .collect();
    let source_to_number_map = parse_number_map_lines(&lines);

    seed_ranges
        .iter()
        .map(|seed_range| {
            let mut cur_ranges: Vec<Range<u64>> = vec![seed_range.clone(); 1];
            let mut cur_source = "seed";

            loop {
                let number_map = source_to_number_map.get(cur_source).unwrap();
                let mut next_ranges: Vec<Range<u64>> = vec![];

                cur_ranges.iter().for_each(|r| {
                    let mut new_ranges = number_map.map_range(r);
                    next_ranges.append(&mut new_ranges);
                });

                cur_source = &number_map.dest_name;
                cur_ranges = next_ranges;

                if cur_source == "location" {
                    return cur_ranges.iter().map(|r| r.start).min().unwrap();
                }
            }
        })
        .min()
        .unwrap()
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
