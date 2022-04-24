use std::collections::{HashMap, VecDeque};
use std::fs::File;
// use std::intrinsics::assume;
use crate::day::Day;
use std::io::{BufRead, BufReader};

pub struct Day9 {
    height_map: HashMap<(usize, usize), usize>,
}

impl Day9 {
    pub fn load(filename: &str) -> Day9 {
        // println!("Loading.");
        let mut height_map: HashMap<(usize, usize), usize> = HashMap::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut row: usize = 0;
        for line in reader.lines() {
            row += 1;
            let l = line.unwrap();
            let mut col: usize = 0;
            for c in l.chars() {
                col += 1;
                let height = c.to_digit(10).unwrap() as usize;
                height_map.insert((col, row), height);
            }
        }

        Day9 {
            height_map,
        }
    }

    fn is_min(&self, coord: &(usize, usize)) -> bool {
        let col = coord.0;
        let row = coord.1;
        let center = self.height_map.get(coord);

        for neighbor in [
            (col - 1, row),
            (col + 1, row),
            (col, row - 1),
            (col, row + 1),
        ] {
            if self.height_map.contains_key(&neighbor) {
                if self.height_map.get(&neighbor) <= center {
                    return false;
                }
            }
        }

        // println!("Min at ({}, {})", col, row);

        // Didn't find a neighbor that made this not a minimum, so ...
        true
    }

    fn sum_risks(&self) -> usize {
        let mut sum: usize = 0;
        for coord in self.height_map.keys() {
            // check to see if this is a local minimum
            if self.is_min(coord) {
                // if so, the risk is h+1.  Add that to sum
                sum += self.height_map.get(coord).unwrap() + 1;
            }
        }

        // return the sum of risks
        sum
    }

    fn drains(&self) -> Vec<(usize, usize)> {
        let mut drains: Vec<(usize, usize)> = Vec::new();

        for coord in self.height_map.keys() {
            // check to see if this is a local minimum
            if self.is_min(coord) {
                drains.push(*coord);
            }
        }

        drains
    }

    fn basin_size(&self, lowest: (usize, usize)) -> usize {
        let mut to_check: VecDeque<(usize, usize)> = VecDeque::new();
        let mut in_basin: Vec<(usize, usize)> = Vec::new();

        to_check.push_back(lowest);
        while to_check.len() > 0 {
            let next = to_check.pop_front().unwrap();
            let height = self.height_map.get(&next).unwrap();
            in_basin.push(next);
            // println!("Adding ({}, {})={} to basin.", next.0, next.1, height);

            for neighbor in [
                (next.0 + 1, next.1),
                (next.0 - 1, next.1),
                (next.0, next.1 + 1),
                (next.0, next.1 - 1),
            ] {
                // See that neighbor coordinate is on map
                if !self.height_map.contains_key(&neighbor) {
                    continue;
                }

                // See that other height is not 9
                let neighbor_height = self.height_map.get(&neighbor).unwrap();
                if *neighbor_height == 9 {
                    continue;
                }

                // See that neighbor is >= height of 'next'
                if *neighbor_height < *height {
                    continue;
                }

                // See that neighbor isn't already in the basin
                if in_basin.contains(&neighbor) {
                    continue;
                }

                // See that neighbor isn't already in to_check
                if to_check.contains(&neighbor) {
                    continue;
                }

                // The neighbor is part of the basin, add it to to_check
                to_check.push_back(neighbor);
            }
        }

        in_basin.len()
    }

    fn largest_basins(&self) -> [usize; 3] {
        let mut largest: [usize; 3] = [0, 0, 0];

        for drain in self.drains() {
            let size = self.basin_size(drain);

            if size > largest[2] {
                largest[0] = largest[1];
                largest[1] = largest[2];
                largest[2] = size;
            } else if size > largest[1] {
                largest[0] = largest[1];
                largest[1] = size;
            } else if size > largest[0] {
                largest[0] = size;
            }
        }

        largest
    }
}

impl Day for Day9 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(self.sum_risks())
    }

    fn part2(&self) -> Result<usize, &str> {
        let largest = self.largest_basins();

        Ok(largest[0] * largest[1] * largest[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day9::load("data/day9_example1.txt");
        assert_eq!(d.height_map.len(), 50);
    }

    #[test]
    fn test_sum_risks() {
        let d = Day9::load("data/day9_example1.txt");
        assert_eq!(d.sum_risks(), 15);
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part1() {
        let d = Day9::load("data/day9_input.txt");
        assert_eq!(d.part1(), Ok(489));
    }

    #[test]
    // Test basin_size
    fn test_basin_size() {
        let d = Day9::load("data/day9_example1.txt");
        assert_eq!(d.basin_size((2, 1)), 3);
        assert_eq!(d.basin_size((10, 1)), 9);
        assert_eq!(d.basin_size((3, 3)), 14);
        assert_eq!(d.basin_size((7, 5)), 9);
    }

    #[test]
    fn test_largest_basins() {
        let d = Day9::load("data/day9_example1.txt");
        assert_eq!(d.largest_basins(), [9, 9, 14]);
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part2_1() {
        let d = Day9::load("data/day9_example1.txt");
        assert_eq!(d.part2(), Ok(9 * 9 * 14));
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part2() {
        let d = Day9::load("data/day9_input.txt");
        assert_eq!(d.part2(), Ok(1056330));
    }
}
