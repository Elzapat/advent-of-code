use regex::Regex;
use nalgebra::geometry::Point3;
use array_tool::vec::Intersect;

#[derive(Debug, Clone, Eq, PartialOrd)]
struct Beacon {
    pos: Point3<i32>,
    all_pos: [Point3<i32>; 24],
}

impl PartialEq for Beacon {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

#[derive(Default, Debug, Clone)]
struct Scanner {
    id: usize,
    beacons: Vec<Beacon>,
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let mut scanners = vec![];
    let mut lines = file.lines();

    while let Some(line) = lines.next() {
        let mut cur_scanner = Scanner {
            id: Regex::new(r"([a-z]|-| )").unwrap().replace_all(line, "").parse::<usize>().unwrap(),
            ..Scanner::default()
        };

        for line in &mut lines {
            if line.is_empty() {
                for beacon in cur_scanner.beacons.iter_mut() {
                    let mut index = 0;
                    let mut p = beacon.pos;
                    for _ in 0..4 {
                        for _ in 0..4 {
                            beacon.all_pos[index] = p;
                            p = Point3::new(p.z, p.y, -p.x);
                            index += 1;
                        }

                        beacon.all_pos[index] = Point3::new(p.y, -p.x, p.z);
                        beacon.all_pos[index + 1] = Point3::new(-p.y, p.x, p.z);
                        index += 2;
                        p = Point3::new(p.x, p.z, -p.y);
                    }
                }

                scanners.push(cur_scanner);
                break;
            }

            let mut pos = line.split(',');
            cur_scanner.beacons.push(Beacon {
                pos: Point3::new(
                    pos.next().unwrap().parse::<i32>().unwrap(),
                    pos.next().unwrap().parse::<i32>().unwrap(),
                    pos.next().unwrap().parse::<i32>().unwrap(),
                ),
                all_pos: [Point3::new(0, 0, 0); 24],
            });
        }
    }

    let mut scanners = scanners.clone();
    let mut s0 = scanners.remove(0);
    let mut origins = vec![];

    while !scanners.is_empty() {
        let sn = scanners.remove(0);

        match find_common_beacons(&s0, &sn) {
            Some((result, origin)) => {
                origins.push(origin);

                for p in &result {
                    if !s0.beacons.contains(&Beacon { pos: *p, all_pos: [Point3::new(0, 0, 0); 24] }) {
                        s0.beacons.push(Beacon {pos: *p, all_pos: [Point3::new(0, 0, 0); 24] });
                    }
                }
            },
            None => scanners.push(sn),
        }
    }

    println!("Part 1: {}", s0.beacons.len());
    println!("Part 2: {}", origins.iter().map(|o1| origins.iter().map(|o2| manhattan_distance(*o1, *o2)).max().unwrap()).max().unwrap());
}

fn find_common_beacons(s1: &Scanner, s2: &Scanner) -> Option<(Vec<Point3<i32>>, Point3<i32>)> {
    for i in 0..24 {
        for b1 in &s1.beacons {
            for b2 in &s2.beacons {
                let origin = b2.all_pos[i] - b1.pos;
                let mut result = vec![];

                for b2b in &s2.beacons {
                    result.push(b2b.all_pos[i] - origin);
                }

                if result.intersect(s1.beacons.iter().map(|b| b.pos).collect()).len() >= 12 {
                    return Some((result, origin.into()));
                }
            }
        }
    }

    None
}

fn manhattan_distance(p1: Point3<i32>, p2: Point3<i32>) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs()
}
