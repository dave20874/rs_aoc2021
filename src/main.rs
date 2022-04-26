#[macro_use]
extern crate lazy_static;
extern crate regex;

mod day;
mod day_n;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

use day::Day;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use day10::Day10;
use day11::Day11;

fn do_day(n: usize, day: &dyn Day) {
    match day.part1() {
        Ok(val) => println!("day {}, part 1: {}", n, val),
        Err(msg) => println!("day {}, part 1: {}", n, msg),
    }
    match day.part2() {
        Ok(val) => println!("day {}, part 2: {}", n, val),
        Err(msg) => println!("day {}, part 2: {}", n, msg),
    }
}

fn main() {
    println!("Advent of Code 2021!");
    println!("See adventofcode.com/2021 for background.");
    println!("");

    let day1 = Day1::load("data/day1_input.txt");
    let day2 = Day2::load("data/day2_input.txt");
    let day3 = Day3::load("data/day3_input.txt");
    let day4 = Day4::load("data/day4_input.txt");
    let day5 = Day5::load("data/day5_input.txt");
    let day6 = Day6::load("data/day6_input.txt");
    let day7 = Day7::load("data/day7_input.txt");
    let day8 = Day8::load("data/day8_input.txt");
    let day9 = Day9::load("data/day9_input.txt");
    let day10 = Day10::load("data/day10_input.txt");
    let day11 = Day11::load("data/day11_input.txt");
    let days: Vec<&dyn Day> = vec![
        &day1, &day2, &day3, &day4, &day5, &day6, &day7, &day8, &day9, &day10,
        &day11,
    ];

    let selected_day: Option<usize> = None;
    match selected_day {
        None => {
            // No day selected, do them all
            for (n, day) in days.iter().enumerate() {
                do_day(n + 1, *day);
            }
        }
        Some(n) => {
            do_day(n - 1, days[n - 1]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_day1() {
        let d = Day1::load("data/day1_input.txt");
        assert_eq!(d.part1(), Ok(1301));
        assert_eq!(d.part2(), Ok(1346));
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_day2() {
        let d = Day2::load("data/day2_input.txt");
        assert_eq!(d.part1(), Ok(1383564));
        assert_eq!(d.part2(), Ok(1488311643));
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_day3() {
        let d = Day3::load("data/day3_input.txt");
        assert_eq!(d.part1(), Ok(2972336));
        assert_eq!(d.part2(), Ok(3368358));
    }


    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_day4() {
        let d = Day4::load("data/day4_input.txt");
        assert_eq!(d.part1(), Ok(39902));
        assert_eq!(d.part2(), Ok(26936));
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_day5() {
        let d = Day5::load("data/day5_input.txt");
        assert_eq!(d.part1(), Ok(6397));
        assert_eq!(d.part2(), Ok(22335));
    }


    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_day6() {
        let d = Day6::load("data/day6_input.txt");
        assert_eq!(d.part1(), Ok(372984));
        assert_eq!(d.part2(), Ok(1681503251694));
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_day7() {
        let d = Day7::load("data/day7_input.txt");
        assert_eq!(d.part1(), Ok(355989));
        assert_eq!(d.part2(), Ok(102245489));
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_day8() {
        let d = Day8::load("data/day8_input.txt");
        assert_eq!(d.part1(), Ok(409));
        assert_eq!(d.part2(), Ok(1024649));
    }


    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_day9() {
        let d = Day9::load("data/day9_input.txt");
        assert_eq!(d.part1(), Ok(489));
        assert_eq!(d.part2(), Ok(1056330));
    }

    #[test]
    fn test_day10() {
        let d = Day10::load("data/day10_input.txt");
        assert_eq!(d.part1(), Ok(370407));
        assert_eq!(d.part2(), Ok(3249889609));
    }

    #[test]
    fn test_day11() {
        let d = Day11::load("data/day11_input.txt");
        assert_eq!(d.part1(), Ok(1721));
        assert_eq!(d.part2(), Ok(298));
    }
}

