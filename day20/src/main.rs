#![allow(dead_code)]
use num::integer::lcm;
use std::{collections::HashMap, collections::VecDeque, env, fs};

fn parse_space_separated_nums(line: &str) -> Vec<u64> {
    line.split(" ")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| -> u64 { chunk.parse().unwrap() })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
enum ModuleT {
    Broadcaster,
    Conjunction,
    FlipFlop,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Eq, PartialEq)]
enum FFState {
    On,
    Off,
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    t: ModuleT,
    out: Vec<&'a str>,
}

#[derive(Debug)]
struct State<'a> {
    ff_states: HashMap<&'a str, FFState>,
    conj_states: HashMap<(&'a str, &'a str), Pulse>,
}

impl Module<'_> {
    fn process<'a>(
        &'a self,
        sender: &'a str,
        pulse: Pulse,
        state: &mut State<'a>,
    ) -> Vec<(&'a str, Pulse)> {
        let out_pulse = match self.t {
            ModuleT::Broadcaster => Some(pulse),
            ModuleT::FlipFlop => {
                let ff_state = state.ff_states.get_mut(self.name).unwrap();

                if pulse == Pulse::High {
                    None
                } else if *ff_state == FFState::On {
                    *ff_state = FFState::Off;
                    Some(Pulse::Low)
                } else {
                    *ff_state = FFState::On;
                    Some(Pulse::High)
                }
            }
            ModuleT::Conjunction => {
                *state.conj_states.get_mut(&(self.name, sender)).unwrap() = pulse;
                let all_high = state
                    .conj_states
                    .iter()
                    .filter_map(|(k, v)| if k.0 == self.name { Some(*v) } else { None })
                    .all(|p| p == Pulse::High);

                if all_high {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        };
        if out_pulse.is_none() {
            return vec![];
        }
        self.out.iter().map(|o| (*o, out_pulse.unwrap())).collect()
    }
}

fn parse_module<'a>(line: &'a str) -> Module<'a> {
    let (name_type, outs) = line.split_once(" -> ").unwrap();
    let (name, t) = if name_type == "broadcaster" {
        (name_type, ModuleT::Broadcaster)
    } else {
        (
            name_type[1..].as_ref(),
            match name_type.chars().nth(0).unwrap() {
                '%' => ModuleT::FlipFlop,
                '&' => ModuleT::Conjunction,
                _ => panic!(),
            },
        )
    };
    Module {
        name,
        t,
        out: outs.split(", ").collect(),
    }
}

fn parse_modules<'a>(content: &'a str) -> HashMap<&'a str, Module<'a>> {
    let mut modules = HashMap::new();
    content.split("\n").filter(|l| !l.is_empty()).for_each(|l| {
        let m = parse_module(l);
        modules.insert(m.name, m);
    });
    modules
}

fn create_state<'a>(modules: &HashMap<&'a str, Module<'a>>) -> State<'a> {
    let mut state = State {
        ff_states: HashMap::new(),
        conj_states: HashMap::new(),
    };
    for (name, m) in modules {
        if m.t == ModuleT::FlipFlop {
            state.ff_states.insert(name, FFState::Off);
        }

        if m.t == ModuleT::Conjunction {
            for (other_name, other_m) in modules {
                if other_m.out.contains(name) {
                    state.conj_states.insert((name, other_name), Pulse::Low);
                }
            }
        }
    }
    state
}

fn part_one(content: &str) -> u64 {
    let modules = parse_modules(content);
    let mut state = create_state(&modules);

    let mut num_high = 0;
    let mut num_low = 0;

    for _ in 0..1000 {
        let mut q = VecDeque::from([("button", "broadcaster", Pulse::Low)]);
        while !q.is_empty() {
            let (sender, receiver, pulse) = q.pop_front().unwrap();
            // println!("{} -{:?}-> {}", sender, pulse, receiver);

            if pulse == Pulse::Low {
                num_low += 1;
            } else {
                num_high += 1;
            }
            let m = modules.get(receiver);
            if m.is_some() {
                m.unwrap()
                    .process(sender, pulse, &mut state)
                    .iter()
                    .for_each(|p| {
                        q.push_back((receiver.clone(), p.0.clone(), p.1));
                    });
            }
        }
    }
    num_high * num_low
}

fn print_dot<'a>(modules: &HashMap<&'a str, Module<'a>>) {
    let mut s = String::from("digraph {\n  rankdir=LR;\n");
    for (_, m) in modules {
        s += format!("{} [label=\"{}\\n{:?}\"];\n", m.name, m.name, m.t).as_str();
        for o in &m.out {
            s += format!("{} -> {};\n", m.name, o).as_str();
        }
    }
    s += "}";
    fs::write("/tmp/1.dot", s).unwrap();
}

fn get_cycle_length<'a>(
    modules: &HashMap<&'a str, Module<'a>>,
    required_sender: &'a str,
    required_pulse: Pulse,
) -> u64 {
    let mut state = create_state(&modules);

    // Check that it exists.
    modules.get(required_sender);

    let mut i = 0;
    loop {
        let mut q = VecDeque::from([("button", "broadcaster", Pulse::Low)]);
        while !q.is_empty() {
            let (sender, receiver, pulse) = q.pop_front().unwrap();
            if sender == required_sender && pulse == required_pulse {
                return i + 1;
            }

            let m = modules.get(receiver);
            if m.is_some() {
                m.unwrap()
                    .process(sender, pulse, &mut state)
                    .iter()
                    .for_each(|p| {
                        q.push_back((receiver.clone(), p.0.clone(), p.1));
                    });
            }
        }
        i += 1;
    }
}

fn part_two(content: &str) -> u64 {
    let modules = parse_modules(content);
    // I couldn't come up with a general solution that would be fast enough for the real input :(
    // The solution below is hardcoded for the specific input.
    print_dot(&modules);
    let cycle_lengths: Vec<u64> = ["sx", "kb", "jt", "ks"]
        .iter()
        .map(|s| get_cycle_length(&modules, s, Pulse::High))
        .collect();
    cycle_lengths.into_iter().reduce(lcm).unwrap()
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
