use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
#[derive(PartialEq)]
struct Num {
    left: Elt,
    right: Elt,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Elt {
    SIMPLE(usize),
    COMPOUND(Box<Num>),
}

struct ExplodeTarget {
    index1: usize,
    incr1: isize,
    index2: usize,
    incr2: isize,
    index_rm: usize,
}

impl Elt {
    pub fn magnitude(&self) -> usize {
        match self {
            Elt::SIMPLE(val) => {
                *val
            },
            Elt::COMPOUND(n) => {
                n.magnitude()
            }
        }
    }
}

impl Num {
    // take vector of characters, return Num and number of chars consumed.
    fn parse_aux(expr: &Vec<char>, cursor: usize) -> (Num, usize) {
        let mut consumed = 0;

        // Num must start with '['
        assert_eq!(expr[cursor+consumed], '[');
        consumed += 1;

        // Process left side
        // It starts with a bracket or a number.
        let c = &expr[cursor+consumed];
        let left = match c {
            '[' => {
                // recurse
                let (num, newly_consumed) = Num::parse_aux(expr, cursor+consumed);
                consumed += newly_consumed;
                Elt::COMPOUND(Box::new(num))
            }
            '0'..='9' => {
                // left side is a simple digit
                consumed += 1;
                Elt::SIMPLE(c.to_digit(10).unwrap() as usize)
            }
            _ => {
                // A valid expression shouldn't have anything else
                panic!("Invalid char in expression.");
            }
        };

        // Expect a comma right here between left and right
        assert_eq!(expr[cursor+consumed], ',');
        consumed += 1;

        // Process right side
                // It starts with a bracket or a number.
        let c = &expr[cursor+consumed];
        let right = match c {
            '[' => {
                // recurse
                let (num, newly_consumed) = Num::parse_aux(expr, cursor+consumed);
                consumed += newly_consumed;
                Elt::COMPOUND(Box::new(num))
            }
            '0'..='9' => {
                // left side is a simple digit
                consumed += 1;
                Elt::SIMPLE(c.to_digit(10).unwrap() as usize)
            }
            _ => {
                // A valid expression shouldn't have anything else
                panic!("Invalid char in expression.");
            }
        };

        // Finally, the num must end with a closing bracket
        assert_eq!(expr[cursor+consumed], ']');
        consumed += 1;

        (Num {left, right}, consumed)
    }

    pub fn parse(expr: &str) -> Num {
        let (num, _consumed) = Num::parse_aux(&expr.chars().collect(), 0);

        num
    }

    // Recursively probe a number to see if an explode operation is needed.
    // If so, returns a tuple
    fn probe_explode(&self, depth: isize, offset: isize) -> Option<ExplodeTarget> {

    }



    // Attempt an "explode" operation.
    // Return true if an explode was processed, false otherwise
    pub fn explode(&mut self) -> bool {
        let explode_targets = self.probe_explode(1, 1);
        match explode_targets {
            None => false,
            Some(target) => {
                self.do_explode(1, 1, target);
                true
            }
        }
        false
    }

    // Attempt a split operation.
    // Return true if a split occurred, false otherwise
    pub fn split(&mut self) -> bool {
        // TODO
        false
    }

    pub fn reduce(&mut self) {
        let mut operated = true;
        while operated {
            // attempt explode
            operated = self.explode();

            if !operated {
                // attempt split
                operated = self.split();
            }
        }
    }

    pub fn add(a: Num, b: Num) -> Num {
        let left_elt = Elt::COMPOUND(Box::new(a));
        let right_elt = Elt::COMPOUND(Box::new(b));
        let mut sum = Num{ left: left_elt, right: right_elt };

        sum.reduce();

        sum
    }

    pub fn magnitude(&self) -> usize {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }
}

pub struct Day18 {
    numbers: Vec<Num>,
}

impl Day18 {
    pub fn load(filename: &str) -> Day18 {
        let mut numbers: Vec<Num> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            numbers.push(Num::parse(l));
        }

        Day18 { numbers }
    }
}

impl Day for Day18 {
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
        let d = Day18::load("data/day18_example1.txt");
        assert_eq!(d.numbers.len(), 10);
    }

    #[test]
    fn test_magnitude() {
        let tests = vec![
            ("[9,1]", 29),
            ("[[9,1],[1,9]]", 129),
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488),
        ];
        for (s, mag) in tests {
            let num = Num::parse(s);
            assert_eq!(num.magnitude(), mag);
        }
    }

    #[test]
    fn test_parse1() {
        let n = Num::parse("[8,[9,1]]");
        assert_eq!(n.magnitude(), 3*8+2*(3*9+2*1));
    }

    #[test]
    fn test_parse2() {
        let n = Num::parse("[[8,9],1]");
        assert_eq!(n.magnitude(), 3*(3*8+2*9)+2*1);
    }

    #[test]
    fn test_parse3() {
        let n = Num::parse("[[1,2],[8,9]]");
        assert_eq!(n.magnitude(), 3*(3*1+2*2)+2*(3*8+2*9));
    }

    #[test]
    fn test_explode() {
        // TODO
    }

    #[test]
    fn test_split() {
        // TODO
    }

    #[test]
    fn test_reduce() {
        // TODO
    }

    #[test]
    fn test_add() {
        let n1 = Num::parse("[1,2]");
        let n2 = Num::parse("[3,4]");

        let actual = Num::add(n1, n2);
        let expected = Num::parse("[[1,2],[3,4]]");

        assert_eq!(actual, expected);
    }
}
