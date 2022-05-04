use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day12 {
    nodes: HashMap<String, usize>,
    connected: HashMap<usize, Vec<usize>>,
}

impl Day12 {
    fn new() -> Day12 {
        Day12 { nodes: HashMap::new(), connected: HashMap::new() }
    }

    fn add_edge(& mut self, a: &String, b: &String) {
        let a_index = match self.nodes.contains_key(a) {
            true => *self.nodes.get(a).unwrap(),
            false => {
                println!("New node for a: {}", a);
                let new_index = self.nodes.len();
                self.nodes.insert(a.to_string(), new_index);
                self.connected.insert(new_index, Vec::new());
                new_index
            }
        };

        let b_index = match self.nodes.contains_key(b) {
            true => *self.nodes.get(b).unwrap(),
            false => {
                println!("New node for b: {}", b);
                let new_index = self.nodes.len();
                self.nodes.insert(b.to_string(), new_index);
                self.connected.insert(new_index, Vec::new());
                new_index
            }
        };

        let a_connected = self.connected.get_mut(&a_index).unwrap();
        a_connected.push(b_index);
        let b_connected = self.connected.get_mut(&b_index).unwrap();
        b_connected.push(a_index);
    }

    pub fn load(filename: &str) -> Day12 {
        let mut day12 = Day12::new();

        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("([a-zA-Z]+)-([a-zA-Z]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let caps = LINE_RE.captures(&l);
            match caps {
                Some(caps) => {
                    day12.add_edge(&caps[1].to_string(), &caps[2].to_string());
                }
                None => {}
            }
        }

        day12
    }

    // TODO: Add methods of DayN
}

impl Day for Day12 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(1)
    }

    fn part2(&self) -> Result<usize, &str> {
        Ok(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day12::load("data/day12_example1.txt");
        assert_eq!(d.nodes.len(), 6);
        assert_eq!(d.connected.len(), 6);
    }

    // TODO: Add tests for methods

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_soln() {
        let d = Day12::load("data/dayN_example1.txt");
        assert_eq!(d.part1(), Ok(1));
        assert_eq!(d.part2(), Ok(2));
    }
}
