use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::day::Day;

pub struct Day0 {
    p1: i64,
    p2: i64,
}

impl Day0 {
    pub fn load(filename: &str) -> Day0 {
        let mut numbers: Vec<i64> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            numbers.push(l.parse::<i64>().unwrap());
        }

        Day0 { p1: numbers[0], p2: numbers[1] }
    }
}

impl Day for Day0 {
    fn part1(&self) -> Result<i64, &str> {
        Ok(self.p1)
    }

    fn part2(&self) -> Result<i64, &str> {
        Ok(self.p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day0::load("data/day0_input.txt");
        assert_eq!(d.p1, 69);
        assert_eq!(d.p2, 70);
    }

    #[test]
    fn test_part1() {
        let d = Day0::load("data/day0_input.txt");
        assert_eq!(d.part1(), Ok(69));
    }

    #[test]
    fn test_part2() {
        let d = Day0::load("data/day0_input.txt");
        assert_eq!(d.part2(), Ok(70));
    }
}
