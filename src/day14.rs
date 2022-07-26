use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::mem;

pub struct Day14 {
    initial: String,
    rules: HashMap<String, String>,
}

impl Day14 {
    pub fn load(filename: &str) -> Day14 {
        let mut initial = String::from("");
        let mut rules: HashMap<String, String> = HashMap::new();
        lazy_static! {
            static ref INITIAL_RE: Regex =
                Regex::new("^([A-Z]+)$").unwrap();
            static ref RULE_RE: Regex =
                Regex::new("([A-Z]+) -> ([A-Z])").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let caps = INITIAL_RE.captures(&l);
            match caps {
                Some(caps) => {
                    initial = caps[1].to_string();
                }
                None => {}
            }

            let caps = RULE_RE.captures(&l);
            match caps {
                Some(caps) => {
                    rules.insert(caps[1].to_string(), caps[2].to_string());
                }
                None => {}
            }
        }

        Day14 { initial, rules }
    }

    fn gen(&self, n: usize) -> String {
        let mut s1 = self.initial.to_string();
        let mut s2 = String::new();

        let from: &mut String = &mut s1;
        let to: &mut String = &mut s2;

        for _gen in 0..n {
            // add first letter
            *to += &from[0..1];

            // add intervening characters and following one.
            for index in 0..from.len()-1 {
                let key = &from[index..index+2];
                match self.rules.get(key) {
                    Some(c) => {
                        *to += c;
                        *to += &key[1..2];
                    }
                    None => { }
                }
            }

            // swap from and to for next iteration
            mem::swap(from, to);

            // clear "to" for next round
            to.clear();
        }

        from.to_string()
    }

    fn score(&self, n: usize) -> usize {
        let s = self.gen(n);

        // find min and max letter counts
        let mut max: usize = 0;
        let mut min: usize = s.len();

        let mut checked = String::new();
        for c in s.chars() {
            if !(checked.contains(c)) {
                let count = s.matches(c).count();
                if count > max {
                    max = count;
                }
                if count < min {
                    min = count;
                }

                checked.push(c);
            }
        }

        max - min
    }

    fn score2(&self, n: usize) -> usize {
        // convert rules to a hashmap mapping a pair of letters to two new pairs.
        let mut conversions: HashMap<String, (String, String)> = HashMap::new();
        for s in self.rules.keys() {
            let mut s1 = String::new();
            let mut s2: String = String::new();
            s1 += &s[0..1];
            s1 += &self.rules[s];
            s2 += &self.rules[s];
            s2 += &s[1..2];
            conversions.insert(s.to_string(), (s1, s2));
        }

        let mut pairs1: HashMap<String, usize> = HashMap::new();
        let mut pairs2: HashMap<String, usize> = HashMap::new();

        // initialize pair counts from self.initial
        let pairs = &mut pairs1;

        for n in 0..self.initial.len()-1 {
            let pair = &self.initial[n..n+2];
            if pairs.contains_key(pair) {
                *pairs.get_mut(pair).unwrap() += 1;
            }
            else {
                pairs.insert(pair.to_string(), 1);
            }
        }

        // pair counts at end of the generation process.
        let pairs_out = &mut pairs2;

        // Go through n generations of pair count updates.
        for _gen in 0..n {
            for (pair, count) in pairs.iter() {
                match conversions.get(pair) {
                    Some((s1, s2)) => {
                        if pairs_out.contains_key(s1) {
                            *pairs_out.get_mut(s1).unwrap() += *count;
                        }
                        else {
                            pairs_out.insert(s1.to_string(), *count);
                        }
                        if pairs_out.contains_key(s2) {
                            *pairs_out.get_mut(s2).unwrap() += *count;
                        }
                        else {
                            pairs_out.insert(s2.to_string(), *count);
                        }
                    }
                    None => {
                        // No change for this pair
                        if pairs_out.contains_key(pair) {
                            *pairs_out.get_mut(pair).unwrap() += *count;
                        }
                        else {
                            pairs_out.insert(pair.to_string(), *count);
                        }
                    }
                }
            }

            // use output of last generation for input of next
            mem::swap(pairs, pairs_out);
            pairs_out.clear();
        }

        // Convert pair counts into character counts
        let mut char_count: HashMap<String, usize> = HashMap::new();
        for (pair, count) in pairs.iter() {
            let c1 = &pair[0..1];
            if char_count.contains_key(c1) {
                *char_count.get_mut(c1).unwrap() += count;
            }
            else {
                char_count.insert(c1.to_string(), *count);
            }
            let c2 = &pair[1..2];
            if char_count.contains_key(c2) {
                *char_count.get_mut(c2).unwrap() += count;
            }
            else {
                char_count.insert(c2.to_string(), *count);
            }
        }

        // Adjust counts for first and last chars.
        // All the characters have been counted twice now, as the first char of a pair and
        // as the second.  All but the first and last chars, that is.  We want to divide by two,
        // then to get the actual number but first we should add one to the first char's entry
        // and one to the last char's entry.
        let first = &self.initial[0..1];
        let last = &self.initial.chars().last().unwrap().to_string();
        *char_count.get_mut(first).unwrap() += 1;
        *char_count.get_mut(last).unwrap() += 1;

        // now divide by two
        for (_, count) in char_count.iter_mut() {
            *count /= 2;
            // println!("Char '{c}' : {count}");
        }

        // find min and max letter counts
        let max = char_count.values().max().unwrap();
        let min = char_count.values().min().unwrap();

        max - min
    }
}

impl Day for Day14 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(self.score(10))
    }

    fn part2(&self) -> Result<usize, &str> {
        Ok(self.score2(40))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day14::load("data/day14_example1.txt");
        assert_eq!(d.initial.len(), 4);
        assert_eq!(d.rules.len(), 16);
    }

    #[test]
    fn test_gen() {
        let d = Day14::load("data/day14_example1.txt");
        assert_eq!(d.gen(1), "NCNBCHB");
        assert_eq!(d.gen(2), "NBCCNBBBCBHCB");
        assert_eq!(d.gen(10).len(), 3073);
    }

    #[test]
    fn test_score() {
        let d = Day14::load("data/day14_example1.txt");
        let score = d.score(10);
        assert_eq!(score, 1588);

    }

    #[test]
    fn test_score2() {
        let d = Day14::load("data/day14_example1.txt");
        let score = d.score2(10);
        assert_eq!(score, 1588);
        let score = d.score2(40);
        assert_eq!(score, 2188189693529);
    }

    #[test]
    fn test_part1() {
        let d = Day14::load("data/day14_example1.txt");
        assert_eq!(d.part1(), Ok(1588));
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_soln() {
        let d = Day14::load("data/day14_input.txt");
        assert_eq!(d.part1(), Ok(2590));
        assert_eq!(d.part2(), Ok(2875665202438));
    }
}
