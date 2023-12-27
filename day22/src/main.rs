#![allow(dead_code)]
use std::{collections::HashSet, env, fs};

#[derive(Debug, Clone, Copy)]
struct P {
    x: usize,
    y: usize,
    z: usize,
}

impl P {
    fn parse(s: &str) -> P {
        let v: Vec<usize> = s
            .split(",")
            .filter(|chunk| !chunk.is_empty())
            .map(|chunk| -> usize { chunk.parse().unwrap() })
            .collect();
        P {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

#[derive(Debug)]
struct Brick {
    a: P,
    b: P,
}

fn parse_brick(l: &str) -> Brick {
    let (a_str, b_str) = l.trim().split_once("~").unwrap();
    let a = P::parse(a_str);
    let b = P::parse(b_str);
    if a.z <= b.z {
        Brick { a, b }
    } else {
        Brick { a: b, b: a }
    }
}

fn ranges_intersect(a: &(usize, usize), b: &(usize, usize)) -> bool {
    ((a.0 <= b.0) && (b.0 <= a.1))
        || ((a.0 <= b.1) && (b.1 <= a.1))
        || ((b.0 <= a.0) && (a.0 <= b.1))
        || ((b.0 <= a.1) && (a.1 <= b.1))
}

impl Brick {
    fn xrange(&self) -> (usize, usize) {
        (self.a.x.min(self.b.x), self.a.x.max(self.b.x))
    }

    fn yrange(&self) -> (usize, usize) {
        (self.a.y.min(self.b.y), self.a.y.max(self.b.y))
    }

    fn xy_intersects(&self, other: &Brick) -> bool {
        ranges_intersect(&self.xrange(), &other.xrange())
            && ranges_intersect(&self.yrange(), &other.yrange())
    }

    fn setz(&mut self, z: usize) {
        self.b.z = self.b.z - self.a.z + z;
        self.a.z = z;
    }

    fn is_strictly_above(&self, other: &Brick) -> bool {
        return self.a.z > other.b.z;
    }
}

fn parse_and_drop_bricks(content: &str) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_brick)
        .collect();

    bricks.sort_by_key(|brick| brick.a.z);

    // Make the bricks fall.
    for i in 0..bricks.len() {
        let mut intersections: Vec<usize> = vec![];
        for j in (0..i).rev() {
            if bricks[i].xy_intersects(&bricks[j]) && bricks[i].is_strictly_above(&bricks[j]) {
                intersections.push(j);
            }
        }
        if intersections.is_empty() {
            bricks[i].setz(1);
        } else {
            let maxz = intersections.iter().map(|j| bricks[*j].b.z).max().unwrap();
            bricks[i].setz(maxz + 1);
        }
    }
    bricks.sort_by_key(|brick| brick.a.z);
    bricks
}

fn compute_supports(bricks: &Vec<Brick>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut supports: Vec<Vec<usize>> = vec![vec![]; bricks.len()];
    let mut supported_by: Vec<Vec<usize>> = vec![vec![]; bricks.len()];
    for i in 0..bricks.len() {
        for j in 0..bricks.len() {
            if bricks[i].xy_intersects(&bricks[j]) && bricks[i].b.z + 1 == bricks[j].a.z {
                supports[i].push(j);
                supported_by[j].push(i);
            }
        }
    }
    (supports, supported_by)
}

fn part_one(content: &str) -> u64 {
    let bricks = parse_and_drop_bricks(content);
    let (supports, supported_by) = compute_supports(&bricks);

    (0..bricks.len())
        .map(|i| {
            for supported in &supports[i] {
                let mut found_other_support = false;
                // Check that it's supported by some other brick at the same level as i.
                for other in &supported_by[*supported] {
                    if *other != i {
                        found_other_support = true;
                        break;
                    }
                }

                if !found_other_support {
                    return 0;
                }
            }
            return 1;
        })
        .sum()
}

fn part_two(content: &str) -> u64 {
    let bricks = parse_and_drop_bricks(content);
    let (supports, supported_by) = compute_supports(&bricks);

    // drops[i] = all other bricks that would fall if i would be removed.
    let mut drops: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];

    // seed.
    for i in 0..bricks.len() {
        for supported in &supports[i] {
            let mut found_other_support = false;
            // Check that it's supported by some other brick at the same level as i.
            for other in &supported_by[*supported] {
                if *other != i {
                    found_other_support = true;
                    break;
                }
            }

            if !found_other_support {
                drops[i].insert(*supported);
            }
        }
    }
    // iterate until fixpoint.
    let mut stack: Vec<usize> = vec![];
    for i in 0..bricks.len() {
        if !drops[i].is_empty() {
            stack.push(i);
        }
    }

    while !stack.is_empty() {
        let i = stack.pop().unwrap();
        let mut could_drop = vec![];
        for d in &drops[i] {
            for j in &supports[*d] {
                could_drop.push(*j);
            }
        }

        let size_before = drops[i].len();
        for d in &could_drop {
            if supported_by[*d].iter().all(|s| drops[i].contains(s)) {
                drops[i].insert(*d);
            }
        }
        if drops[i].len() > size_before {
            for j in &supported_by[i] {
                stack.push(*j);
            }
            stack.push(i);
        }
    }

    drops.iter().map(|d| d.len() as u64).sum()
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
