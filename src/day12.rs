use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day12<'a> {
    nodes: Vec<&'a str>,
    connected: HashMap<&'a str, Vec<&'a str>>,
}

impl Day12<'_> {
    pub fn load<'a>(filename: &str) -> Day12<'a> {
        let mut nodes: Vec<&'a str> = Vec::new();
        let mut connected: HashMap<&'a str, Vec<&'a str>> = HashMap::new();

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
                    // If either of these is a new name, add it to nodes
                    let name: &'a str = &caps[1].to_string();
                    if !nodes.contains(&name) {
                        println!("Node: {}", &name);
                        connected.insert(name, Vec::new());
                        nodes.push(name);
                        // TODO: Add name and empty vector to connected
                    }

                    let name: &'a str = &caps[2].to_string();
                    if !nodes.contains(&name) {
                        println!("Node: {}", &name);
                        connected.insert(name, Vec::new());
                        nodes.push(name);
                        // TODO: Add name and empty vector to connected
                    }
                }
                None => {}
            }
        }

        Day12 { nodes, connected }
    }

    // TODO: Add methods of DayN
}

impl Day for Day12<'_> {
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
