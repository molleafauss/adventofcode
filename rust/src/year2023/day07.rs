// https://adventofcode.com/2023/day/7

use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use log::{debug, info};
use adventofcode::Solver;

pub struct Solution {
    hands: Vec<Hand>
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            hands: Vec::new()
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        self.hands.push(Hand::parse(&line[..5], &line[6..]))
    }

    fn solve(&mut self) -> Option<(String, String)> {
        // sort hands
        self.hands.iter_mut().for_each(|hand| {
            hand.result = score_hand(&hand.cards, false);
        });
        self.hands.sort_by(|a, b| compare_hands(a, b, CARD_RANK_PART1));
        let winnings_part1: u32 = self.hands.iter().enumerate()
            .map(|(idx, hand)| {
                debug!("[{}] Cards {:?}, bid {} => {} result", idx, hand.cards, hand.bid, hand.result);
                (idx as u32 + 1) * hand.bid
            })
            .sum();
        info!("[1] Total winnings: {}", winnings_part1);

        // reset scores
        self.hands.iter_mut().for_each(|hand| {
            hand.result = score_hand(&hand.cards, true);
        });
        self.hands.sort_by(|a, b| compare_hands(a, b, CARD_RANK_PART2));
        let winnings_part2: u32 = self.hands.iter().enumerate()
            .map(|(idx, hand)| {
                debug!("[{}] Cards {:?}, bid {} => {} result", idx, hand.cards, hand.bid, hand.result);
                (idx as u32 + 1) * hand.bid
            })
            .sum();
        info!("[2] Total winnings: {}", winnings_part2);

        Some((winnings_part1.to_string(), winnings_part2.to_string()))
    }
}

struct Hand {
    cards: Vec<char>,
    result: u8,
    bid: u32,
}

impl Hand {
    fn parse(text: &str, val: &str) -> Hand {
        let cards = text.chars().collect();
        // calculate result
        Hand {
            cards,
            result: 0,
            bid: u32::from_str(val).unwrap(),
        }
    }
}

fn compare_hands(this: &Hand, that: &Hand, ties: &str) -> Ordering {
    match this.result.cmp(&that.result) {
        Ordering::Equal => (0..5)
            .find_map(|i| compare_card(this.cards[i], that.cards[i], ties))
            .unwrap(),
        x => x,
    }
}

const CARD_RANK_PART1: &str = "23456789TJQKA";
const CARD_RANK_PART2: &str = "J23456789TQKA";

fn compare_card(left: char, right: char, ties: &str) -> Option<Ordering> {
    if left == right {
        return None;
    }
    let lv = ties.find(left).unwrap();
    let rv = ties.find(right).unwrap();
    Some(lv.cmp(&rv))
}

const HIGH_CARD: u8 = 0;
const ONE_PAIR: u8 = 1;
const TWO_PAIR: u8 = 2;
const THREE_OF_A_KIND: u8 = 3;
const FULL_HOUSE: u8 = 4;
const FOUR_OF_A_KIND: u8 = 5;
const FIVE_OF_A_KIND: u8 = 6;

fn score_hand(chars: &Vec<char>, use_wildcards: bool) -> u8 {
    let mut groups = HashMap::new();
    let mut wildcards = 0;
    if use_wildcards {
        wildcards = chars.iter().filter(|ch| **ch == 'J').count();
        chars.iter()
            .filter(|ch| **ch != 'J')
            .for_each(|ch| {
                groups.entry(ch).and_modify(|val| *val += 1).or_insert(1);
            });
    } else {
        chars.iter().for_each(|ch| {
            groups.entry(ch).and_modify(|val| *val += 1).or_insert(1);
        });
    }

    let mut freqs: Vec<usize> = groups.iter().map(|(_, count)| count.clone()).collect();
    freqs.sort();
    freqs.reverse();
    if use_wildcards {
        // add jokers to the most frequent card
        if !freqs.is_empty() {
            freqs[0] += wildcards;
        } else {
            // all J hand
            freqs.push(wildcards);
        }
    }

    match (freqs.len(), freqs[0]) {
        // all the same
        (1, _) => FIVE_OF_A_KIND,
        // 4+1
        (2, 4) => FOUR_OF_A_KIND,
        // 3 + 2
        (2, 3) => FULL_HOUSE,
        // 3+1+1
        (3, 3) => THREE_OF_A_KIND,
        // 2+2+1
        (3, 2) => TWO_PAIR,
        // 2+1+1+1
        (4, _) => ONE_PAIR,
        // all different
        (5, _) => HIGH_CARD,
        // invalid?
        (_, _) => panic!("Invalid hand? {:?} => freqs {:?}", chars, freqs),
    }
}