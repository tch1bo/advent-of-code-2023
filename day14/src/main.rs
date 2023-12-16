#![allow(dead_code)]
use std::{collections::HashMap, env, fs};

type GridT = Vec<Vec<u8>>;

fn part_one(content: &str) -> u64 {
    let grid: GridT = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.bytes().collect())
        .collect();

    let mut sum: u64 = 0;
    for j in 0..grid[0].len() {
        let mut max_next_spot = 0;
        for i in 0..grid.len() {
            if grid[i][j] == b'O' {
                sum += (grid.len() - max_next_spot) as u64;

                max_next_spot += 1;
                continue;
            }

            if grid[i][j] == b'#' {
                max_next_spot = i + 1;
            }
        }
    }
    sum
}

fn print_grid(grid: &GridT) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{}", grid[i][j] as char);
        }
        println!("");
    }
}

fn do_one_cycle(grid: &mut GridT) {
    // north.
    for j in 0..grid[0].len() {
        let mut max_next_spot = 0;
        for i in 0..grid.len() {
            if grid[i][j] == b'O' {
                if max_next_spot != i {
                    grid[max_next_spot][j] = b'O';
                    grid[i][j] = b'.';
                }
                max_next_spot += 1;
            } else if grid[i][j] == b'#' {
                max_next_spot = i + 1;
            }
        }
    }

    // west.
    for i in 0..grid.len() {
        let mut max_next_spot = 0;
        for j in 0..grid[0].len() {
            if grid[i][j] == b'O' {
                if max_next_spot != j {
                    grid[i][max_next_spot] = b'O';
                    grid[i][j] = b'.';
                }
                max_next_spot += 1;
            } else if grid[i][j] == b'#' {
                max_next_spot = j + 1;
            }
        }
    }

    // south.
    for j in 0..grid[0].len() {
        let mut max_next_spot = grid.len() - 1;
        for i in (0..grid.len()).rev() {
            if grid[i][j] == b'O' {
                if max_next_spot != i {
                    grid[max_next_spot][j] = b'O';
                    grid[i][j] = b'.';
                }
                if max_next_spot > 0 {
                    max_next_spot -= 1;
                }
            } else if grid[i][j] == b'#' {
                if i >= 1 {
                    max_next_spot = i - 1;
                }
            }
        }
    }

    // east.
    for i in 0..grid.len() {
        let mut max_next_spot = grid[0].len() - 1;
        for j in (0..grid[0].len()).rev() {
            if grid[i][j] == b'O' {
                if max_next_spot != j {
                    grid[i][max_next_spot] = b'O';
                    grid[i][j] = b'.';
                }
                if max_next_spot > 0 {
                    max_next_spot -= 1;
                }
            } else if grid[i][j] == b'#' {
                if j >= 1 {
                    max_next_spot = j - 1;
                }
            }
        }
    }
}

fn calc_sum(grid: &GridT) -> u64 {
    let mut cur_sum: u64 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'O' {
                cur_sum += (grid.len() - i) as u64;
            }
        }
    }
    cur_sum
}

fn part_two(content: &str) -> u64 {
    let mut grid: GridT = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.bytes().collect())
        .collect();

    let mut grid_to_id: HashMap<GridT, usize> = HashMap::new();
    let mut id_to_grid: HashMap<usize, GridT> = HashMap::new();

    let mut insert = |g: &GridT| -> usize {
        if let Some(id) = grid_to_id.get(g) {
            return *id;
        }
        let new_id = grid_to_id.len();
        let id = *grid_to_id.entry(g.clone()).or_insert(new_id);
        id_to_grid.insert(id, g.clone());

        id
    };

    let mut trans_map: HashMap<usize, usize> = HashMap::new();
    let first_id_in_cycle: usize;
    let last_id_in_cycle: usize;
    loop {
        let from_id = {
            let g = &grid;
            insert(g)
        };
        {
            let g = &mut grid;
            do_one_cycle(g);
        }

        let to_id = {
            let g = &grid;
            insert(g)
        };
        if from_id > to_id {
            first_id_in_cycle = to_id;
            last_id_in_cycle = from_id;
            break;
        }
        trans_map.insert(from_id, to_id);
    }

    let index_in_cycle =
        (1000000000 - first_id_in_cycle) % (last_id_in_cycle - first_id_in_cycle + 1);
    let final_id = first_id_in_cycle + index_in_cycle;

    calc_sum(&id_to_grid[&final_id])
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
