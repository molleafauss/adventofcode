// https://adventofcode.com/2023/day/4
// til: split returns "" when two separators are contiguous
// the card_queue contains the number of same cards won from the previous round. Keep adding or
// updating and pop when you get the next card, no need to re-score.

use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use log::{debug, info};

use adventofcode::Solver;

pub struct Solution {
    total_points: u32,
    card_queue: VecDeque<u32>,
    total_cards: u32
}

impl Solution {
    pub(crate) fn new() -> Solution {
        Solution {
            total_points: 0,
            card_queue: VecDeque::new(),
            total_cards: 0,
        }
    }
}

impl Solver for Solution {
    fn parse(&mut self, line: &str) {
        let mut parts = line.split(" ");
        assert_eq!("Card", parts.next().unwrap());
        let id_text = parts.find(|text| text.ends_with(':')).unwrap();
        let card_id = u32::from_str(id_text.trim_end_matches(':')).unwrap();
        let mut winning = true;
        let mut winning_numbers = HashSet::new();
        let mut card_winners = 0;
        parts
            .filter(|str| !str.is_empty())
            .for_each(|part| {
                if part == "|" {
                    winning = false;
                    return;
                }
                if winning {
                    winning_numbers.insert(u32::from_str(part).unwrap());
                    return;
                }

                let number = u32::from_str(part).unwrap();
                if winning_numbers.contains(&number) {
                    card_winners += 1;
                }
            });

        let card_value = match card_winners {
            0 => 0,
            1 => 1,
            _ => 2 << (card_winners - 2),
        };

        let num_cards_scored = 1 + match self.card_queue.is_empty() {
            true => 0,
            false => self.card_queue.pop_front().unwrap(),
        };
        self.total_cards += num_cards_scored;

        for i in 0..card_winners {
            if self.card_queue.len() > i {
                self.card_queue[i] += num_cards_scored;
            } else {
                self.card_queue.push_back(num_cards_scored);
            }
        }

        debug!("Card {} => value {}, scored cards {}", card_id, card_value, num_cards_scored);
        self.total_points += card_value;
    }

    fn solve(&mut self) -> Option<(String, String)> {
        info!("[1] Total points: {}", self.total_points);
        info!("[2] Total cards: {}", self.total_cards);

        Some((self.total_points.to_string(), self.total_cards.to_string()))
    }
}
