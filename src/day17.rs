use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day17 {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Day17 {
    pub fn new(x_min: isize, x_max: isize, y_min: isize, y_max: isize) -> Day17 {
        Day17 { x_min, x_max, y_min, y_max }
    }

    pub fn load(filename: &str) -> Day17 {
        lazy_static! {
            static ref LINE_RE: Regex =
                // Takes, e.g., 'target area: x=236..262, y=-78..-58'
                // Captures x_min in caps[1]: 236, x_max in caps[2]: 262
                //          y_min in caps[3]: -78, y_max in caps[4]: -58
                Regex::new("target area: x=(-?[0-9]+)..(-?[0-9]+), y=(-?[0-9]+)..(-?[0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;

        for line in reader.lines() {
            let l = &line.unwrap();
            let caps = LINE_RE.captures(&l);
            match caps {
                Some(caps) => {
                    x_min = caps[1].parse::<isize>().unwrap();
                    x_max = caps[2].parse::<isize>().unwrap();
                    y_min = caps[3].parse::<isize>().unwrap();
                    y_max = caps[4].parse::<isize>().unwrap();
                }
                None => {}
            }
        }

        Day17 { x_min, x_max, y_min, y_max }
    }

    // Perform a shot.
    // returns (hit, apex) where:
    //    hit is true if the target zone was hit.
    //    apex is the highest y value achieved.
    fn test_shot(&self, initial_vx: isize, initial_vy: isize) -> (bool, isize, isize, isize) {
        let mut x = 0;
        let mut y = 0;
        let mut vx = initial_vx;
        let mut vy = initial_vy;
        let mut hit = false;
        let mut apex = 0;

        while (x <= self.x_max) && (y >= self.y_min) && !hit {
            // project one time step.
            x += vx;
            y += vy;
            if vx > 0 {
                vx -= 1;
            }
            if vx < 0 {
                vx += 1;
            }
            vy -= 1;

            // update apex
            if y > apex {
                apex = y;
            }

            // evaluate if this is a hit
            if (x >= self.x_min) && (x <= self.x_max) &&
                (y >= self.y_min) && (y <= self.y_max) {
                hit = true;
            }
        }

        (hit, apex, x, y)
    }

    // returns optimal vx, vy, apex of this shot
    fn optimal_shot(&self) -> (isize, isize, isize) {
        let mut vx= 0;
        let mut vy;
        let mut apex = 0;
        let mut hit = false;

        // Any more vy and the projectile will go from 0 to beyond target
        // in one step.  We may have to search for a smaller vy if we can't
        // find a vx that works with this.
        vy = -self.y_min - 1;
        while !hit {
            vx = 0;
            let mut final_x = 0;
            let mut _final_y;
            while !hit && (final_x < self.x_max) {
                // print!("Testing vx: {}, vy: {}", vx, vy);
                (hit, apex, final_x, _final_y) = self.test_shot(vx, vy);
                // println!("-> {}, {}, Apex: {}, Hit: {:?}", final_x, final_y, apex, hit);
                if !hit {
                    vx += 1;
                }
            }

            // try reducing initial vy
            vy -= 1;
        }

        (vx, vy, apex)
    }

    fn num_shots(&self) -> usize {
        let (_highest_vx, highest_vy, _highest_apex) = self.optimal_shot();

        let mut count = 0;
        for vx in 0..=self.x_max+1 {
            for vy in self.y_min..=highest_vy+1 {
                let (hit, _, _, _) = self.test_shot(vx, vy);

                if hit {
                    count += 1;
                }
            }
        }

        count
    }
}

impl Day for Day17 {
    fn part1(&self) -> Result<usize, &str> {
        let (_vx, _vy, apex) = self.optimal_shot();

        Ok(apex as usize)
    }

    fn part2(&self) -> Result<usize, &str> {
        Ok(self.num_shots())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day17::load("data/day17_example1.txt");
        assert_eq!(d.x_min, 20);
        assert_eq!(d.x_max, 30);
        assert_eq!(d.y_min, -10);
        assert_eq!(d.y_max, -5);
    }

    #[test]
    fn test_new() {
        let d = Day17::new(1, 2, 3, 4);
        assert_eq!(d.x_min, 1);
        assert_eq!(d.x_max, 2);
        assert_eq!(d.y_min, 3);
        assert_eq!(d.y_max, 4);
    }

    #[test]
    fn test_test_shot() {
        let d = Day17::new(20, 30, -10, -5);
        let (hit, apex, _, _) = d.test_shot(7, 2);
        assert_eq!(hit, true);
        assert_eq!(apex, 3);

        let (hit, apex, _, _) = d.test_shot(6, 3);
        assert_eq!(hit, true);
        assert_eq!(apex, 6);

        let (hit, apex, _, _) = d.test_shot(9, 0);
        assert_eq!(hit, true);
        assert_eq!(apex, 0);

        let (hit, apex, _, _) = d.test_shot(6, 9);
        assert_eq!(hit, true);
        assert_eq!(apex, 45);

        let (hit, apex, x, y) = d.test_shot(6, 10);
        assert_eq!(hit, false);
        assert_eq!(apex, 55);
        assert_eq!(x, 21);
        assert_eq!(y, -11);

        let (hit, apex, _, _) = d.test_shot(17, -4);
        assert_eq!(hit, false);
        assert_eq!(apex, 0);
    }

    #[test]
    fn test_optimize() {
        let d = Day17::new(20, 30, -10, -5);
        let (_vx, _vy, apex) = d.optimal_shot();
        // assert_eq!(vx, 7);
        // assert_eq!(vy, 2);
        assert_eq!(apex, 45);
    }

    #[test]
    fn test_num_shots() {
        let d = Day17::new(20, 30, -10, -5);
        let count = d.num_shots();
        assert_eq!(count, 112);
    }

    #[test]
    fn test_part1() {
        let d = Day17::load("data/day17_example1.txt");
        assert_eq!(d.part1(), Ok(45));
    }

    #[test]
    fn test_part2() {
        let d = Day17::load("data/day17_example1.txt");
        assert_eq!(d.part2(), Ok(112));
    }

}
