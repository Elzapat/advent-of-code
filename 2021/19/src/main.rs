use regex::Regex;
use std::{
    ops::Sub,
    collections::HashSet,
};

#[derive(Default, Hash, Debug, Copy, Clone, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Pos {
    fn diff_abs(self, other: &Self) -> Self {
        Self {
            x: (self.x - other.x).abs(),
            y: (self.y - other.y).abs(),
            z: (self.z - other.z).abs(),
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Scanner {
    id: usize,
    beacons: Vec<Pos>,
    beacons_all_pos: Vec<Pos>,
}

fn main() {
    let file = std::fs::read_to_string("input.ex.txt").unwrap();

    let mut scanners = vec![];
    let mut lines = file.lines();

    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break,
        };

        let mut cur_scanner = Scanner::default();
        cur_scanner.id = Regex::new(r"([a-z]|-| )").unwrap().replace_all(line, "").parse::<usize>().unwrap();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                for pos in &cur_scanner.beacons {
                    let mut pos = pos.clone();

                    for _ in 0..2 {
                        // Rotations around x axis
                        for _ in 0..4 {
                            pos.y = -1 * pos.z;
                            pos.z = 1 * pos.y;
                            cur_scanner.beacons_all_pos.push(pos);
                        }

                        // Rotations around y axis
                        for _ in 0..4 {
                            pos.x = 1 * pos.z;
                            pos.z = -1 * pos.x;
                            cur_scanner.beacons_all_pos.push(pos);
                        }

                        // Rotations around z axis
                        for _ in 0..4 {
                            pos.x = -1 * pos.y;
                            pos.y = 1 * pos.x;
                            cur_scanner.beacons_all_pos.push(pos);
                        }

                        pos.x *= -1;
                        pos.y *= -1;
                        pos.z *= -1;
                    }
                }

                scanners.push(cur_scanner);
                cur_scanner = Scanner::default();

                break;
            }

            let mut pos = line.split(",");
            cur_scanner.beacons.push(Pos {
                x: pos.next().unwrap().parse::<i32>().unwrap(),
                y: pos.next().unwrap().parse::<i32>().unwrap(),
                z: pos.next().unwrap().parse::<i32>().unwrap(),
            });
        }
    }
    for scanner in &scanners {
        // println!("{}", scanner.id);
        if scanner.id == 0 {
            println!("{:?}", scanner.beacons);
        }
    }

    {
        let scanners = scanners.clone();
        let scanner_pos = vec![Pos::default(); scanners.len()];

        for scanner1 in &scanners {
            for scanner2 in &scanners {
                if scanner1.id == scanner2.id {
                    continue;
                }

                let mut scanner1_differences = HashSet::new();
                for pos1 in &scanner1.beacons {
                    for pos2 in &scanner1.beacons {
                        if pos1 == pos2 {
                            continue;
                        }

                        scanner1_differences.insert(pos1.diff_abs(pos2));
                    }
                }

                let mut scanner2_differences = HashSet::new();
                for pos1 in &scanner2.beacons {
                    for pos2 in &scanner2.beacons {
                        if pos1 == pos2 {
                            continue;
                        }

                        scanner2_differences.insert(pos1.diff_abs(pos2));
                    }
                }

                let intersection: HashSet<&Pos> = scanner1_differences.intersection(&scanner2_differences).collect();
                println!("intersection between {} and {}: {}", scanner1.id, scanner2.id, intersection.len());
                if intersection.len() >= 12 {
                    println!("SHOULD WORK BETWEEN {} and {}", scanner1.id, scanner2.id);
                }
            }
        }

        println!("Part 1: {}", 0);
    }
}
