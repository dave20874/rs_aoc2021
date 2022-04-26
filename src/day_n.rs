use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct DayN {
    tbd: Vec<usize>,
}

impl DayN {
    pub fn load(filename: &str) -> DayN {
        let mut tbd: Vec<usize> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let _l = &line.unwrap();
            tbd.push(0);
        }

        DayN { tbd }
    }

    // TODO: Add methods of DayN
}

impl Day for DayN {
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
        let d = DayN::load("data/dayN_example1.txt");
        assert_eq!(d.tbd.len(), 10);
    }

    // TODO: Add tests for methods

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_soln() {
        let d = DayN::load("data/dayN_example1.txt");
        assert_eq!(d.part1(), Ok(1));
        assert_eq!(d.part2(), Ok(2));
    }
}
