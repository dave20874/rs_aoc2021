use crate::day::Day;
use array2d::Array2D;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day11 {
    energy: Array2D<usize>,
}

impl Day11 {
    pub fn load(filename: &str) -> Day11 {
        let mut cavern = Array2D::filled_with(0, 10, 10);

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut row: usize = 0;
        for line in reader.lines() {
            let l = line.unwrap();
            let mut col: usize = 0;
            for c in l.chars() {
                let energy = c.to_digit(10).unwrap() as usize;
                cavern[(col, row)] = energy;
                col += 1;
            }
            row += 1;
        }

        Day11 { energy: cavern }
    }

    // Updates energy matrix for one step, returns number of flashes in this step.
    fn one_step(energy: &mut Array2D<usize>) -> usize {
        const NEIGHBOR_OFFSETS: [(isize, isize); 8] =
            [(-1, -1), (0, -1), (1, -1),
             (-1, 0),           (1, 0),
             (-1, 1),  (0, 1),  (1, 1)];

        let mut flashes = 0;
        let mut flashers: Vec<(usize, usize)> = Vec::new();

        // start each step by adding one to each energy level
        for y in 0..energy.num_rows() {
            for x in 0..energy.num_columns() {
                energy[(x, y)] += 1;

                if energy[(x, y)] > 9 {
                    flashers.push((x, y));
                }
            }
        }

        // next: process flashes
        while !flashers.is_empty() {
            // get coordinate of next flash
            let (x, y) = flashers.pop().unwrap();

            // count the flashes
            flashes += 1;

            // reset energy to 0 at site of flash
            energy[(x, y)] = 0;

            // dump flash energy into all neighbors
            for (dx, dy) in NEIGHBOR_OFFSETS {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < 10 && ny >= 0 && ny < 10 {
                    let xx = nx as usize;
                    let yy= ny as usize;
                    // println!("Checking energy at ({}, {}).", xx, yy);
                    if energy[(xx, yy)] > 0 {
                        // neighbor gets one unit of energy
                        energy[(xx, yy)] += 1;
                    }
                    if energy[(xx, yy)] > 9 && !flashers.contains(&(xx, yy)) {
                        // neighor will flash, too.
                        flashers.push((xx, yy));
                    }
                }
            }
        }

        flashes
    }

    // Simulates a number of steps and returns the number of flashes observed.
    fn do_steps(&self, steps:usize) -> usize {
        let mut flashes: usize = 0;
        let mut energy: Array2D<usize> = self.energy.clone();

        for _step in 0..steps {
            flashes += Day11::one_step(&mut energy);
        }

        flashes
    }

        // Simulates a number of steps and returns the number of flashes observed.
    fn to_sync(&self) -> usize {
        let mut flashes: usize = 0;
        let mut energy: Array2D<usize> = self.energy.clone();
        let mut steps = 0;

        while flashes != 100 {
            flashes = Day11::one_step(&mut energy);
            steps += 1;
        }

        steps
    }
}

impl Day for Day11 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(self.do_steps(100))
    }

    fn part2(&self) -> Result<usize, &str> {
        Ok(self.to_sync())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day11::load("data/day11_example1.txt");
        assert_eq!(d.energy.num_rows(), 10);
        assert_eq!(d.energy.num_columns(), 10);
        assert_eq!(d.energy[(0, 0)], 5);
        assert_eq!(d.energy[(0, 1)], 2);
        assert_eq!(d.energy[(1, 0)], 4);
        assert_eq!(d.energy[(9, 9)], 6);
    }

    #[test]
    fn test_step1() {
        let d = Day11::load("data/day11_example1.txt");
        assert_eq!(d.do_steps(1), 0);
    }

    #[test]
    fn test_step2() {
        let d = Day11::load("data/day11_example1.txt");
        assert_eq!(d.do_steps(2), 35);
    }

    #[test]
    fn test_step100() {
        let d = Day11::load("data/day11_example1.txt");
        assert_eq!(d.do_steps(100), 1656);
    }

    #[test]
    fn test_part1() {
        let d = Day11::load("data/day11_input.txt");
        assert_eq!(d.part1(), Ok(1721));
    }

    #[test]
    fn test_to_sync() {
        let d = Day11::load("data/day11_example1.txt");
        assert_eq!(d.to_sync(), 195);
    }

    #[test]
    fn test_part2() {
        let d = Day11::load("data/day11_input.txt");
        assert_eq!(d.part2(), Ok(298));
    }
}