#![allow(dead_code)]
use std::{env, fs};

#[derive(Debug)]
struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Hailstone {
    fn xy_intersection(&self, other: &Hailstone) -> Option<(f64, f64)> {
        if self.vx == other.vx && self.vy == other.vy {
            if self.x == other.x && self.y == other.y {
                return Some((0.0, 0.0));
            }
            return None;
        }
        let t2 = (self.vx * (other.y - self.y) - self.vy * (other.x - self.x)) as f64
            / (self.vy * other.vx - self.vx * other.vy) as f64;
        let t1 = ((other.x - self.x) as f64 + t2 * (other.vx as f64)) / (self.vx as f64);
        if t1 < 0.0 || t2 < 0.0 {
            return None;
        }

        Some((t1, t2))
    }

    fn calc_x(&self, t: f64) -> f64 {
        self.x as f64 + t * (self.vx as f64)
    }

    fn calc_y(&self, t: f64) -> f64 {
        self.y as f64 + t * (self.vy as f64)
    }
}

fn parse_hailstone(line: &str) -> Hailstone {
    let (a, b) = line.trim().split_once(" @ ").unwrap();
    let avec: Vec<i64> = a.split(", ").map(|c| c.parse::<i64>().unwrap()).collect();
    let bvec: Vec<i64> = b
        .split(", ")
        .map(|c| c.trim().parse::<i64>().unwrap())
        .collect();
    Hailstone {
        x: avec[0],
        y: avec[1],
        z: avec[2],
        vx: bvec[0],
        vy: bvec[1],
        vz: bvec[2],
    }
}

fn parse_hailstones(content: &str) -> Vec<Hailstone> {
    content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| parse_hailstone(l))
        .collect()
}

fn part_one(content: &str) -> u64 {
    let hailstones = parse_hailstones(content);

    const SMALLEST: i64 = 200000000000000;
    const LARGEST: i64 = 400000000000000;

    let check = |v: f64| -> bool { v >= SMALLEST as f64 && v <= LARGEST as f64 };

    let mut count = 0;
    for i in 0..(hailstones.len() - 1) {
        let h = &hailstones[i];
        for j in (i + 1)..hailstones.len() {
            let intersection = h.xy_intersection(&hailstones[j]);
            if let Some((t1, t2)) = intersection {
                if check(h.calc_x(t1)) && check(h.calc_y(t1)) {
                    assert!(check(hailstones[j].calc_x(t2)));
                    assert!(check(hailstones[j].calc_y(t2)));
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_two(content: &str) -> u64 {
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
