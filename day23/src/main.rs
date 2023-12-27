#![allow(dead_code)]
use std::{collections::HashMap, collections::HashSet, env, fs};

fn parse_grid(content: &str) -> Vec<Vec<char>> {
    content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

type P = (usize, usize);

fn get_possible_directions(p: &P, grid: &Vec<Vec<char>>) -> Vec<P> {
    let mut d: Vec<P> = vec![];
    let (i, j) = *p;
    if (grid[i][j] == '.' || grid[i][j] == '^') && i > 0 {
        d.push((i - 1, j));
    }
    if (grid[i][j] == '.' || grid[i][j] == 'v') && i + 1 < grid.len() {
        d.push((i + 1, j));
    }
    if (grid[i][j] == '.' || grid[i][j] == '<') && j > 0 {
        d.push((i, j - 1));
    }
    if (grid[i][j] == '.' || grid[i][j] == '>') && j + 1 < grid[0].len() {
        d.push((i, j + 1));
    }

    d.into_iter()
        .filter(|(newi, newj)| grid[*newi][*newj] != '#')
        .collect()
}

fn backtrack(
    i: usize,
    j: usize,
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<(usize, usize)>,
) -> usize {
    let mut d: Vec<(usize, usize)>;
    let mut cur = (i, j);
    loop {
        visited.push(cur);
        d = get_possible_directions(&cur, &grid)
            .into_iter()
            .filter(|newp| !visited.contains(&newp))
            .collect();

        if d.is_empty() {
            if visited.last().unwrap().0 == grid.len() - 1 {
                return visited.len();
            }
            return 1;
        }
        if d.len() != 1 {
            break;
        }
        cur = d[0];
    }

    d.into_iter()
        .map(|(newi, newj)| {
            let mut v = visited.clone();
            let x = backtrack(newi, newj, grid, &mut v);
            x
        })
        .max()
        .unwrap()
}

fn solve(grid: Vec<Vec<char>>) -> usize {
    let si = 0;
    let sj = grid[si].iter().position(|c| *c == '.').unwrap();

    backtrack(si, sj, &grid, &mut vec![]) - 1
}

fn part_one(content: &str) -> usize {
    let grid = parse_grid(content);

    solve(grid)
}

fn part_two(content: &str) -> usize {
    let mut grid = parse_grid(content);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '#' {
                grid[i][j] = '.';
            }
        }
    }

    solve(grid)
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
