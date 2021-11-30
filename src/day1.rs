use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::day::Day;

pub struct Day1 {
    p1: i64,  // TODO: modify
    p2: i64,  // TODO: modify
}

impl Day1 {
    pub fn load(filename: &str) -> Day1 {
        let mut numbers: Vec<i64> = Vec::new();  // TODO: remove

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            numbers.push(l.parse::<i64>().unwrap());  // TODO: remove
        }

        Day1 { p1: numbers[0], p2: numbers[1] }  // TODO: modify
    }
}

impl Day for Day1 {
    fn part1(&self) -> Result<i64, &str> {
        Ok(self.p1)  // TODO: modify
    }

    fn part2(&self) -> Result<i64, &str> {
        Ok(self.p2)  // TODO: modify
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day1::load("data/day1_input.txt");  // TODO: modify
        assert_eq!(d.p1, 69);                       // TODO: modify
        assert_eq!(d.p2, 70);                       // TODO: modify
    }

    #[test]
    fn test_part1() {
        let d = Day1::load("data/day1_input.txt");  // TODO: modify
        assert_eq!(d.part1(), Ok(69));
    }

    #[test]
    fn test_part2() {
        let d = Day1::load("data/day1_input.txt");   // TODO: modify
        assert_eq!(d.part2(), Ok(70));
    }
}
