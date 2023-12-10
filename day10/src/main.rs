#![allow(dead_code)]
use std::{collections::HashMap, collections::HashSet, env, fs};

fn parse_space_separated_nums(line: &str) -> Vec<i64> {
    line.split(" ")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| -> i64 { chunk.parse().unwrap() })
        .collect()
}

type Tile = (i64, i64);

struct P {
    neighbors: HashMap<Tile, Vec<Tile>>,
    maze: Vec<Vec<char>>,
    height: i64,
    width: i64,
}

fn parse(content: &str) -> P {
    let maze: Vec<Vec<char>> = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().into_iter().collect())
        .collect();

    let height = maze.len() as i64;
    let width = maze[0].len() as i64;

    let mut spos: Option<Tile> = None;
    for i in 0..height {
        for j in 0..width {
            if maze[i as usize][j as usize] == 'S' {
                spos = Some((i, j));
                break;
            }
        }
        if spos.is_some() {
            break;
        }
    }
    let spos = spos.unwrap();
    let mut q: Vec<Tile> = vec![spos.clone()];
    let mut neighbors: HashMap<Tile, Vec<Tile>> = HashMap::new();
    let m = |i: i64, j: i64| -> char { maze[i as usize][j as usize] };

    while !q.is_empty() {
        let cur = q.pop().unwrap();
        let cur_symbol = maze[cur.0 as usize][cur.1 as usize];

        let ns = neighbors.entry(cur).or_insert(vec![]);
        let mut push = |i, j| {
            let n = (i, j);
            if !ns.contains(&n) {
                // println!("{:?} -> {:?}", cur, n);
                q.push(n);
                ns.push(n);
            }
        };
        if cur_symbol == 'S' {
            if cur.0 >= 1 && "|7F".contains(m(cur.0 - 1, cur.1)) {
                push(cur.0 - 1, cur.1);
            }

            if cur.0 + 1 < height && "|LJ".contains(m(cur.0 + 1, cur.1)) {
                push(cur.0 + 1, cur.1);
            }

            if cur.1 >= 1 && "-FL".contains(m(cur.0, cur.1 - 1)) {
                push(cur.0, cur.1 - 1);
            }

            if cur.1 + 1 < height && "-7J".contains(m(cur.0, cur.1 + 1)) {
                push(cur.0, cur.1 + 1);
            }
        } else {
            let next_ns: Vec<Tile> = match cur_symbol {
                '|' => vec![(cur.0 - 1, cur.1), (cur.0 + 1, cur.1)],
                '-' => vec![(cur.0, cur.1 - 1), (cur.0, cur.1 + 1)],
                'L' => vec![(cur.0 - 1, cur.1), (cur.0, cur.1 + 1)],
                'J' => vec![(cur.0 - 1, cur.1), (cur.0, cur.1 - 1)],
                '7' => vec![(cur.0 + 1, cur.1), (cur.0, cur.1 - 1)],
                'F' => vec![(cur.0 + 1, cur.1), (cur.0, cur.1 + 1)],

                _ => panic!("unexpected symbol"),
            };
            for n in next_ns {
                push(n.0, n.1);
            }
        }
    }
    P {
        neighbors,
        maze,
        height,
        width,
    }
}
fn part_one(content: &str) -> i64 {
    let p = parse(content);
    assert_eq!(p.neighbors.len() % 2, 0);

    (p.neighbors.len() as i64) / 2
}

fn part_two(content: &str) -> i64 {
    let p = parse(content);

    let is_part_of_loop = |i: i64, j: i64| -> bool { p.neighbors.contains_key(&(i, j)) };

    let mut can_escape: Vec<Vec<bool>> = vec![vec![false; p.width as usize]; p.height as usize];
    let tile_is_ok = |tile: &Tile| -> bool {
        tile.0 >= 0 && tile.0 < p.height && tile.1 >= 0 && tile.1 < p.width
    };

    let tile_neighbors_of_tile = |i: i64, j: i64| -> Vec<Tile> {
        let n: Vec<Tile> = vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
        n.into_iter().filter(tile_is_ok).collect()
    };

    // CBP == "coordinates between pipes"
    type CBP = (i64, i64, i64, i64);
    let mut reachable_cbp: HashSet<CBP> = HashSet::new();

    let cbp_is_squeezable = |c: &CBP| -> bool {
        return !p
            .neighbors
            .get(&(c.0, c.1))
            .is_some_and(|x| x.contains(&(c.2, c.3)))
            && !p
                .neighbors
                .get(&(c.2, c.3))
                .is_some_and(|x| x.contains(&(c.0, c.1)));
    };

    let cbp_is_ok =
        |cbp: &CBP| -> bool { tile_is_ok(&(cbp.0, cbp.1)) && tile_is_ok(&(cbp.2, cbp.3)) };

    let cbp_neighbors_of_tile = |i: i64, j: i64| -> Vec<CBP> {
        let n: Vec<CBP> = vec![
            (i - 1, j - 1, i - 1, j),
            (i - 1, j - 1, i, j - 1),
            (i - 1, j, i - 1, j + 1),
            (i - 1, j + 1, i, j + 1),
            (i, j - 1, i + 1, j - 1),
            (i + 1, j - 1, i + 1, j),
            (i, j + 1, i + 1, j + 1),
            (i + 1, j, i + 1, j + 1),
        ];

        n.into_iter()
            .filter(cbp_is_ok)
            .filter(cbp_is_squeezable)
            .collect()
    };

    let cbp_neighbors_of_cbp = |c: CBP| -> Vec<CBP> {
        let n = if c.0 == c.2 {
            vec![
                // Squeeze north.
                (c.0 - 1, c.1, c.2 - 1, c.3),
                // Squeeze north-west.
                (c.0 - 1, c.1, c.0, c.1),
                // Squeeze north-east.
                (c.2 - 1, c.3, c.2, c.3),
                // Squeeze south.
                (c.0 + 1, c.1, c.2 + 1, c.3),
                // Squeeze south-west.
                (c.0, c.1, c.0 + 1, c.1),
                // Squeeze south-east.
                (c.2, c.3, c.2 + 1, c.3),
            ]
        } else {
            assert_eq!(c.1, c.3);
            vec![
                // Squeeze east.
                (c.0, c.1 - 1, c.2, c.3 - 1),
                // Squeeze north-east.
                (c.0, c.1 - 1, c.0, c.1),
                // Squeeze south-east.
                (c.2, c.3 - 1, c.2, c.3),
                // Squeeze west.
                (c.0, c.1 + 1, c.2, c.3 + 1),
                // Squeeze north-west.
                (c.0, c.1, c.0, c.1 + 1),
                // Squeeze south-west.
                (c.2, c.3, c.2, c.3 + 1),
            ]
        };

        n.into_iter()
            .filter(cbp_is_ok)
            .filter(cbp_is_squeezable)
            .collect()
    };

    let tile_neighbors_of_cbp = |c: CBP| -> Vec<Tile> {
        let n = if c.0 == c.2 {
            vec![
                (c.0 - 1, c.1),
                (c.0 - 1, c.3),
                (c.0 + 1, c.1),
                (c.0 + 1, c.3),
            ]
        } else {
            assert_eq!(c.1, c.3);
            vec![
                (c.0, c.1 - 1),
                (c.2, c.3 - 1),
                (c.0, c.1 + 1),
                (c.2, c.3 + 1),
            ]
        };

        n.into_iter()
            .filter(tile_is_ok)
            .filter(|c| !is_part_of_loop(c.0, c.1))
            .collect()
    };

    let mut traverse = |si: i64, sj: i64| {
        if is_part_of_loop(si, sj) {
            return;
        }
        if can_escape[si as usize][sj as usize] {
            // This tile was already processed.
            return;
        }
        let mut q: Vec<Tile> = vec![(si, sj)];
        let mut cbp_q: Vec<CBP> = vec![];

        while !q.is_empty() || !cbp_q.is_empty() {
            while !q.is_empty() {
                let (i, j) = q.pop().unwrap();
                if can_escape[i as usize][j as usize] {
                    continue;
                }
                can_escape[i as usize][j as usize] = true;

                // Propagate through direct neighbors.
                tile_neighbors_of_tile(i, j).iter().for_each(|c| {
                    if !is_part_of_loop(c.0, c.1) {
                        q.push((c.0, c.1));
                    }
                });

                // Try squeezing in between pipes.
                cbp_neighbors_of_tile(i, j).into_iter().for_each(|c| {
                    cbp_q.push(c);
                });
            }

            while !cbp_q.is_empty() {
                let cur_cbp = cbp_q.pop().unwrap();
                if !reachable_cbp.insert(cur_cbp) {
                    continue;
                }

                // Check what other pipes can be squeezed through.
                cbp_neighbors_of_cbp(cur_cbp)
                    .into_iter()
                    .for_each(|c| cbp_q.push(c));

                // Check what tiles can be reached from current `cur_cbp`.
                tile_neighbors_of_cbp(cur_cbp)
                    .into_iter()
                    .for_each(|c| q.push(c));
            }
        }
    };
    for i in 0..p.height {
        traverse(i, 0);
        traverse(i, p.width - 1);
    }
    for j in 0..p.width {
        traverse(0, j);
        traverse(p.height - 1, j);
    }
    // for i in 0..p.height {
    //     for j in 0..p.width {
    //         if is_part_of_loop(i, j) {
    //             print!("{}", p.maze[i][j]);
    //         } else if !can_escape[i][j] {
    //             print!("I");
    //         } else {
    //             print!("O");
    //         }
    //     }
    //     println!("");
    // }

    let mut count_not_escapeable = 0;
    for i in 0..p.height {
        for j in 0..p.width {
            if !can_escape[i as usize][j as usize] && !is_part_of_loop(i, j) {
                count_not_escapeable += 1;
            }
        }
    }
    count_not_escapeable
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
