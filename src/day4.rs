use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::day::Day;
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

struct BingoCard {
    // maps numbers on the card to their coordinates.
    values: HashMap<usize, (usize, usize)>
}

impl BingoCard {
    fn from_vecs(vecs: Vec<Vec<usize>>) -> BingoCard {
        let mut values= HashMap::new();

        for (row_no, row) in vecs.iter().enumerate() {
            for (col_no, value) in row.iter().enumerate() {
                values.insert(*value, (col_no, row_no));
            }
        }

        BingoCard { values }
    }

    // check to see if this match array contains all true in one row or column.
    fn is_win(matches: [[bool; 5]; 5]) -> bool {
        // check rows (all Y equal)
        for y in 0..5 {
            let mut all_match = true;
            for x in 0..5 {
                if !matches[x][y] {
                    all_match = false;
                    break;
                }
            }
            if all_match {
                return true;
            }
        }

        // check columns (all X equal)
        for x in 0..5 {
            let mut all_match = true;
            for y in 0..5 {
                if !matches[x][y] {
                    all_match = false;
                    break;
                }
            }
            if all_match {
                return true;
            }
        }

        false
    }

    fn card_score(&self, matches: [[bool; 5]; 5]) -> usize {
        let mut score = 0;
        for x in 0..5 {
            for y in 0..5 {
                if !matches[x][y] {
                    for (k, v) in self.values.iter() {
                        if v == &(x, y) {
                            score += k;
                        }
                    }
                }
            }
        }

        score
    }

    // run a sequence of calls with this card, it returns None if no win, Some((round, score))
    // if it eventually wins.
    pub fn play(&self, calls: &Vec<usize>) -> Option<(usize, usize)> {
        let mut matched: [[bool; 5]; 5] = [[false; 5]; 5];
        let mut round: usize = 0;
        for call in calls {
            round += 1;
            if self.values.contains_key(&call) {
                let (x, y) = self.values.get(&call).unwrap();
                matched[*x][*y] = true;

                if BingoCard::is_win(matched) {
                    let score = self.card_score(matched) * call;
                    return Some((round, score));
                }
            }
        }

        // There was no win on this card.
        None
    }
}

pub struct Day4 {
    calls: Vec<usize>,
    cards: Vec<BingoCard>,
}

impl Day4 {
    pub fn load(filename: &str) -> Day4 {
        // println!("Loading.");
        lazy_static! {
            static ref ROW_RE: Regex =
                Regex::new("([0-9]+) +([0-9]+) +([0-9]+) +([0-9]+) +([0-9]+)").unwrap();
        }
        let mut on_calls = true;
        let mut card_line = 0;
        let mut calls = Vec::new();
        let mut card: Vec<Vec<usize>> = Vec::new();
        let mut cards: Vec<BingoCard> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = line.unwrap();

            if on_calls {
                for call_str in l.split(",") {
                    calls.push(call_str.parse::<usize>().unwrap());
                }
                on_calls = false;
                card_line = 0;
            }
            else {
                // store values
                // Do RE Magic.
                let caps = ROW_RE.captures(&l);
                match caps {
                    Some(caps) => {
                        let mut row: Vec<usize> = Vec::new();
                        row.push(caps[1].parse::<usize>().unwrap());
                        row.push(caps[2].parse::<usize>().unwrap());
                        row.push(caps[3].parse::<usize>().unwrap());
                        row.push(caps[4].parse::<usize>().unwrap());
                        row.push(caps[5].parse::<usize>().unwrap());
                        card.push(row);
                        card_line += 1;
                        // println!("card_line = {}", card_line);
                        if card_line == 5 {
                            // panic!();
                            cards.push(BingoCard::from_vecs(card));
                            card_line = 0;
                            card = Vec::new();
                        }
                    }
                    None => {
                        // println!("No match.");
                        // Blank line
                    }
                }
            }
        }

        Day4 { calls, cards }
    }
}

impl Day for Day4 {
    fn part1(&self) -> Result<usize, &str> {
        let mut win_round = 1000;
        let mut win_score = 0;

        for card in &self.cards {
            match card.play(&self.calls) {
                Some((round, score)) => {
                    if round < win_round {
                        win_round = round;
                        win_score = score;
                    }
                }
                _ => { /* No win, ignore this card */ }
            }

        }

        Ok(win_score as usize)
    }

    fn part2(&self) -> Result<usize, &str> {
        let mut win_round = 0;
        let mut win_score = 0;

        for card in &self.cards {
            match card.play(&self.calls) {
                Some((round, score)) => {
                    if round > win_round {
                        win_round = round;
                        win_score = score;
                    }
                }
                _ => { /* No win, ignore this card */ }
            }

        }

        Ok(win_score as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day4::load("data/day4_example1.txt");
        assert_eq!(d.calls.len(), 27);
        assert_eq!(d.cards.len(), 3);
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part1() {
        let d = Day4::load("data/day4_example1.txt");
        assert_eq!(d.part1(), Ok(4512));

        let d = Day4::load("data/day4_input.txt");
        assert_eq!(d.part1(), Ok(39902));
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part2() {
        let d = Day4::load("data/day4_example1.txt");
        assert_eq!(d.part2(), Ok(1924));

        let d = Day4::load("data/day4_input.txt");
        assert_eq!(d.part2(), Ok(26936));
    }
}
