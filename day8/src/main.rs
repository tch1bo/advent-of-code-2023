#![allow(dead_code)]
use num::integer::lcm;
use std::{collections::HashMap, env, fs};

fn parse_steps_and_graph(content: &str) -> (String, HashMap<String, (String, String)>) {
    let lines: Vec<&str> = content.split("\n").filter(|l| !l.is_empty()).collect();
    let steps = lines[0].to_string();
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    lines[1..].iter().for_each(|line| {
        let (cur_node, children) = line.split_once(" = (").unwrap();
        let (left_child, right_child) = children.split_once(", ").unwrap();
        let right_child = right_child.replace(")", "");
        graph.insert(cur_node.to_string(), (left_child.to_string(), right_child));
    });
    (steps, graph)
}

fn part_one(content: &str) -> u64 {
    let (steps, graph) = parse_steps_and_graph(content);
    let mut num_steps: u64 = 0;
    let mut cur_node = "AAA";
    loop {
        let cur_action = steps.as_bytes()[(num_steps as usize) % steps.len()];
        let children = graph.get(cur_node).unwrap();
        if cur_action == b'L' {
            cur_node = &children.0;
        } else {
            cur_node = &children.1;
        }
        num_steps += 1;
        if cur_node == "ZZZ" {
            break;
        }
    }
    num_steps
}

fn part_two(content: &str) -> u64 {
    let (steps, graph) = parse_steps_and_graph(content);
    let cur_nodes: Vec<&str> = graph
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| k.as_str())
        .collect();
    let num_steps: Vec<u64> = cur_nodes
        .iter()
        .map(|x| {
            let mut num_steps: u64 = 0;
            let mut cur_node = *x;
            loop {
                let cur_action = steps.as_bytes()[(num_steps as usize) % steps.len()];
                let children = graph.get(cur_node).unwrap();
                if cur_action == b'L' {
                    cur_node = &children.0;
                } else {
                    cur_node = &children.1;
                }
                num_steps += 1;
                if cur_node.ends_with("Z") {
                    break;
                }
            }
            num_steps
        })
        .collect();
    num_steps.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
