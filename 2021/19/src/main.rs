use regex::Regex;
use std::collections::HashMap;
use nalgebra::{
    base::Vector3,
    geometry::{ Point3, Rotation3 },
};

#[derive(Debug, Clone)]
struct Beacon {
    pos: Point3<i32>,
    all_pos: Vec<Point3<i32>>,
}

#[derive(Default, Debug, Clone)]
struct Scanner {
    id: usize,
    beacons: Vec<Beacon>,
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
                for beacon in cur_scanner.beacons.iter_mut() {
                    for (i, axis) in [&Vector3::x_axis(), &Vector3::y_axis(), &Vector3::z_axis()].iter().enumerate() {
                        for _ in 0..2 {
                            for j in 0..4 {
                                let rot = Rotation3::from_axis_angle(axis, j as f32 * std::f32::consts::FRAC_PI_2);
                                let pt = Point3::new(beacon.pos.x as f32, beacon.pos.y as f32, beacon.pos.z as f32);
                                let new_pt = rot.transform_point(&pt);
                                beacon.all_pos.push(Point3::new(new_pt.x.round() as i32, new_pt.y.round() as i32, new_pt.z.round() as i32));
                            }

                            match i {
                                0 => beacon.pos.x *= -1,
                                1 => beacon.pos.y *= -1,
                                _ => beacon.pos.z *= -1,
                            }
                        }
                    }
                }

                scanners.push(cur_scanner);
                break;
            }

            let mut pos = line.split(",");
            cur_scanner.beacons.push(Beacon {
                pos: Point3::new(
                    pos.next().unwrap().parse::<i32>().unwrap(),
                    pos.next().unwrap().parse::<i32>().unwrap(),
                    pos.next().unwrap().parse::<i32>().unwrap(),
                ),
                all_pos: vec![],
            });
        }
    }
    /*
    for scanner in &scanners {
        for beacon in &scanner.beacons {
            for pos in &beacon.all_pos {
                println!("{:?}", pos);
            }
        }
    }
    */

    {
        let scanners = scanners.clone();
        let scanner_pos = vec![Point3::<f32>::new(0.0, 0.0, 0.0); scanners.len()];

        for mut scanner1 in &scanners {
            for mut scanner2 in &scanners {
                scanner1 = &scanners[1];
                scanner2 = &scanners[4];
                if scanner1.id == scanner2.id {
                    continue;
                }

                let mut scanner1_differences = HashMap::new();
                for b1 in &scanner1.beacons {
                    for b2 in &scanner2.beacons {
                        for p in &b2.all_pos {
                            if *p == b1.pos {
                                continue;
                            }

                            let diff = b1.pos - *p;
                            // let diff = Point3::new(diff.x.abs(), diff.y.abs(), diff.z.abs());
                            match scanner1_differences.get_mut(&diff) {
                                Some(count) => *count += 1,
                                None => drop(scanner1_differences.insert(diff, 1)),
                            };
                        }
                    }
                }

                /*
                if scanner1.id == 0 && scanner2.id == 4 || scanner2.id == 0 && scanner1.id == 4 {
                    println!("{:?}", "hello");
                    for (p, val) in &scanner1_differences {
                        if *val > 3 {
                            println!("{:?} ---- {}", p, val);
                        }
                    }
                    // println!("{:?}", scanner1_differences);
                }
                */

                if let Some(max) = scanner1_differences.iter().map(|(_, val)| val).max() {
                    if *max >= 12 {
                        println!("intersection between {} and {}: {}", scanner1.id, scanner2.id, max);
                        for (p, val) in &scanner1_differences {
                            if *val >= 12 {
                                println!("{:?}", p);
                            }
                        }
                    }
                }
            }
        }

        println!("Part 1: {}", 0);
    }
}
