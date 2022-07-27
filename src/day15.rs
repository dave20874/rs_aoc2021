use std::collections::HashMap;
use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};
use priority_queue::PriorityQueue;

pub struct Day15 {
    // Risk array HashMap maps coordinate: (usize, usize) to risk: usize
    risk: HashMap<(usize, usize), usize>,
    max_x: usize,
    max_y: usize,
}

impl Day15 {
    pub fn load(filename: &str) -> Day15 {
        let mut risk: HashMap<(usize, usize), usize> = HashMap::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut max_x = 0;
        let mut max_y= 0;

        let mut y = 0;
        for line in reader.lines() {
            let l = &line.unwrap();
            let mut x = 0;
            for c in l.chars() {
                // Get one digit and store it.
                let val = c.to_digit(10).unwrap() as usize;
                risk.insert((x, y), val);

                // Record max X and Y stored.
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }

                // Advance to next column
                x += 1
            }
            y += 1;
        }

        Day15 { risk, max_x, max_y }
    }
}

struct Solver {
    // (x, y, total_risk), priority is usize::MAX-total_risk
    frontier: PriorityQueue<(usize, usize, usize), usize>,
    total_risk: Vec<Vec<usize>>,
    risk: Vec<Vec<usize>>,
    max_x: usize,
    max_y: usize,
    goal_x: usize,
    goal_y: usize,
}

impl Solver {
    fn new(problem: &Day15) -> Solver {
        let mut frontier: PriorityQueue<(usize, usize, usize), usize> = PriorityQueue::new();
        frontier.push((0, 0, 0), usize::MAX-0);

        let mut risk= Vec::new();
        let mut total_risk = Vec::new();
        for y in 0..=problem.max_y {
            let mut risk_row: Vec<usize> = Vec::new();
            let mut total_risk_row: Vec<usize> = Vec::new();
            for x in 0..=problem.max_x {
                risk_row.push(*problem.risk.get(&(x, y)).unwrap());
                total_risk_row.push(0);
            }
            risk.push(risk_row);
            total_risk.push(total_risk_row);
        }
        let goal_x = problem.max_x;
        let goal_y = problem.max_y;

        Solver { frontier, total_risk, risk, max_x: problem.max_x, max_y: problem.max_y, goal_x, goal_y }
    }

    fn new_augmented(problem: &Day15) -> Solver {
        let mut frontier: PriorityQueue<(usize, usize, usize), usize> = PriorityQueue::new();
        frontier.push((0, 0, 0), usize::MAX-0);

        let mut risk= Vec::new();
        let mut total_risk = Vec::new();
        for y_iter in 0..5 {
            for y in 0..=problem.max_y {
                let mut risk_row: Vec<usize> = Vec::new();
                let mut total_risk_row: Vec<usize> = Vec::new();
                for x_iter in 0..5 {
                    for x in 0..=problem.max_x {
                        let mut r = *problem.risk.get(&(x, y)).unwrap();
                        r = ((r + y_iter + x_iter - 1) % 9) + 1;
                        risk_row.push(r);
                        total_risk_row.push(0);
                    }
                }
                risk.push(risk_row);
                total_risk.push(total_risk_row);
            }
        }
        let goal_x = (problem.max_x+1)*5-1;
        let goal_y = (problem.max_y+1)*5-1;
        let max_x = goal_x;
        let max_y = goal_y;

        Solver { frontier, total_risk: total_risk, risk: risk, max_x, max_y, goal_x, goal_y }
    }

    fn run(&mut self) -> usize {
        while !self.frontier.is_empty() && (self.total_risk[self.goal_y][self.goal_x] == 0) {
            let ((x, y, total_risk), _priority) = self.frontier.pop().unwrap();

            let possible_neighbors: [(isize, isize); 4] = [
                (x as isize +1, y as isize),
                (x as isize -1, y as isize),
                (x as isize, y as isize +1),
                (x as isize, y as isize -1)];
            for (nx, ny) in possible_neighbors {
                if (nx > self.max_x as isize) || (nx < 0) || (ny > self.max_y as isize) || (ny < 0) ||
                    (self.total_risk[ny as usize][nx as usize] != 0) {
                    // skip this possible neighbor, it's invalid or already evaluated.
                    continue;
                }
                else {
                    // found a new cell we can evaluate
                    let new_risk = total_risk + self.risk[ny as usize][nx as usize];
                    self.total_risk[ny as usize][nx as usize] = new_risk;
                    self.frontier.push((nx as usize, ny as usize, new_risk), usize::MAX-new_risk);
                }
            }
        }

        // return total risk on best path
        self.total_risk[self.goal_y][self.goal_x]
    }
}

impl Day for Day15 {
    fn part1(&self) -> Result<usize, &str> {
        let mut solver: Solver = Solver::new(self);

        Ok(solver.run())
    }

    fn part2(&self) -> Result<usize, &str> {

        let mut solver: Solver = Solver::new_augmented(self);

        Ok(solver.run())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day15::load("data/day15_example1.txt");
        assert_eq!(d.risk.len(), 100);
        assert_eq!(*d.risk.get(&(5, 5)).unwrap(), 2);
        assert_eq!(d.max_x, 9);
        assert_eq!(d.max_y, 9);
    }

    #[test]
    fn test_run() {
        let d = Day15::load("data/day15_example1.txt");
        let mut solver = Solver::new(&d);
        assert_eq!(solver.run(), 40);
    }

    #[test]
    fn test_run2() {
        let d = Day15::load("data/day15_example1.txt");
        let mut solver = Solver::new_augmented(&d);
        assert_eq!(solver.run(), 315);
    }

    #[test]
    fn test_part1() {
        let d = Day15::load("data/day15_example1.txt");
        assert_eq!(d.part1(), Ok(40));
    }

    #[test]
    fn test_part2() {
        let d = Day15::load("data/day15_example1.txt");
        assert_eq!(d.part2(), Ok(315));
    }

}
