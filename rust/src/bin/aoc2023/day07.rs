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
        self.hands.sort();
        let winnings: u32 = self.hands.iter().enumerate()
            .map(|(idx, hand)| {
                debug!("[{}] Cards {:?}, bid {} => {} result", idx, hand.cards, hand.bid, hand.result);
                (idx as u32 + 1) * hand.bid
            })
            .sum();
        info!("[1] Total winnings: {}", winnings);

        Some((winnings.to_string(), "".to_string()))
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
        let result = hand_result(&cards);
        // calculate result
        Hand {
            cards,
            result,
            bid: u32::from_str(val).unwrap(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.result.cmp(&other.result) {
            Ordering::Equal => (0..5)
                .find_map(|i| compare_card(self.cards[i], other.cards[i]))
                .unwrap(),
            x => x,
        }
    }
}

const CARD_RANK: &str = "23456789TJQKA";

fn compare_card(left: char, right: char) -> Option<Ordering> {
    if left == right {
        return None;
    }
    let lv = CARD_RANK.find(left).unwrap();
    let rv = CARD_RANK.find(right).unwrap();
    Some(lv.cmp(&rv))
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

const HIGH_CARD: u8 = 0;
const ONE_PAIR: u8 = 1;
const TWO_PAIR: u8 = 2;
const THREE_OF_A_KIND: u8 = 3;
const FULL_HOUSE: u8 = 4;
const FOUR_OF_A_KIND: u8 = 5;
const FIVE_OF_A_KIND: u8 = 6;

fn hand_result(chars: &Vec<char>) -> u8 {
    let mut groups = HashMap::new();
    chars.iter().for_each(|ch| {
        groups.entry(ch).and_modify(|val| *val += 1).or_insert(1);
    });
    let mut freqs: Vec<u8> = groups.iter().map(|(_, count)| count.clone()).collect();
    freqs.sort();
    freqs.reverse();
    let uniques = freqs.len();
    if uniques == 1 {
        // all the same
        return FIVE_OF_A_KIND;
    } else if uniques == 2 && freqs[0] == 4 {
        // all the same
        return FOUR_OF_A_KIND;
    } else if uniques == 2 && freqs[0] == 3 {
        // all the same
        return FULL_HOUSE;
    } else if uniques == 3 && freqs[0] == 3 {
        // all the same
        return THREE_OF_A_KIND;
    } else if uniques == 3 && freqs[0] == 2 {
        // all the same
        return TWO_PAIR;
    } else if uniques == 4 {
        // all the same
        return ONE_PAIR;
    } else if uniques == 5 {
        // all the same
        return HIGH_CARD;
    }

    panic!("Invalid hand? {:?} => freqs {:?}", chars, freqs);
}
