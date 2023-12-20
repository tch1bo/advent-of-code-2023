#![allow(dead_code)]
use std::{collections::HashMap, env, fs};

fn parse_space_separated_nums(line: &str) -> Vec<u64> {
    line.split(" ")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| -> u64 { chunk.parse().unwrap() })
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Condition {
    attr: char,
    num: u64,
    operator: char,
}

#[derive(Debug, Eq, PartialEq)]
struct Cmd {
    cond: Option<Condition>,
    jump_to: String,
}

fn parse_cmd(cmd_str: &str) -> Cmd {
    let split = cmd_str.split_once(":");
    if split.is_none() {
        Cmd {
            cond: None,
            jump_to: cmd_str.into(),
        }
    } else {
        let (a, name) = split.unwrap();
        Cmd {
            cond: Some(Condition {
                attr: a.chars().nth(0).unwrap(),
                operator: a.chars().nth(1).unwrap(),
                num: a[2..].parse().unwrap(),
            }),
            jump_to: name.into(),
        }
    }
}

type Piece = HashMap<char, u64>;
fn parse_piece(piece_str: &str) -> Piece {
    let mut chars = piece_str.chars();
    chars.next();
    chars.next_back();
    let mut piece = Piece::new();
    chars.as_str().split(",").for_each(|s| {
        piece.insert(s.chars().nth(0).unwrap(), s[2..].parse().unwrap());
    });
    piece
}

fn process(cmds: &Vec<Cmd>, piece: &Piece) -> String {
    for cmd in cmds {
        if cmd.cond.is_none() {
            return cmd.jump_to.clone();
        }
        let cond = cmd.cond.as_ref().unwrap();
        let v = piece[&cond.attr];
        if cond.operator == '<' && v < cond.num {
            return cmd.jump_to.clone();
        }

        if cond.operator == '>' && v > cond.num {
            return cmd.jump_to.clone();
        }
    }
    panic!();
}

fn parse_workflows(s: &str) -> HashMap<&str, Vec<Cmd>> {
    let mut workflows: HashMap<&str, Vec<Cmd>> = HashMap::new();
    s.split("\n").filter(|l| !l.is_empty()).for_each(|l| {
        let (name, r) = l.split_once("{").unwrap();
        let (cmd_str, _) = r.split_once("}").unwrap();
        let cmds: Vec<Cmd> = cmd_str.split(",").map(parse_cmd).collect();
        workflows.insert(name, cmds);
    });
    workflows
}

fn part_one(content: &str) -> u64 {
    let (workflows_str, pieces_str) = content.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows_str);

    let pieces: Vec<Piece> = pieces_str
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_piece)
        .collect();

    pieces
        .iter()
        .map(|piece| -> u64 {
            let mut cur_name = String::from("in");
            loop {
                let cmds = &workflows[cur_name.as_str()];
                cur_name = process(cmds, piece);
                if cur_name == "R" {
                    return 0;
                }
                if cur_name == "A" {
                    return piece.iter().map(|(_, v)| *v).sum();
                }
            }
        })
        .sum()
}

fn negate_condition(cond: &Condition) -> Option<Condition> {
    if cond.operator == '<' && cond.num == 0 {
        return None;
    }
    let (operator, num) = if cond.operator == '<' {
        ('>', cond.num - 1)
    } else {
        ('<', cond.num + 1)
    };
    Some(Condition {
        num,
        operator,
        attr: cond.attr,
    })
}

fn part_two<'a>(content: &'a str) -> u64 {
    let workflows = parse_workflows(content.split_once("\n\n").unwrap().0);

    #[derive(Debug)]
    struct QItem<'a> {
        workflow_id: &'a str,
        cmd_id: usize,
        condition_id: usize,
    }
    let mut q: Vec<QItem> = vec![QItem {
        workflow_id: "in",
        cmd_id: 0,
        condition_id: 0,
    }];

    let mut conditions: Vec<Option<Condition>> = vec![None];
    let mut parents: Vec<usize> = vec![0];
    let mut accept_ids: Vec<usize> = vec![];
    while !q.is_empty() {
        let cur = q.pop().unwrap();

        if cur.workflow_id == "A" {
            accept_ids.push(cur.condition_id);
        }

        if cur.workflow_id == "R" || cur.workflow_id == "A" {
            continue;
        }

        let cmd = &workflows[cur.workflow_id][cur.cmd_id];

        conditions.push(cmd.cond.clone());
        parents.push(cur.condition_id);

        // If the condition holds.
        q.push(QItem {
            workflow_id: cmd.jump_to.as_str(),
            cmd_id: 0,
            condition_id: conditions.len() - 1,
        });

        if cmd.cond.is_some() {
            // If the condition doesn't hold.
            assert!(cur.cmd_id + 1 < workflows[cur.workflow_id].len());

            conditions.push(negate_condition(cmd.cond.as_ref().unwrap()));
            parents.push(cur.condition_id);

            q.push(QItem {
                workflow_id: cur.workflow_id,
                cmd_id: cur.cmd_id + 1,
                condition_id: conditions.len() - 1,
            });
        }
    }

    assert_eq!(parents.len(), conditions.len());

    let ratings = vec!['x', 'm', 'a', 's'];
    accept_ids
        .iter()
        .map(|id| {
            let mut cur_id = *id;
            let mut ranges: HashMap<char, Vec<bool>> = HashMap::new();
            for r in &ratings {
                ranges.insert(*r, vec![true; 4001]);
            }

            while cur_id != 0 {
                if conditions[cur_id].is_some() {
                    let cond = conditions[cur_id].as_ref().unwrap();
                    let v = ranges.get_mut(&cond.attr).unwrap();
                    if cond.operator == '<' {
                        v[(cond.num as usize)..].iter_mut().for_each(|x| *x = false);
                    } else {
                        v[1..=(cond.num as usize)]
                            .iter_mut()
                            .for_each(|x| *x = false);
                    }
                }
                cur_id = parents[cur_id];
            }

            let num_matches: Vec<(char, u64)> = ranges
                .iter()
                .map(|(k, v)| (*k, v[1..].iter().map(|b| *b as u64).sum()))
                .collect();
            let product = num_matches.iter().map(|(_, x)| x).product::<u64>();
            product
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
