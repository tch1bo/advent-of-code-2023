#![allow(dead_code)]
use priority_queue::PriorityQueue;
use std::{collections::HashMap, collections::HashSet, env, fs};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
    Start,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct C {
    dir: Direction,
    i: usize,
    j: usize,
    num_steps: usize,
}

fn part_one(content: &str) -> u64 {
    let g: Vec<Vec<u64>> = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();

    let w = g[0].len();
    let h = g.len();
    let mut pq: PriorityQueue<C, i64> = PriorityQueue::new();
    let mut dist_map: HashMap<C, u64> = HashMap::new();
    for dir in [
        Direction::Left,
        Direction::Right,
        Direction::Down,
        Direction::Up,
    ] {
        for num_steps in 1..4 {
            for i in 0..h {
                for j in 0..w {
                    let dist = if i == 0 && j == 0 { 0 } else { 1000000 };
                    let c = C {
                        dir,
                        i,
                        j,
                        num_steps,
                    };
                    pq.push(c, -dist);
                    dist_map.insert(c, dist as u64);
                }
            }
        }
    }

    let update = |p: &mut PriorityQueue<C, i64>,
                  d: &mut HashMap<C, u64>,
                  i: usize,
                  j: usize,
                  neg_dist: i64,
                  dir: Direction,
                  cur_num_steps: usize| {
        let (di, dj): (i64, i64) = match dir {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            _ => panic!(),
        };
        let mut inc_dist = 0;
        let mut newi = i as i64;
        let mut newj = j as i64;
        for cur_step in (cur_num_steps + 1)..4 {
            newi += di;
            newj += dj;
            if newi >= 0 && (newi < h as i64) && newj >= 0 && (newj < w as i64) {
                inc_dist += g[newi as usize][newj as usize];
                let new_dist = (-neg_dist as u64) + inc_dist;
                let newc = C {
                    i: newi as usize,
                    j: newj as usize,
                    dir,
                    num_steps: cur_step,
                };
                if new_dist < d[&newc] {
                    *d.get_mut(&newc).unwrap() = new_dist;
                    p.change_priority(&newc, -(new_dist as i64));
                }
            }
        }
    };

    while !pq.is_empty() {
        let (c, cur_dist) = pq.pop().unwrap();

        if c.dir == Direction::Up {
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Up,
                c.num_steps,
            );

            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Left,
                0,
            );

            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Right,
                0,
            );
        }

        if c.dir == Direction::Down {
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Down,
                c.num_steps,
            );
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Left,
                0,
            );
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Right,
                0,
            );
        }

        if c.dir == Direction::Left {
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Left,
                c.num_steps,
            );
            update(&mut pq, &mut dist_map, c.i, c.j, cur_dist, Direction::Up, 0);
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Down,
                0,
            );
        }

        if c.dir == Direction::Right {
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Right,
                c.num_steps,
            );
            update(&mut pq, &mut dist_map, c.i, c.j, cur_dist, Direction::Up, 0);
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Down,
                0,
            );
        }
    }
    dist_map
        .iter()
        .filter_map(|(k, v)| {
            if k.i != h - 1 || k.j != w - 1 {
                None
            } else {
                Some(*v)
            }
        })
        .min()
        .unwrap()
}

fn part_two(content: &str) -> u64 {
    let g: Vec<Vec<u64>> = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();

    let w = g[0].len();
    let h = g.len();
    let mut pq: PriorityQueue<C, i64> = PriorityQueue::new();
    let mut dist_map: HashMap<C, u64> = HashMap::new();
    for dir in [
        Direction::Left,
        Direction::Right,
        Direction::Down,
        Direction::Up,
    ] {
        for num_steps in 4..11 {
            for i in 0..h {
                for j in 0..w {
                    let dist = if i == 0 && j == 0 { 0 } else { 1000000 };
                    let c = C {
                        dir,
                        i,
                        j,
                        num_steps,
                    };
                    pq.push(c, -dist);
                    dist_map.insert(c, dist as u64);
                }
            }
        }
    }
    let update = |p: &mut PriorityQueue<C, i64>,
                  d: &mut HashMap<C, u64>,
                  i: usize,
                  j: usize,
                  neg_dist: i64,
                  dir: Direction,
                  cur_num_steps: usize| {
        let (di, dj): (i64, i64) = match dir {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            _ => panic!(),
        };
        let mut inc_dist = 0;
        let mut newi = i as i64;
        let mut newj = j as i64;
        for cur_step in (cur_num_steps + 1)..11 {
            newi += di;
            newj += dj;
            if newi >= 0 && (newi < h as i64) && newj >= 0 && (newj < w as i64) {
                inc_dist += g[newi as usize][newj as usize];
                if cur_step >= 4 {
                    let new_dist = (-neg_dist as u64) + inc_dist;
                    let newc = C {
                        i: newi as usize,
                        j: newj as usize,
                        dir,
                        num_steps: cur_step,
                    };
                    if new_dist < d[&newc] {
                        *d.get_mut(&newc).unwrap() = new_dist;
                        p.change_priority(&newc, -(new_dist as i64));
                    }
                }
            }
        }
    };

    while !pq.is_empty() {
        let (c, cur_dist) = pq.pop().unwrap();

        if c.dir == Direction::Up {
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Up,
                c.num_steps,
            );

            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Left,
                0,
            );

            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Right,
                0,
            );
        }

        if c.dir == Direction::Down {
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Down,
                c.num_steps,
            );
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Left,
                0,
            );
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Right,
                0,
            );
        }

        if c.dir == Direction::Left {
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Left,
                c.num_steps,
            );
            update(&mut pq, &mut dist_map, c.i, c.j, cur_dist, Direction::Up, 0);
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Down,
                0,
            );
        }

        if c.dir == Direction::Right {
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Right,
                c.num_steps,
            );
            update(&mut pq, &mut dist_map, c.i, c.j, cur_dist, Direction::Up, 0);
            update(
                &mut pq,
                &mut dist_map,
                c.i,
                c.j,
                cur_dist,
                Direction::Down,
                0,
            );
        }
    }
    dist_map
        .iter()
        .filter_map(|(k, v)| {
            if k.i != h - 1 || k.j != w - 1 {
                None
            } else {
                Some(*v)
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
