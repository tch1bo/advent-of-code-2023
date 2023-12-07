#![allow(dead_code)]
use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
    env, fs,
};

type Hand = [u64; 5];
type HandAndBid = (Hand, u64);

fn parse_hand(hand_str: &str, use_jokers: bool) -> Hand {
    let chars_str = if use_jokers {
        "J23456789TQKA"
    } else {
        "23456789TJQKA"
    };
    hand_str
        .chars()
        .map(|c| chars_str.chars().position(|x| x == c).unwrap() as u64)
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap()
}

fn hand_strength(hand: &Hand, use_jokers: bool) -> u64 {
    let mut card_type_counts: HashMap<u64, u64> = HashMap::new();
    let mut num_jokers = 0;
    hand.iter().for_each(|x| {
        if use_jokers && *x == 0 {
            // If this is a joker, then don't count it for `card_type_counts`.
            num_jokers += 1;
            return;
        }
        *card_type_counts.entry(*x).or_insert(0) += 1;
    });

    if num_jokers == 5 {
        // Corner case - treat 5 jokers as five of a kind.
        return 6;
    }
    let mut card_type_counts: Vec<(u64, u64)> =
        Vec::from_iter(card_type_counts.iter().map(|(a, b)| (*b, *a)));
    card_type_counts.sort_by_key(|x| Reverse(*x));

    let highest_num_of_same_kind = card_type_counts[0].0 + num_jokers;
    match highest_num_of_same_kind {
        // five of a kind
        5 => 6,
        // four of a kind
        4 => 5,
        3 => {
            match card_type_counts[1].0 {
                // full house
                2 => 4,
                // three of a kind
                _ => 3,
            }
        }
        2 => {
            match card_type_counts[1].0 {
                // two pairs
                2 => 2,
                // one pair
                _ => 1,
            }
        }
        // highest card
        _ => 0,
    }
}

fn solve(content: &str, use_jokers: bool) -> u64 {
    let mut hands_and_bids: Vec<HandAndBid> = content
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (hand_str, bid_str) = line.split_once(" ").unwrap();
            let hand = parse_hand(hand_str, use_jokers);
            let bid: u64 = bid_str.parse().unwrap();

            (hand, bid)
        })
        .collect();

    hands_and_bids.sort_by(|a: &HandAndBid, b: &HandAndBid| -> Ordering {
        let strength_a = hand_strength(&a.0, use_jokers);
        let strength_b = hand_strength(&b.0, use_jokers);
        if strength_a != strength_b {
            return strength_a.cmp(&strength_b);
        }

        return a.cmp(b);
    });

    hands_and_bids
        .iter()
        .enumerate()
        .map(|(index, hb)| hb.1 * (index as u64 + 1))
        .sum()
}

fn part_one(content: &str) -> u64 {
    solve(content, false)
}

fn part_two(content: &str) -> u64 {
    solve(content, true)
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_path).unwrap();

    let answer1 = part_one(&content);
    println!("the answer for the first part is: {answer1}");

    let answer2 = part_two(&content);
    println!("the answer for the second part is: {answer2}");
}
