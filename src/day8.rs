use std::collections::HashMap;
use std::fs::File;
// use std::intrinsics::assume;
use crate::day::Day;
use std::io::{BufRead, BufReader};

struct Display {
    patterns: Vec<Vec<char>>,
    displayed: Vec<Vec<char>>,
}

impl Display {
    fn common_segs(&self, a: &Vec<char>, b: &Vec<char>) -> usize {
        let mut count = 0;

        for c in a {
            if b.contains(c) {
                count += 1;
            }
        }

        count
    }

    fn decode(&self) -> usize {
        let mut pat_to_value: HashMap<&Vec<char>, usize> = HashMap::new();
        let mut value_to_pat: HashMap<usize, &Vec<char>> = HashMap::new();
        // let mut fives: Vec<&Vec<char>> = Vec::new();
        // let mut sixes: Vec<&Vec<char>> = Vec::new();

        // put unique-width patterns into the map
        for pattern in &self.patterns {
            match pattern.len() {
                2 => {
                    pat_to_value.insert(pattern, 1);
                    value_to_pat.insert(1, pattern);
                }
                3 => {
                    pat_to_value.insert(pattern, 7);
                    value_to_pat.insert(7, pattern);
                }
                4 => {
                    pat_to_value.insert(pattern, 4);
                    value_to_pat.insert(4, pattern);
                }
                // 5 => { fives.push(pattern); }
                // 6 => { sixes.push( pattern); }
                7 => {
                    pat_to_value.insert(pattern, 8);
                    value_to_pat.insert(8, pattern);
                }
                _ => {}
            }
        }

        // Second pass through the patterns.  We can now compare those with length 5 or 6
        // To things we already know to determine which digit they are.
        for pattern in &self.patterns {
            match pattern.len() {
                6 => {
                    if self.common_segs(pattern, value_to_pat[&1]) == 1 {
                        // One segment in common with '1'.  This is the 6.
                        pat_to_value.insert(pattern, 6);
                        value_to_pat.insert(6, pattern);
                    } else {
                        if self.common_segs(pattern, value_to_pat[&4]) == 4 {
                            // Four segments in common with 4: This is the 9
                            pat_to_value.insert(pattern, 9);
                            value_to_pat.insert(9, pattern);
                        } else {
                            // Not four segments in common with 4: This is the 0
                            pat_to_value.insert(pattern, 0);
                            value_to_pat.insert(0, pattern);
                        }
                    }
                }
                5 => {
                    if self.common_segs(pattern, value_to_pat[&1]) == 2 {
                        // Two segments in common with '1'.  This is the 3.
                        pat_to_value.insert(pattern, 3);
                        value_to_pat.insert(3, pattern);
                    } else {
                        if self.common_segs(pattern, value_to_pat[&4]) == 2 {
                            // Two segments in common with 4: This is the 2
                            pat_to_value.insert(pattern, 2);
                            value_to_pat.insert(2, pattern);
                        } else {
                            // Not two segments in common with 4: This is the 5
                            pat_to_value.insert(pattern, 5);
                            value_to_pat.insert(5, pattern);
                        }
                    }
                }
                _ => {}
            }
        }

        //  All 10 patterns should be determined now.
        assert_eq!(pat_to_value.len(), 10);

        // Convert displayed pattern to a number
        let mut value = 0;
        for digit in &self.displayed {
            value *= 10;
            value += pat_to_value[&digit];
        }

        value
    }
}

pub struct Day8 {
    displays: Vec<Display>,
}

impl Day8 {
    pub fn load(filename: &str) -> Day8 {
        // println!("Loading.");
        let mut displays: Vec<Display> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = line.unwrap();
            let parts: Vec<&str> = l.split("|").collect();
            let obs: Vec<&str> = parts[0].trim().split(" ").collect();
            let disp: Vec<&str> = parts[1].trim().split(" ").collect();

            let mut patterns: Vec<Vec<char>> = Vec::new();
            let mut displayed: Vec<Vec<char>> = Vec::new();
            for n in 0..10 {
                let mut pattern: Vec<char> = Vec::new();
                for c in obs[n].trim().chars() {
                    pattern.push(c);
                }
                pattern.sort();
                patterns.push(pattern);
            }
            for n in 0..4 {
                let mut one_display: Vec<char> = Vec::new();
                for c in disp[n].trim().chars() {
                    one_display.push(c);
                }
                one_display.sort();
                displayed.push(one_display);
            }

            displays.push(Display {
                patterns,
                displayed,
            });
        }

        Day8 { displays }
    }

    fn unique_out(&self) -> usize {
        let mut uniques = 0;
        let unique_values: [usize; 4] = [2, 3, 4, 7];

        for display in &self.displays {
            for d in &display.displayed {
                if unique_values.contains(&d.len()) {
                    uniques += 1;
                }
            }
        }

        uniques
    }

    fn sum_displays(&self) -> usize {
        let mut sum = 0;

        for display in &self.displays {
            let value = display.decode();
            // println!("value: {}", value);
            sum += value;
        }
        sum
    }
}

impl Day for Day8 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(self.unique_out())
    }

    fn part2(&self) -> Result<usize, &str> {
        Ok(self.sum_displays())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day8::load("data/day8_example1.txt");
        assert_eq!(d.displays.len(), 10);
    }

    #[test]
    fn test_unique() {
        let d = Day8::load("data/day8_example1.txt");
        assert_eq!(d.unique_out(), 26);
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part1() {
        let d = Day8::load("data/day8_example1.txt");
        // > 296
        assert_eq!(d.part1(), Ok(26));
    }

    #[test]
    fn test_sum_displays() {
        let d = Day8::load("data/day8_example1.txt");
        assert_eq!(d.sum_displays(), 61229);
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part2() {
        let d = Day8::load("data/day8_example1.txt");
        assert_eq!(d.part2(), Ok(61229));
    }
}
