use crate::day::Day;
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day7 {
    positions: Vec<usize>,
}

impl Day7 {
    pub fn load(filename: &str) -> Day7 {
        // println!("Loading.");
        let mut positions: Vec<usize> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = line.unwrap();
            for n in l.split(",") {
                positions.push(n.parse::<usize>().unwrap());
            }
        }
        positions.sort();

        Day7 { positions }
    }

    fn median(&self) -> usize {
        let mid = self.positions.len() / 2;

        self.positions[mid]
    }

    fn fuel_used(&self, pos: usize) -> usize {
        let mut used = 0;

        for p in &self.positions {
            used += ((*p as isize) - (pos as isize)).abs();
        }

        used as usize
    }

    fn mean(&self) -> f64 {
        let mut sum: usize = 0;
        for n in &self.positions {
            sum += n;
        }

        let avg: f64 = (sum as f64) / self.positions.len() as f64;
        avg
    }

    fn fuel_used2(&self, pos: usize) -> usize {
        let mut used = 0;

        for p in &self.positions {
            let diff: isize = ((*p as isize) - (pos as isize)).abs();
            let delta: isize = diff * (diff + 1) / 2;
            // println!("Move from {} to {}: {} fuel", *p, pos, delta);
            used += delta;
        }

        used as usize
    }
}

impl Day for Day7 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(self.fuel_used(self.median()))
    }

    fn part2(&self) -> Result<usize, &str> {
        let ans1 = self.fuel_used2(self.mean() as usize); // truncate
        let ans2 = self.fuel_used2(1 + self.mean() as usize); // round up

        Ok(min(ans1, ans2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day7::load("data/day7_example1.txt");
        assert_eq!(d.positions.len(), 10);
    }

    #[test]
    fn test_median() {
        let d = Day7::load("data/day7_example1.txt");
        assert_eq!(d.median(), 2);
    }

    #[test]
    fn test_fuel_used() {
        let d = Day7::load("data/day7_example1.txt");
        assert_eq!(d.fuel_used(2), 37);
    }

    #[test]
    fn test_mean() {
        let d = Day7::load("data/day7_example1.txt");
        assert_eq!(d.mean().round() as usize, 5);
    }

    #[test]
    fn test_fuel_used2() {
        let d = Day7::load("data/day7_example1.txt");
        assert_eq!(d.fuel_used2(5), 168);
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part1() {
        let d = Day7::load("data/day7_example1.txt");
        assert_eq!(d.part1(), Ok(37));
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part2() {
        let d = Day7::load("data/day7_example1.txt");
        assert_eq!(d.part2(), Ok(168));
    }
}
