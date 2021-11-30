mod day;
mod day0;
mod day1;

use day::Day;
use day0::Day0;

fn do_day(n: usize, day: &dyn Day) {
    match day.part1() {
        Ok(val) => println!("day {} part 1: {}", n, val),
        Err(msg) => println!("day {} part 1: {}", n, msg),
    }
    match day.part2() {
        Ok(val) => println!("day {} part 2: {}", n, val),
        Err(msg) => println!("day {} part 2: {}", n, msg),
    }
}

fn main() {
    println!("Advent of Code 2021!");

    let day0 = Day0::load("data/day0_input.txt");
    let days: Vec<&dyn Day> = vec![
        &day0,
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
