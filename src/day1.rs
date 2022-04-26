use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day1 {
    depths: Vec<usize>,
}

impl Day1 {
    pub fn load(filename: &str) -> Day1 {
        let mut depths: Vec<usize> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            depths.push(l.parse::<usize>().unwrap());
        }

        Day1 { depths }
    }

    fn increases(&self) -> usize {
        let mut count = 0;
        for n in 1..self.depths.len() {
            if self.depths[n - 1] < self.depths[n] {
                count += 1;
            }
        }
        count
    }

    fn avg_increases(&self, window: usize) -> usize {
        let mut avg_depths: Vec<f32> = Vec::new();
        for n in 0..self.depths.len() - window + 1 {
            let mut sum = 0;
            for i in 0..window {
                sum += self.depths[n + i];
            }
            avg_depths.push((sum as f32) / (window as f32));
        }

        let mut count = 0;
        for n in 1..avg_depths.len() {
            if avg_depths[n - 1] < avg_depths[n] {
                count += 1;
            }
        }
        count
    }
}

impl Day for Day1 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(self.increases())
    }

    fn part2(&self) -> Result<usize, &str> {
        Ok(self.avg_increases(3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day1::load("data/day1_example1.txt");
        assert_eq!(d.depths.len(), 10);
    }

    #[test]
    fn test_increases() {
        let d = Day1::load("data/day1_example1.txt");
        assert_eq!(d.increases(), 7);
    }

    #[test]
    fn test_avg_increases() {
        let d = Day1::load("data/day1_example1.txt");
        assert_eq!(d.avg_increases(3), 5);
    }
}
