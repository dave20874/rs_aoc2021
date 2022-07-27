use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::VecDeque;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day12 {
    index: HashMap<String, usize>,
    is_upper: HashMap<usize, bool>,
    connected: HashMap<usize, Vec<usize>>,
}

impl Day12 {
    fn new() -> Day12 {
        Day12 { index: HashMap::new(), is_upper: HashMap::new(), connected: HashMap::new() }
    }

    fn add_edge<'t>(& mut self, a: &str, b: &str) {
        let a_index = match self.index.contains_key(a) {
            true => *self.index.get(a).unwrap(),
            false => {
                // println!("New node for a: {}", a);
                let next = self.index.len();
                self.index.insert(a.to_string(), next);
                self.is_upper.insert(next, a.chars().nth(0).unwrap().is_uppercase());
                self.connected.insert(next, Vec::new());
                next
            }
        };

        let b_index = match self.index.contains_key(b) {
            true => *self.index.get(b).unwrap(),
            false => {
                // println!("New node for b: {}", b);
                let next = self.index.len();
                self.index.insert(b.to_string(), next);
                self.is_upper.insert(next, b.chars().nth(0).unwrap().is_uppercase());
                self.connected.insert(next, Vec::new());
                next
            }
        };

        let a_connected = &mut self.connected.get_mut(&a_index).unwrap();
        a_connected.push(b_index);
        let b_connected = &mut self.connected.get_mut(&b_index).unwrap();
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
                    day12.add_edge(&caps[1], &caps[2]);
                }
                None => {}
            }
        }

        day12
    }

    fn index_of(&self, name: &str) -> Option<&usize> {
        self.index.get(name)
    }

    fn is_upper(&self, node: &usize) -> bool {
        *self.is_upper.get(node).unwrap()
    }

    fn visited(path: &Vec<usize>, node: &usize) -> bool {
        for n in path {
            if n == node {
                return true;
            }
        }

        return false;
    }

    fn num_paths(&self, can_double: bool) -> usize {
        let mut in_progress: VecDeque<(bool, Vec<usize>)> = VecDeque::new();
        let mut final_paths: Vec<Vec<usize>> = Vec::new();

        // Create path that starts at "start"
        let mut start_path = Vec::new();
        start_path.push(*self.index_of("start").unwrap());
        in_progress.push_back((false, start_path));

        while !in_progress.is_empty() {
            // Get the path we want to extend
            let (doubled, p) = in_progress.pop_front().unwrap();
            // println!("Working from {:?}", p);

            // Extend to all valid next nodes
            let last_index = p.get(p.len()-1).unwrap();
            for next in self.connected[last_index].iter() {
                // println!("    Next: {}", next);
                let mut new_doubled = doubled;
                let mut can_visit = true;

                // Can we not visit this node
                if !self.is_upper(next) {
                    if Day12::visited(&p, next) {
                        // we've visited this lower case node before.  If we're not supposed
                        // to double (part 1) or we already doubled (part 2) we can't visit it.
                        // otherwise we can but we need to account for the doubling we just did.
                        if !can_double || doubled || (next == self.index_of("start").unwrap()) {
                            can_visit = false;
                        } else {
                            new_doubled = true;
                        }
                    }
                }

                if can_visit {
                    // We could go to this node
                    let mut new_path: Vec<usize> = Vec::new();
                    for n in p.iter() {
                        new_path.push(*n);
                    }
                    new_path.push(*next);

                    // If the path ends there, add it to final paths, otherwise push it back
                    // on in_progress
                    if next == self.index_of("end").unwrap() {
                        // println!("Final: {:?}", new_path);
                        final_paths.push(new_path);

                    } else {
                        // println!("Intermediate: {:?}", new_path);
                        in_progress.push_back((new_doubled, new_path));
                    }
                }
            }
        }

        final_paths.len()
    }
    // TODO: Add methods of DayN
}

impl Day for Day12 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(self.num_paths(false))
    }

    fn part2(&self) -> Result<usize, &str> {
        Ok(self.num_paths(true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day12::load("data/day12_example1.txt");
        assert_eq!(d.connected.len(), 6);
    }

    #[test]
    fn test_num_paths_1() {
        let d = Day12::load("data/day12_example1.txt");
        assert_eq!(d.num_paths(false), 10);
        let d = Day12::load("data/day12_example2.txt");
        assert_eq!(d.num_paths(false), 19);
        let d = Day12::load("data/day12_example3.txt");
        assert_eq!(d.num_paths(false), 226);
    }

    #[test]
    fn test_num_paths_2() {
        let d = Day12::load("data/day12_example1.txt");
        assert_eq!(d.num_paths(true), 36);
        let d = Day12::load("data/day12_example2.txt");
        assert_eq!(d.num_paths(true), 103);
        let d = Day12::load("data/day12_example3.txt");
        assert_eq!(d.num_paths(true), 3509);
    }
}
