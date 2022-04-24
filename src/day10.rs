use std::collections::HashMap;
use std::fs::File;
// use std::intrinsics::assume;
use crate::day::Day;
use std::io::{BufRead, BufReader};

pub struct Day10 {
    lines: Vec<String>,
}

impl Day10 {
    pub fn load(filename: &str) -> Day10 {
        // println!("Loading.");
        let mut lines: Vec<String> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = line.unwrap();
            lines.push(l);
        }

        Day10 { lines }
    }

    fn score_line(&self, line: &String) -> usize {
        const OPENS: &str = "{[(<";
        const CLOSES: &str = "}])>";
        let mut open_for: HashMap<char, char> = HashMap::new();
        for (k, v) in [('}', '{'), (']', '['), (')', '('), ('>', '<')].iter() {
            open_for.insert(*k, *v);
        }

        let mut score_for: HashMap<char, usize> = HashMap::new();
        for (k, v) in [('}', 1197), (']', 57), (')', 3), ('>', 25137)].iter() {
            score_for.insert(*k, *v);
        }

        let mut stack: Vec<char> = Vec::new();

        for c in line.chars() {
            if OPENS.contains(c) {
                // It's an open, push it.
                stack.push(c);
            } else if CLOSES.contains(c) {
                // It's a close, check it.
                let paired = stack.pop().unwrap();
                if paired == *open_for.get(&c).unwrap() {
                    // It's a match
                    continue;
                } else {
                    // It's a mismatch
                    return *score_for.get(&c).unwrap();
                }
            }
        }

        0
    }

    fn syntax_score(&self) -> usize {
        let mut score = 0;
        for line in self.lines.iter() {
            score += self.score_line(&line);
        }
        score
    }

    fn complete_score(&self, line:&String) -> usize {
        let mut score: usize = 0;

        const OPENS: &str = "{[(<";
        const CLOSES: &str = "}])>";
        let mut open_for: HashMap<char, char> = HashMap::new();
        for (k, v) in [('}', '{'), (']', '['), (')', '('), ('>', '<')].iter() {
            open_for.insert(*k, *v);
        }

        let mut score_for: HashMap<char, usize> = HashMap::new();
        for (k, v) in [('}', 1197), (']', 57), (')', 3), ('>', 25137)].iter() {
            score_for.insert(*k, *v);
        }

        let mut stack: Vec<char> = Vec::new();

        // we already know all the opens and closes match
        for c in line.chars() {
            if OPENS.contains(c) {
                // It's an open, push it.
                stack.push(c);
            } else if CLOSES.contains(c) {
                stack.pop();
            }
        }

        // Now all that remains on the stack are unclosed opens.
        while stack.len() > 0 {
            let c = stack.pop().unwrap();
            score *= 5;
            match c {
                '(' => {
                    score += 1;
                }
                '[' => {
                    score += 2;
                }
                '{' => {
                    score += 3;
                }
                '<' => {
                    score += 4;
                }
                _ => {}
            }
        }

        score
    }

    fn complete_scores(&self) -> Vec<usize> {
        let mut scores: Vec<usize> = Vec::new();
        for line in self.lines.iter() {
            if self.score_line(&line) == 0 {
                // This is an incomplete line
                scores.push(self.complete_score(&line));
            }
        }

        scores.sort();
        scores
    }
}

impl Day for Day10 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(self.syntax_score())
    }

    fn part2(&self) -> Result<usize, &str> {
        let complete_scores = self.complete_scores();
        // println!("Found {} complete scores.", complete_scores.len());
        let median_index = complete_scores.len() / 2;
        // println!("Median index is {}.", median_index);
        Ok(complete_scores[median_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day10::load("data/day10_example1.txt");
        assert_eq!(d.lines.len(), 10);
    }

    #[test]
    fn test_scores() {
        let d = Day10::load("data/day10_example1.txt");
        assert_eq!(d.score_line(&d.lines[2]), 1197);
        assert_eq!(d.score_line(&d.lines[4]), 3);
        assert_eq!(d.score_line(&d.lines[5]), 57);
        assert_eq!(d.score_line(&d.lines[7]), 3);
        assert_eq!(d.score_line(&d.lines[8]), 25137);
    }

    #[test]
    fn test_syntax_score() {
        let d = Day10::load("data/day10_example1.txt");
        assert_eq!(d.syntax_score(), 1197+3+57+3+25137);
    }

    #[test]
    fn test_part1() {
        let d = Day10::load("data/day10_input.txt");
        assert_eq!(d.part1(), Ok(370407));
    }

    #[test]
    fn test_complete_score() {
        let d = Day10::load("data/day10_example1.txt");
        assert_eq!(d.complete_score(&String::from("[({(<(())[]>[[{[]{<()<>>")), 288957);
        assert_eq!(d.complete_score(&String::from("[(()[<>])]({[<{<<[]>>(")), 5566);
        assert_eq!(d.complete_score(&String::from("(((({<>}<{<{<>}{[]{[]{}")), 1480781);
        assert_eq!(d.complete_score(&String::from("{<[[]]>}<{[{[{[]{()[[[]")), 995444);
        assert_eq!(d.complete_score(&String::from("<{([{{}}[<[[[<>{}]]]>[]]")), 294);
    }

    #[test]
    fn test_part2() {
        let d = Day10::load("data/day10_example1.txt");
        assert_eq!(d.part2(), Ok(288957));
    }

    #[test]
    fn test_part2_2() {
        let d = Day10::load("data/day10_input.txt");
        assert_eq!(d.part2(), Ok(3249889609));
    }
}
