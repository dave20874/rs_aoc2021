use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Axis {
    X,
    Y,
}

pub struct Day13 {
    coords: HashMap<(isize, isize), bool>,
    folds: Vec<(Axis, isize)>
}

impl Day13 {
    pub fn load(filename: &str) -> Day13 {
        let mut coords: HashMap<(isize, isize), bool> = HashMap::new();
        let mut folds: Vec<(Axis, isize)> = Vec::new();
        lazy_static! {
            static ref COORD_RE: Regex = Regex::new("([0-9]+),([0-9]+)").unwrap();
            static ref FOLD_RE: Regex = Regex::new("fold along ([xy])=([0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();

            // Process coordinates
            let caps = COORD_RE.captures(&l);
            match caps {
                Some(caps) => {
                    let x = caps[1].parse::<isize>().unwrap();
                    let y = caps[2].parse::<isize>().unwrap();
                    coords.insert((x, y), true);
                }
                None => {}
            }

            // Process folds
            let caps = FOLD_RE.captures(&l);
            match caps {
                Some(caps) => {
                    let axis = match &caps[1] {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        _ => panic!(),
                    };
                    let value = caps[2].parse::<isize>().unwrap();
                    folds.push((axis, value));
                }
                None => {}
            }
        }

        Day13 { coords, folds }
    }

    fn fold(&self, input: &HashMap<(isize, isize), bool>, fold: &(Axis, isize)) -> HashMap<(isize, isize), bool> {
        let mut folds:Vec<(Axis, isize)> = Vec::new();

        folds.push((fold.0, fold.1));
        self.folds(input, &folds)
    }

    fn folds(&self, input: &HashMap<(isize, isize), bool>, folds: &Vec<(Axis, isize)>) -> HashMap<(isize, isize), bool> {
        let mut output: HashMap<(isize, isize), bool> = HashMap::new();

        for coord in input.keys() {
            let mut x = coord.0;
            let mut y = coord.1;
            for (axis, value) in folds.iter() {
                // print!("Folding on {:?}={}: ({}, {}) -> ", axis, value, x, y);
                match axis {
                    Axis::X => {
                        // fold left along x=value
                        if x > *value {
                            x = 2*value-x;
                        }
                    }
                    Axis::Y => {
                        // fold up along y=value
                        if y > *value {
                            y = 2*value - y;
                        }
                    }
                }
                // println!("({}, {})", x, y);
            }


            output.insert((x, y), true);
        }

        output
    }

    fn show_result(&self, coords: &HashMap<(isize, isize), bool>) {
        for y in 0..6 {
            for x in 0..40 {
                if coords.contains_key(&(x as isize, y as isize)) {
                    print!("#");
                }
                else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

impl Day for Day13 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(self.fold(&self.coords, &self.folds[0]).len())
    }

    fn part2(&self) -> Result<usize, &str> {
        let result = self.folds(&self.coords, &self.folds);
        self.show_result(&result);
        Ok(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day13::load("data/day13_example1.txt");
        assert_eq!(d.coords.len(), 18);
        assert_eq!(d.folds.len(), 2);
    }

    #[test]
    fn test_one_fold() {
        let d = Day13::load("data/day13_example1.txt");
        let result = d.fold(&d.coords, &d.folds[0]);
        assert_eq!(result.len(), 17);
        d.show_result(&result);
    }

    #[test]
    fn test_all_folds() {
        let d = Day13::load("data/day13_example1.txt");
        let result = d.folds(&d.coords, &d.folds);
        assert_eq!(result.len(), 16);
        d.show_result(&result);
    }

    #[test]
    fn test_all_folds2() {
        // HGAJBEHC
        let d = Day13::load("data/day13_example1.txt");
        let result = d.folds(&d.coords, &d.folds);
        assert_eq!(result.len(), 16);
        d.show_result(&result);
    }
}
