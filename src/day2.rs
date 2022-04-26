use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Dir {
    FORWARD,
    UP,
    DOWN,
}

struct Command {
    dir: Dir,
    dist: usize,
}

pub struct Day2 {
    commands: Vec<Command>,
}

impl Day2 {
    pub fn load(filename: &str) -> Day2 {
        let mut commands: Vec<Command> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let parts = l.split(" ").collect::<Vec<&str>>();
            if parts.len() == 2 {
                // we have the two parts
                let mut command = Command {
                    dir: Dir::FORWARD,
                    dist: 0,
                };
                match parts[0] {
                    "forward" => {
                        command.dir = Dir::FORWARD;
                    }
                    "down" => {
                        command.dir = Dir::DOWN;
                    }
                    "up" => {
                        command.dir = Dir::UP;
                    }
                    _ => {
                        panic!();
                    }
                }
                command.dist = parts[1].parse::<usize>().unwrap();
                commands.push(command);
            }
        }

        Day2 { commands }
    }

    fn follow_course(&self, mut pos: (usize, usize)) -> (usize, usize) {
        for cmd in &self.commands {
            match &cmd.dir {
                Dir::FORWARD => {
                    pos.0 = pos.0 + cmd.dist;
                } // increase fwd pos
                Dir::DOWN => {
                    pos.1 = pos.1 + cmd.dist;
                } // increase depth
                Dir::UP => {
                    pos.1 = pos.1 - cmd.dist;
                } // decrease depth
            }
        }

        pos
    }

    fn follow_course2(&self, mut pos: (isize, isize, isize)) -> (isize, isize, isize) {
        // pos is (forward, depth, aim)

        for cmd in &self.commands {
            match &cmd.dir {
                Dir::FORWARD => {
                    pos.0 = pos.0 + cmd.dist as isize; // increase fwd pos
                    pos.1 = pos.1 + (cmd.dist as isize) * pos.2; // change depth using aim.
                }
                Dir::DOWN => {
                    pos.2 = pos.2 + cmd.dist as isize; // change aim
                }
                Dir::UP => {
                    pos.2 = pos.2 - cmd.dist as isize; // change aim
                }
            }
        }

        pos
    }
}

impl Day for Day2 {
    fn part1(&self) -> Result<usize, &str> {
        let mut pos: (usize, usize) = (0, 0);
        pos = self.follow_course(pos);

        Ok(pos.0 * pos.1)
    }

    fn part2(&self) -> Result<usize, &str> {
        let mut pos: (isize, isize, isize) = (0, 0, 0);
        pos = self.follow_course2(pos);

        Ok((pos.0 * pos.1) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day2::load("data/day2_example1.txt");
        assert_eq!(d.commands.len(), 6);
    }

    #[test]
    fn test_follow_course() {
        let d = Day2::load("data/day2_example1.txt");
        let mut pos: (usize, usize) = (0, 0);
        pos = d.follow_course(pos);
        assert_eq!(pos.0, 15);
        assert_eq!(pos.1, 10);
    }

    #[test]
    fn test_follow_course2() {
        let d = Day2::load("data/day2_example1.txt");
        let mut pos: (isize, isize, isize) = (0, 0, 0);
        pos = d.follow_course2(pos);
        assert_eq!(pos.0, 15);
        assert_eq!(pos.1, 60);
        assert_eq!(pos.2, 10); // final aim
    }

}
