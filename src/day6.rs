use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::day::Day;


pub struct Day6 {
    // Timers (Generations until spawning)
    timers: Vec<usize>,
}

impl Day6 {
    pub fn load(filename: &str) -> Day6 {
        // println!("Loading.");
        let mut timers: Vec<usize> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = line.unwrap();

            for s in l.split(',') {
                let n = s.parse::<usize>().unwrap();
                timers.push(n);
            }
        }

        Day6 { timers }
    }

    #[allow(dead_code)]
    // run some generations, return size of population
    // This processes fish one at a time.  A more efficient method is in sim_smart, below
    fn sim(&self, generations: usize) -> usize {
        let mut population: Vec<usize> = Vec::new();

        // init population
        for timer in &self.timers {
            population.push(*timer);
        }
        // println!("{:?}", population);

        for _ in 0..generations {
            let mut new_fish: usize = 0;
            let pop_size = population.len();

            for _ in 0..pop_size {
                // pop from head
                let timer = population.remove(0);
                if timer == 0 {
                    new_fish += 1;             // create a fish at end of this gen
                    population.push(6);  // rollover this timer from 0 to 6
                }
                else {
                    population.push(timer-1);  // timer ticks down.
                }
            }

            for _ in 0..new_fish {
                population.push(8);
            }
            // println!("{:?}", population);
        }

        population.len()
    }

    fn sim_smart(&self, generations: usize) -> usize {
        let mut per_timer: [usize; 9] = [0; 9];

        // initialize per_timer counts based on initial population
        for t in &self.timers {
            per_timer[*t] += 1;
        }

        // update per_timer counts per generation
        for _ in 0..generations {
            // all fish with timer at 0 will spawn new fish with timer 8 at the end
            let spawns = per_timer[0];

            // in next generation, the number with timer=T is the number with timer=T=1 now.
            for t in 0..8 {
                per_timer[t] = per_timer[t+1];
            }

            // Take care of those spawning cases now.
            per_timer[6] += spawns;
            per_timer[8] = spawns;

            // println!("{:?}", per_timer);
        }

        // sum up the fish
        let mut sum = 0;
        for n in 0..9 {
            sum += per_timer[n];
        }

        sum
    }
}

impl Day for Day6 {
    fn part1(&self) -> Result<usize, &str> {
        Ok(self.sim_smart(80))
    }

    fn part2(&self) -> Result<usize, &str> {
        Ok(self.sim_smart(256))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day6::load("data/day6_example1.txt");
        assert_eq!(d.timers.len(), 5);
    }

    #[test]
    fn test_sim() {
        let d = Day6::load("data/day6_example1.txt");
        assert_eq!(d.sim(18), 26);
    }


    #[test]
    fn test_sim_smart() {
        let d = Day6::load("data/day6_example1.txt");
        assert_eq!(d.sim_smart(18), 26);
    }

    #[test]
    fn test_sim_smart2() {
        let d = Day6::load("data/day6_example1.txt");
        assert_eq!(d.sim_smart(256), 26984457539);
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part1() {
        let d = Day6::load("data/day6_input.txt");
        assert_eq!(d.part1(), Ok(372984));
    }

    #[test]
    // Test results based on my inputs.  Yours will be different.
    fn test_part2() {
        let d = Day6::load("data/day6_input.txt");
        assert_eq!(d.part2(), Ok(1681503251694));
    }
}
