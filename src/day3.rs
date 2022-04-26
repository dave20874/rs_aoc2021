use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day3 {
    report_len: usize,
    reports: Vec<u64>,
}

impl Day3 {
    pub fn load(filename: &str) -> Day3 {
        let mut report_len = 0;
        let mut reports: Vec<u64> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = line.unwrap();
            let report = l.trim();

            if report.len() > 0 {
                // set report_len on first viable report seen
                if report_len == 0 {
                    report_len = report.len();
                }

                // convert string to a value and store it.
                let mut report_val: u64 = 0;
                for c in report.chars() {
                    report_val <<= 1;
                    if c == '1' {
                        report_val |= 1;
                    }
                }
                reports.push(report_val);
            }
        }

        Day3 {
            report_len,
            reports: reports,
        }
    }

    fn gamma(&self) -> u64 {
        let mut balance: Vec<isize> = vec![0; self.report_len];

        for report in &self.reports {
            for position in 0..self.report_len {
                let mask = 1 << position;
                if (report & mask) > 0 {
                    balance[position] += 1;
                } else {
                    balance[position] -= 1;
                }
            }
        }

        let mut result = 0;
        for position in 0..self.report_len {
            if balance[position] > 0 {
                result |= 1 << position;
            }
        }

        result
    }

    fn epsilon(&self) -> u64 {
        let mut balance: Vec<isize> = vec![0; self.report_len];

        for report in &self.reports {
            for position in 0..self.report_len {
                let mask = 1 << position;
                if (report & mask) > 0 {
                    balance[position] += 1;
                } else {
                    balance[position] -= 1;
                }
            }
        }

        let mut result = 0;
        for position in 0..self.report_len {
            if balance[position] < 0 {
                result |= 1 << position;
            }
        }

        result
    }

    fn oxy_recursive(&self, reports: &Vec<u64>, bit_pos: usize) -> u64 {
        // find most common value for the given bit position
        let mask: u64 = 1 << bit_pos;
        let mut balance: i64 = 0;
        for report in reports {
            if (report & mask) == mask {
                balance += 1;
            } else {
                balance -= 1;
            }
        }
        let keep = if balance >= 0 { 1 << bit_pos } else { 0 };

        // keep only values that meet the keep criteria
        let mut filtered: Vec<u64> = Vec::new();
        for report in reports {
            if (report & mask) == keep {
                filtered.push(*report);
            }
        }

        // If only one report left, use it, otherwise recurse.
        let retval;
        if filtered.len() == 1 {
            retval = filtered[0]
        } else {
            retval = self.oxy_recursive(&filtered, bit_pos - 1)
        }

        retval
    }

    fn oxygen(&self) -> u64 {
        self.oxy_recursive(&self.reports, self.report_len - 1)
    }

    fn co2_recursive(&self, reports: &Vec<u64>, bit_pos: usize) -> u64 {
        // find most common value for the given bit position
        let mask: u64 = 1 << bit_pos;
        let mut balance: i64 = 0;
        for report in reports {
            if (report & mask) == mask {
                balance += 1;
            } else {
                balance -= 1;
            }
        }
        let keep = if balance < 0 { 1 << bit_pos } else { 0 };

        // keep only values that meet the keep criteria
        let mut filtered: Vec<u64> = Vec::new();
        for report in reports {
            if (report & mask) == keep {
                filtered.push(*report);
            }
        }

        // If only one report left, use it, otherwise recurse.
        let retval;
        if filtered.len() == 1 {
            retval = filtered[0]
        } else {
            retval = self.co2_recursive(&filtered, bit_pos - 1)
        }

        retval
    }

    fn co2(&self) -> u64 {
        self.co2_recursive(&self.reports, self.report_len - 1)
    }
}

impl Day for Day3 {
    fn part1(&self) -> Result<usize, &str> {
        Ok((self.gamma() * self.epsilon()) as usize)
    }

    fn part2(&self) -> Result<usize, &str> {
        Ok((self.oxygen() * self.co2()) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day3::load("data/day3_example1.txt");
        assert_eq!(d.reports.len(), 12);
    }

    #[test]
    fn test_gamma() {
        let d = Day3::load("data/day3_example1.txt");
        assert_eq!(d.gamma(), 22);
    }

    #[test]
    fn test_epsilon() {
        let d = Day3::load("data/day3_example1.txt");
        assert_eq!(d.epsilon(), 9);
    }

    #[test]
    fn test_oxygen() {
        let d = Day3::load("data/day3_example1.txt");
        assert_eq!(d.oxygen(), 23);
    }

    #[test]
    fn test_co2() {
        let d = Day3::load("data/day3_example1.txt");
        assert_eq!(d.co2(), 10);
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part1() {
        let d = Day3::load("data/day3_example1.txt");
        assert_eq!(d.part1(), Ok(198));
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part2() {
        let d = Day3::load("data/day3_example1.txt");
        assert_eq!(d.part2(), Ok(230));
    }
}
