mod day;
mod day1;

use day::Day;
use day1::Day1;

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
    let days: Vec<&dyn Day> = vec![
        &day1,
    ];

    let selected_day = None;
    match selected_day {
        None => {
            // No day selected, do them all
            for (n, day) in days.iter().enumerate() {
                do_day(n, *day);
            }
        }
        Some(n) => {
            do_day(n, days[n]);
        }
    }
}
