use crate::day::Day;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// use lazy_static::lazy_static;
use regex::Regex;

struct Map {
    // Maps coord (x,y) to count of how many vent lines cover this coordinate.
    vents: HashMap<(usize, usize), usize>,
}

impl Map {
    // count coordinates where danger is >= threshold.
    fn dangers(&self, thresh: usize) -> usize {
        let mut dangers = 0;

        for (_, count) in &self.vents {
            if count >= &thresh {
                dangers += 1;
            }
        }

        dangers
    }

    // display the top 10x10 area of a map (for debugging.)
    #[allow(dead_code)]
    fn display(&self) {
        for y in 0..10 {
            for x in 0..10 {
                let pt = (x, y);
                if self.vents.contains_key(&pt) {
                    print!("{}", self.vents.get(&pt).unwrap());
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

pub struct Day5 {
    // Vent lines represented as tuple of x1, y2, x2, y2
    lines: Vec<(usize, usize, usize, usize)>,
}

impl Day5 {
    pub fn load(filename: &str) -> Day5 {
        // println!("Loading.");
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)").unwrap();
        }
        let mut lines: Vec<(usize, usize, usize, usize)> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = line.unwrap();

            let caps = LINE_RE.captures(&l);
            match caps {
                Some(caps) => {
                    let line = (
                        caps[1].parse::<usize>().unwrap(),
                        caps[2].parse::<usize>().unwrap(),
                        caps[3].parse::<usize>().unwrap(),
                        caps[4].parse::<usize>().unwrap(),
                    );

                    lines.push(line);
                }
                None => {
                    // Blank line
                }
            }
        }

        Day5 { lines }
    }

    // generate a map with count of vent lines crossing each coordinate
    fn gen_map(&self, diagonals: bool) -> Map {
        let mut vents: HashMap<(usize, usize), usize> = HashMap::new();

        for line in &self.lines {
            let (x1, y1, x2, y2) = *line;

            // If not using diagonals, skip this entry
            if !diagonals && (x1 != x2) && (y1 != y2) {
                continue;
            }

            // add points to the map
            let x_incr = if x1 == x2 {
                0
            } else if x1 < x2 {
                1
            } else {
                -1
            };
            let y_incr = if y1 == y2 {
                0
            } else if y1 < y2 {
                1
            } else {
                -1
            };

            let mut x = x1 as isize;
            let mut y = y1 as isize;
            let mut pt = (x as usize, y as usize);
            if vents.contains_key(&pt) {
                vents.insert(pt, vents.get(&pt).unwrap() + 1);
            } else {
                vents.insert(pt, 1);
            }

            while !((x == x2 as isize) && (y == y2 as isize)) {
                x += x_incr;
                y += y_incr;
                pt = (x as usize, y as usize);
                if vents.contains_key(&pt) {
                    vents.insert(pt, vents.get(&pt).unwrap() + 1);
                } else {
                    vents.insert(pt, 1);
                }
            }
        }

        Map { vents }
    }
}

impl Day for Day5 {
    fn part1(&self) -> Result<usize, &str> {
        let map = self.gen_map(false);
        // map.display();
        let dangers = map.dangers(2);

        Ok(dangers)
    }

    fn part2(&self) -> Result<usize, &str> {
        let map = self.gen_map(true);
        // map.display();
        let dangers = map.dangers(2);

        Ok(dangers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day5::load("data/day5_example1.txt");
        assert_eq!(d.lines.len(), 10);
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part1() {
        let d = Day5::load("data/day5_example1.txt");
        assert_eq!(d.part1(), Ok(5));

        let d = Day5::load("data/day5_input.txt");
        assert_eq!(d.part1(), Ok(6397));
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part2() {
        let d = Day5::load("data/day5_example1.txt");
        assert_eq!(d.part2(), Ok(12));

        let d = Day5::load("data/day5_input.txt");
        assert_eq!(d.part2(), Ok(22335));
    }
}
