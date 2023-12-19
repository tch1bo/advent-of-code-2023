#![allow(dead_code)]
use std::{cmp::max, cmp::min, env, fs};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Dig {
    dir: char,
    num: usize,
}

fn parse_part_one(content: &str) -> Vec<Dig> {
    content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (dir, r) = l.split_once(" ").unwrap();
            let num = r.split_once(" ").unwrap().0.parse().unwrap();

            Dig {
                dir: dir.chars().next().unwrap(),
                num,
            }
        })
        .collect()
}

fn parse_digs(digs: &Vec<Dig>) -> Vec<(usize, usize)> {
    let mut points: Vec<(i64, i64)> = vec![(0, 0)];
    for d in digs {
        let p = points.last().unwrap();
        let mut i = p.0;
        let mut j = p.1;
        match d.dir {
            'R' => j += d.num as i64,
            'L' => j -= d.num as i64,
            'D' => i += d.num as i64,
            'U' => i -= d.num as i64,
            _ => panic!(),
        }
        points.push((i, j));
    }

    // Align to 0.
    let mini = points.iter().map(|p| p.0).min().unwrap();
    let minj = points.iter().map(|p| p.1).min().unwrap();

    for p in &mut points {
        p.0 -= mini;
        p.1 -= minj;
    }

    points
        .iter()
        .map(|p| (p.0 as usize, p.1 as usize))
        .collect()
}

fn solve(digs: &Vec<Dig>) -> Vec<u64> {
    let points = parse_digs(digs);
    let h = points.iter().map(|p| p.0).max().unwrap() as usize + 1;

    let mut walls: Vec<Vec<(usize, usize)>> = vec![vec![]; h];
    for w in points.windows(2) {
        if w[0].1 == w[1].1 {
            // It's a vertical wall.
            for i in min(w[0].0, w[1].0)..max(w[0].0, w[1].0) + 1 {
                walls[i].push((w[0].1, w[0].1));
            }
        } else {
            // It's a horizontal wall.
            assert_eq!(w[0].0, w[1].0);
            walls[w[0].0].push((min(w[0].1, w[1].1), max(w[0].1, w[1].1)));
        }
    }

    walls.iter_mut().for_each(|ws| ws.sort());
    let walls = walls;

    let wall_is_u_shaped = |i: usize, w: &(usize, usize)| {
        if i == 0 || i == h - 1 {
            return true;
        }
        if w.0 == w.1 {
            return false;
        }
        return (walls[i - 1].contains(&(w.0, w.0)) && walls[i - 1].contains(&(w.1, w.1)))
            || (walls[i + 1].contains(&(w.0, w.0)) && walls[i + 1].contains(&(w.1, w.1)));
    };

    (0..h)
        .map(|i| {
            // Merge the adjacent walls.
            if walls[i].is_empty() {
                return 0;
            }
            let mut merged_walls = vec![];
            let mut prev_wall = walls[i][0].clone();
            for k in 1..walls[i].len() {
                if walls[i][k].0 == prev_wall.1 {
                    prev_wall = (prev_wall.0, walls[i][k].1);
                } else {
                    merged_walls.push(prev_wall);
                    prev_wall = walls[i][k];
                }
            }
            merged_walls.push(prev_wall);

            // Replace the U-shaped walls with two single walls.
            let mut normalized_walls = vec![];
            let mut num_perimeter_tiles: u64 = 0;
            for w in &merged_walls {
                if wall_is_u_shaped(i, w) {
                    if normalized_walls.len() % 2 == 1 {
                        num_perimeter_tiles += (w.1 - w.0 - 1) as u64;
                    }
                    // This is a U-shaped wall.
                    normalized_walls.push((w.0, w.0));
                    normalized_walls.push((w.1, w.1));
                } else {
                    normalized_walls.push(w.clone());
                }
            }

            // Count the tiles.
            let num_internal_tiles: u64 = normalized_walls
                .chunks(2)
                .map(|c| (c[1].1 - c[0].0 + 1) as u64)
                .sum();

            return num_internal_tiles + num_perimeter_tiles;
        })
        .collect()
}

fn part_one(content: &str) -> u64 {
    let digs = parse_part_one(content);
    solve(&digs).iter().sum()
}

fn parse_part_two(content: &str) -> Vec<Dig> {
    content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let color = l.split_once(" (#").unwrap().1;
            let num = usize::from_str_radix(&color[..5], 16).unwrap();

            Dig {
                dir: match color.chars().nth_back(1).unwrap() {
                    '0' => 'R',
                    '1' => 'D',
                    '2' => 'L',
                    '3' => 'U',
                    _ => panic!(),
                },
                num,
            }
        })
        .collect()
}

fn part_two(content: &str) -> u64 {
    let digs = parse_part_two(content);
    solve(&digs).iter().sum()
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
