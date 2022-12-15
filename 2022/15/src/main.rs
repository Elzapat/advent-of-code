use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
}

fn distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn positions(c: (i32, i32), radius: i32) -> Vec<(i32, i32)> {
    let mut pos = vec![];

    for x in c.0 - radius..c.0 + radius {
        for y in c.1 - radius..c.1 + radius {
            if (x - c.0).abs() + (y - c.1).abs() <= radius {
                pos.push((x, y));
            }
        }
    }

    pos
}

fn positions_border(center: (i32, i32), radius: i32) -> Vec<(i32, i32)> {
    let mut pos = vec![];

    let corners = [
        (center.0 - radius, center.1),
        (center.0, center.1 - radius),
        (center.0 + radius, center.1),
        (center.0, center.1 + radius),
        (center.0 - radius, center.1),
    ];

    for (i, (x, y)) in corners.iter().enumerate().skip(1) {
        let a = (y - corners[i - 1].1) / (x - corners[i - 1].0);
        let b = y - a * x;

        if a > 0 {
            pos.extend((center.0 - radius..=center.0).map(|x| (x, a * x + b)));
        } else {
            pos.extend((center.0..=center.0 + radius).map(|x| (x, a * x + b)));
        }
    }

    pos
}

#[inline]
fn is_in_range(point: (i32, i32), center: (i32, i32), radius: i32) -> bool {
    (point.0 - center.0).abs() + (point.1 - center.1).abs() <= radius
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>();
    let regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let mut sensors = vec![];

    for line in &lines {
        for cap in regex.captures_iter(&line) {
            sensors.push(Sensor {
                pos: (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                beacon: (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            });
        }
    }

    const ROW: i32 = 2000000;
    let mut used_pos = HashSet::new();

    for sensor in &sensors {
        let dist = distance(sensor.pos, sensor.beacon);

        for x in sensor.pos.0 - dist..sensor.pos.0 + dist {
            if is_in_range((x, ROW), sensor.pos, dist) {
                used_pos.insert((x, ROW));
            }
        }
    }

    for sensor in &sensors {
        used_pos.remove(&sensor.beacon);
    }

    println!("Part 1: {}", used_pos.len());

    const MAX: i32 = 4000000;

    'main: for s1 in &sensors {
        let dist = distance(s1.pos, s1.beacon);

        for pos in positions_border(s1.pos, dist + 1) {
            if pos.0 < 0 || pos.0 > MAX || pos.1 < 0 || pos.1 > MAX {
                continue;
            }

            let mut found = false;
            for s2 in &sensors {
                if is_in_range(pos, s2.pos, distance(s2.pos, s2.beacon)) {
                    found = true;
                }
            }

            if !found {
                println!("Part 2: {}", pos.0 as i64 * MAX as i64 + pos.1 as i64);
                break 'main;
            }
        }
    }
}
