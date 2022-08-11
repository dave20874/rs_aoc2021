use crate::day::Day;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day16 {
    message: String,
}

impl Day16 {
    pub fn load(filename: &str) -> Day16 {
        let mut line: String = String::new();
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("([0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        reader.read_line(&mut line).unwrap();
        let message = line.trim_end();

        Day16 { message: message.to_string() }
    }

    pub fn to_bits(&self) -> Vec<bool> {
        let mut bits = Vec::new();
        for c in self.message.chars() {
            let new_bits = match c {
                '0' => [false, false, false, false],
                '1' => [false, false, false,  true],
                '2' => [false, false,  true, false],
                '3' => [false, false,  true,  true],
                '4' => [false,  true, false, false],
                '5' => [false,  true, false,  true],
                '6' => [false,  true,  true, false],
                '7' => [false,  true,  true,  true],
                '8' => [ true, false, false, false],
                '9' => [ true, false, false,  true],
                'A' => [ true, false,  true, false],
                'B' => [ true, false,  true,  true],
                'C' => [ true,  true, false, false],
                'D' => [ true,  true, false,  true],
                'E' => [ true,  true,  true, false],
                'F' => [ true,  true,  true,  true],
                _ => { panic!() }
            };
            new_bits.map(|b| bits.push(b));
        }

        bits
    }

    // Scans one packet, returns (bits consumed, sum of versions)
    fn scan_packet(&self, bits: &[bool]) -> (isize, usize, usize) {
        // First three bits are version id

        let mut ver: usize = 0;
        let mut sum: usize = ver;
        let mut value: isize;

        let mut cursor = 0;
        for _ in 0..3 {
            ver *= 2;
            if bits[cursor] {
                ver += 1;
            }
            cursor += 1;
        }
        sum += ver;
        // println!("Ver = {}", ver);

        let mut id: usize = 0;
        for _ in 0..3 {
            id *= 2;
            if bits[cursor] {
                id += 1;
            }
            cursor += 1;
        }
        // println!("id = {} ", id);

        if id == 4 {
            // literal.  Value is encoded in chunks of 5 bits
            let mut literal_value: usize = 0;
            let mut non_terminal = true;
            while non_terminal {
                non_terminal = bits[cursor];
                cursor += 1;
                for _ in 0..4 {
                    literal_value *= 2;
                    if bits[cursor] {
                        literal_value += 1;
                    }
                    cursor += 1;
                }
            }

            value = literal_value as isize;
            // println!("literal: {}", _literal_value);
        }
        else {
            // non-literal.
            let mut sub_values: Vec<isize> = Vec::new();

            let i = bits[cursor];
            cursor += 1;
            if i {
                // next 11 bits are number of sub-packets contained in this one.
                let mut len = 0;
                for _ in 0..11 {
                    len *= 2;
                    if bits[cursor] {
                        len += 1;
                    }
                    cursor += 1;
                }
                // println!("number of sub-packets: {}", len);

                // Process <len> subpackets
                for _ in 0..len {
                    // println!("----------------");
                    let (value, consumed, ver_sum) = self.scan_packet(&bits[cursor..]);
                    sub_values.push(value);
                    cursor += consumed;
                    sum += ver_sum;
                }
                // println!("----------------");
            }
            else {
                // next 15 bits are total length in bits of subpackets of this one.
                let mut len = 0;
                for _ in 0..15 {
                    len *= 2;
                    if bits[cursor] {
                        len += 1;
                    }
                    cursor += 1;
                }
                // println!("{} bits of subpackets", len);

                // Process <len> bits of subpackets
                while len > 0 {
                    // println!("======================");
                    let (value, consumed, sum_ver) = self.scan_packet(&bits[cursor..]);
                    sub_values.push(value);
                    cursor += consumed;
                    len -= consumed;
                    sum += sum_ver;
                }
                // println!("===========================");
            }

            // compute value based on packet id and sub_values
            value = match id {
                0 => {
                    // Sum
                    value = 0;
                    for x in sub_values {
                        value += x;
                    }

                    value
                }
                1 => {
                    // Product
                    value = 1;
                    for x in sub_values {
                        value *= x;
                    }

                    value
                }
                2 => {
                    // Minimum
                    value = sub_values[0];
                    for x in sub_values {
                        if x < value {
                            value = x;
                        }
                    }

                    value
                }
                3 => {
                    // Maximum
                    value = sub_values[0];
                    for x in sub_values {
                        if x > value {
                            value = x;
                        }
                    }

                    value
                }
                4 => {
                    // Literal (won't hit this case)
                    panic!();
                }
                5 => {
                    // Greater than
                    value = 0;
                    if sub_values[0] > sub_values[1] {
                        value = 1;
                    }

                    value
                }
                6 => {
                    // Less than
                    value = 0;
                    if sub_values[0] < sub_values[1] {
                        value = 1;
                    }

                    value
                }
                7 => {
                    // Equal To
                    value = 0;
                    if sub_values[0] == sub_values[1] {
                        value = 1;
                    }

                    value
                }
                _ => {
                    // Anything else is illegal
                    panic!()
                }
            }
        }

        // println!("Scanned {}, sum = {}", cursor, sum);
        (value, cursor, sum)
    }

}

impl Day for Day16 {
    fn part1(&self) -> Result<usize, &str> {
        let bits = self.to_bits();
        let (_value, _consumed, sum_ver) = self.scan_packet(&bits);

        Ok(sum_ver)
    }

    fn part2(&self) -> Result<usize, &str> {
        let bits = self.to_bits();
        let (value, _consumed, _sum_ver) = self.scan_packet(&bits);

        Ok(value as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day16::load("data/day16_example1.txt");
        assert_eq!(d.message.len(), 6);
        let d = Day16::load("data/day16_example2.txt");
        assert_eq!(d.message.len(), 14);
    }

    #[test]
    fn test_to_bits() {
        let d = Day16::load("data/day16_example1.txt");
        let bits = d.to_bits();
        assert_eq!(bits.len(), 24);
        assert_eq!(bits[0], true);
        assert_eq!(bits[1], true);
        assert_eq!(bits[2], false);
        assert_eq!(bits[3], true);
        let d = Day16::load("data/day16_example2.txt");
        assert_eq!(d.to_bits().len(), 14*4);
    }

    #[test]
    fn test_scan() {
        let d = Day16::load("data/day16_example1.txt");
        let bits = d.to_bits();
        let (_value, _consumed, sum_ver) = d.scan_packet(&bits);
        assert_eq!(sum_ver, 6);

        println!("------");
        let d = Day16::load("data/day16_example2.txt");
        let bits = d.to_bits();
        let (_value, _consumed, sum_ver) = d.scan_packet(&bits);
        assert_eq!(sum_ver, 9);

        println!("------");
        let d = Day16::load("data/day16_example7.txt");
        let bits = d.to_bits();
        let (_value, _consumed, sum_ver) = d.scan_packet(&bits);
        assert_eq!(sum_ver, 31);
    }

    #[test]
    fn test_scan_value() {
        let d = Day16::load("data/day16_example8.txt");
        let bits = d.to_bits();
        let (value, _consumed, _sum_ver) = d.scan_packet(&bits);
        assert_eq!(value, 3);

        let d = Day16::load("data/day16_example9.txt");
        let bits = d.to_bits();
        let (value, _consumed, _sum_ver) = d.scan_packet(&bits);
        assert_eq!(value, 54);

        let d = Day16::load("data/day16_example10.txt");
        let bits = d.to_bits();
        let (value, _consumed, _sum_ver) = d.scan_packet(&bits);
        assert_eq!(value, 7);

        let d = Day16::load("data/day16_example11.txt");
        let bits = d.to_bits();
        let (value, _consumed, _sum_ver) = d.scan_packet(&bits);
        assert_eq!(value, 9);

        let d = Day16::load("data/day16_example12.txt");
        let bits = d.to_bits();
        let (value, _consumed, _sum_ver) = d.scan_packet(&bits);
        assert_eq!(value, 1);

        let d = Day16::load("data/day16_example13.txt");
        let bits = d.to_bits();
        let (value, _consumed, _sum_ver) = d.scan_packet(&bits);
        assert_eq!(value, 0);

        let d = Day16::load("data/day16_example14.txt");
        let bits = d.to_bits();
        let (value, _consumed, _sum_ver) = d.scan_packet(&bits);
        assert_eq!(value, 0);

        let d = Day16::load("data/day16_example15.txt");
        let bits = d.to_bits();
        let (value, _consumed, _sum_ver) = d.scan_packet(&bits);
        assert_eq!(value, 1);
    }

    #[test]
    fn test_part1() {
        let d = Day16::load("data/day16_example7.txt");
        assert_eq!(d.part1(), Ok(31));
    }

    #[test]
    fn test_part2() {
        let d = Day16::load("data/day16_example15.txt");
        assert_eq!(d.part2(), Ok(1));
    }
}
