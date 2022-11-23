use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Elt {
    VALUE(usize),
    LEFT,
    RIGHT,
}

#[derive(PartialEq, Debug, Clone)]
struct Num {
    elts : Option<Vec<Elt>>,
}

enum Operation {
    EXPLODE(usize, usize),        // elt_index, num_index
    SPLIT(usize, usize),          // index, value being split
    NOP,                          // No operation 
}

impl Num {

    pub fn zero() -> Num {
        Num{elts: None}
    }
    
    pub fn parse(expr: &str) -> Num {
        let mut v: Vec<Elt> = Vec::new();

        for c in expr.chars() {
            match c {
                '[' => v.push(Elt::LEFT),
                ']' => v.push(Elt::RIGHT),
                '0'..='9' => v.push(Elt::VALUE(c.to_digit(10).unwrap() as usize)),
                _ => (),
              
            }
        }

        if v.len() > 0 {
            Num{elts: Some(v)}
        }
        else {
            Num{elts: None}
        }
    }

    fn print_num(&self) {
        match &self.elts {
            None => {print!("NONE")}
            Some(v) => {
                for e in v {
                    match e {
                        Elt::LEFT => {print!("[");}
                        Elt::RIGHT => {print!("]");}
                        Elt::VALUE(n) => {print!(" {} ", n);}
                    }
                }
                print!("\n");
            }
        }
    }

    pub fn magnitude(&self) -> usize {
        let mut stack: Vec<usize> = Vec::new();

        match &self.elts {
            None => {
                0
            }
            Some(v) => {
                for elt in v.iter() {
                    match elt {
                        Elt::LEFT => (),
                        Elt::RIGHT => {
                            // A pair closed, combine two values from stack
                            let b = stack.pop().unwrap();
                            let a = stack.pop().unwrap();
                            stack.push(3*a+2*b);
                        },
                        Elt::VALUE(n) => {
                            // Push this value on the stack.
                            stack.push(*n);
                        },
                    }
                }
        
                // The stack should be left with one value, the magnitude.
                stack.pop().unwrap()
            }
        }

        
    }

    pub fn add(&mut self, other: &Num) {

        match &other.elts {
            None => {
                // Adding zero do nothing.
            }
            Some(other_elts) => {
                match &mut self.elts {
                    None => {
                        let mut new_elts: Vec<Elt> = Vec::new();

                        // copy in other's elements
                        for e in other_elts {
                            new_elts.push(*e);
                        }

                        self.elts = Some(new_elts)
                    }
                    Some(my_elts) => {
                        // Update myElts to wrap and append otherElts
                        my_elts.insert(0, Elt::LEFT);
                        // Create new vector and copy in other's elements
                        for e in other_elts {
                            my_elts.push(*e);
                        }
                        my_elts.push(Elt::RIGHT);
                    }
                }
            }
        };

        self.reduce();
    }

    pub fn old_add(a: Num, b: Num) -> Num {
        let mut new_num = match b.elts {
            None => {
                match a.elts {
                    None => {
                        // Nothing plus nothing
                        Num{ elts: None}
                    }
                    Some(a_elts) => {
                        // result will be a copy of a
                        let mut new_v: Vec<Elt> = Vec::new();
                        for elt in a_elts {
                            new_v.push(elt);
                        }
                        Num{elts: Some(new_v)}
                    }
                }
            }
            Some(b_elts) => {
                let mut new_v: Vec<Elt> = Vec::new();
                match a.elts {
                    None => {
                        // result will be a copy of b
                        for elt in b_elts {
                            new_v.push(elt);
                        }
                    }
                    Some(a_elts) => {
                        // result will be a concatenated with b, wrapped in left, right
                        new_v.push(Elt::LEFT);
                        for elt in a_elts {
                            new_v.push(elt);
                        }
                        for elt in b_elts {
                            new_v.push(elt);
                        }
                        new_v.push(Elt::RIGHT);
                    }
                }

                Num{elts: Some(new_v)}
            }
        };

        new_num.reduce();

        new_num
    }

    pub fn reduce(&mut self) {
        let mut changing = true;

        while changing {
            changing = false;
            // look for an operation that needs doing.
            let operation = self.scan_for_explode();

            match operation {
                Operation::EXPLODE(elt_index, num_index) => {
                    changing = true;
                    self.explode(elt_index, num_index);
                    // print!("explode to :");
                    // self.print_num();
                }
                _ => {
                    let operation = self.scan_for_split();

                    match operation {
                        Operation::SPLIT(elt_index, value) => {
                            changing = true;
                            self.split(elt_index, value);
                            // print!("split to :");
                            // self.print_num();
                        }
                        _ => ()
                    }
                }
            }
        }
    }

    pub fn scan_for_explode(&mut self) -> Operation {
        let mut num_index = 0;  // Index of a regular number in this Num
        let mut level: i32 = 0;
        let mut elt_index: usize = 0;

        match &mut self.elts {
            None => {
                return Operation::NOP;
            }
            Some(my_elts) => {
                for elt in my_elts.iter() {
                    match elt {
                        Elt::LEFT => {
                            level += 1;
                            if level >= 5 {
                                return Operation::EXPLODE(elt_index, num_index)
                            }
                        }
                        Elt::RIGHT => {
                            level -= 1;
                        }
                        Elt::VALUE(_) => {
                            num_index += 1;
                        }
                    }
                    elt_index += 1;
                }
            }
        }

        Operation::NOP
    }

    pub fn scan_for_split(&mut self) -> Operation {
        let mut elt_index: usize = 0;

        match &mut self.elts {
            None => {
                return Operation::NOP;
            }
            Some(my_elts) => {
                for elt in my_elts.iter() {
                    match elt {
                        Elt::VALUE(n) => {
                            if *n >= 10 {
                                return Operation::SPLIT(elt_index, *n);
                            }
                        }
                        _ => ()
                    }
                    elt_index += 1;
                }
            }
        }

        Operation::NOP
    }

    fn explode(&mut self, replace_elt_index:usize, left_num_index:usize) {
        // elt_index is element that went too deep.  It should be replaced
        // num_index is the index of the number that should get the left component.
        // num_index+2 is the index (after substituting 0) of the number that gets right.

        match &mut self.elts {
            None => {
                // Explode does nothing
            }
            Some(v) => {
                // get left and right components of explode.
                let left_val = match v[replace_elt_index+1] {
                    Elt::LEFT => { 
                        print!("Problem at elt_num {}\n", replace_elt_index+1);
                        self.print_num();
                        panic!("Expected first number! Got Left at {}", replace_elt_index+1) }
                    Elt::RIGHT => { panic!("Expected first number! Got Right") }
                    Elt::VALUE(n) => n,
                };
                let right_val = match v[replace_elt_index+2] {
                    Elt::LEFT => { panic!("Expected second number!  Got left") }
                    Elt::RIGHT => { panic!("Expected second number! Got right") }
                    Elt::VALUE(n) => n,
                };

                // Create a new vector to hold elements
                let mut new_elts: Vec<Elt> = Vec::new();
                let mut elt_index = 0;
                let mut num_index = 0;
                for elt in v.iter() {
                    if elt_index == replace_elt_index {
                        // push the value 0
                        new_elts.push(Elt::VALUE(0));
                        num_index += 1;
                    }
                    else if (elt_index > replace_elt_index) && (elt_index <= replace_elt_index+3) {
                        // don't push anything, we're skipping the exploded part
                    }
                    else {
                        match elt {
                            Elt::VALUE(n) => { 
                                num_index += 1;
                                if num_index == left_num_index {
                                    // add left_val to this number
                                    new_elts.push(Elt::VALUE(n+left_val));
                                } 
                                else if num_index == left_num_index+2 {
                                    // add right_val to this number
                                    new_elts.push(Elt::VALUE(n+right_val));
                                }
                                else {
                                    // leave the number as it was
                                    new_elts.push(Elt::VALUE(*n));
                                }
                            }
                            Elt::LEFT => {
                                // push everything else as is
                                new_elts.push(Elt::LEFT);
                            }
                            Elt::RIGHT => {
                                // push everything else as is
                                new_elts.push(Elt::RIGHT);
                            }
                        }       
                    }
                    elt_index += 1;
                }
                self.elts = Some(new_elts);
            }
        }
    }

    pub fn split(&mut self, replace_elt_index: usize, value: usize) {
        match &mut self.elts {
            None => (),
            Some(v) => {
                let left = value/2;
                let right = if value & 1 != 0 {value/2 + 1 } else {value/2};
        
                // Create a new vector to hold elements
                let mut new_elts: Vec<Elt> = Vec::new();
                let mut elt_index = 0;
                for elt in v.iter() {
                    if elt_index == replace_elt_index {
                        // push the new pair instead of the old value
                        new_elts.push(Elt::LEFT);
                        new_elts.push(Elt::VALUE(left));
                        new_elts.push(Elt::VALUE(right));
                        new_elts.push(Elt::RIGHT);
                    }
                    else {
                        new_elts.push(*elt);
                    }
        
                    elt_index += 1;
                  }
        
               self.elts = Some(new_elts);
            }
        }
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
        let mut sum = Num::zero();
        for n in self.numbers.iter() {
            sum.add(n); 
        }

        Ok(sum.magnitude())
    }

    fn part2(&self) -> Result<usize, &str> {
        let mut largest = 0;
        for index1 in 0..self.numbers.len() {
            for index2 in 0..self.numbers.len() {
                if index1 == index2 {
                    continue;
                }
                let mut sum = Num::zero();
                sum.add(self.numbers.get(index1).unwrap());
                sum.add(self.numbers.get(index2).unwrap());
                let magnitude = sum.magnitude();

                if magnitude > largest {
                    largest = magnitude;
                }
            }
        }
        Ok(largest)
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
        let test_cases:Vec<(&str, &str, usize, usize)> = vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]", 4, 0),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]", 8, 4),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]", 7, 3),
            ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", 7, 3),
            ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]", 17, 7),
        ];

        for case in test_cases {
            let mut num = Num::parse(case.0);
            let expected = Num::parse(case.1);
            let elt_index = case.2;
            let num_index = case.3;

            num.explode(elt_index, num_index);

            assert_eq!(num, expected);
        }
    }

    #[test]
    fn test_split() {
        let test_cases:Vec<(Num, Num, usize, usize)> = vec![
            (Num{elts: Some(vec![Elt::LEFT, Elt::VALUE(12), Elt::VALUE(6), Elt::RIGHT])},
                Num::parse("[[6, 6], 6]"), 1, 12),
            (Num{elts: Some(vec![Elt::LEFT, Elt::VALUE(6), Elt::VALUE(12), Elt::RIGHT])},
                Num::parse("[6, [6, 6]]"), 2, 12),            
            (Num{elts: Some(vec![Elt::LEFT, Elt::VALUE(13), Elt::VALUE(6), Elt::RIGHT])},
                Num::parse("[[6, 7], 6]"), 1, 13),
            (Num{elts: Some(vec![Elt::LEFT, Elt::VALUE(6), Elt::VALUE(13), Elt::RIGHT])},
                Num::parse("[6, [6, 7]]"), 2, 13),
        ];

        for case in test_cases {
            let mut num = case.0;
            let expected = case.1;
            let elt_index = case.2;
            let value = case.3;

            num.split(elt_index, value);

            assert_eq!(num, expected);
        }
    }

    #[test]
    fn test_reduce() {
        let n1 = Num::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let n2 = Num::parse("[1,1]");
        let expected = Num::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        let num = Num::old_add(n1, n2);

        assert_eq!(num, expected);
    }

    #[test]
    fn test_add() {
        let n1 = Num::parse("[1,2]");
        let n2 = Num::parse("[3,4]");

        let actual = Num::old_add(n1, n2);
        let expected = Num::parse("[[1,2],[3,4]]");

        assert_eq!(actual, expected);
    }


    #[test]
    fn test_add_explode() {
        let n1 = Num::parse("[1,2]");
        let n2 = Num::parse("[3,4]");

        let actual = Num::old_add(n1, n2);
        let expected = Num::parse("[[1,2],[3,4]]");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_add_reduce() {
        let mut n1 = Num::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let n2 = Num::parse("[1,1]");
        let expected = Num::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        n1.add(&n2);

        assert_eq!(n1, expected);
    }

    #[test]
    fn test_example1() {
        let d = Day18::load("data/day18_example1.txt");
        let expected = Num::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");

        let mut sum = Num::zero();
        for n in d.numbers.iter() {
            sum.add(n); 
        }

        assert_eq!(sum, expected);
    }

    #[test]
    fn test_example2() {
        let d = Day18::load("data/day18_example2.txt");
        let expected = Num::parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");

        let mut sum = Num::zero();
        for n in d.numbers.iter() {
            sum.add(n); 
        }

        assert_eq!(sum, expected);
        assert_eq!(sum.magnitude(), 4140);
        assert_eq!(d.part1(), Ok(4140));
        assert_eq!(d.part2(), Ok(3993));
    }
}

