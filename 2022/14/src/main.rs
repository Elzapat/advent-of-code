use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>();

    let mut rocks = HashSet::<(i32, i32)>::new();
    let mut highest_y = 0;

    for line in &lines {
        let points = line.split("->").collect::<Vec<&str>>();

        for (i, point) in points.iter().enumerate().skip(1) {
            let mut coords = point.trim().split(',');
            let mut prev_coords = points[i - 1].trim().split(',');

            let (x, y): (i32, i32) = (
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
            );
            let (px, py): (i32, i32) = (
                prev_coords.next().unwrap().parse().unwrap(),
                prev_coords.next().unwrap().parse().unwrap(),
            );

            let (mut rx, mut ry) = (px, py);
            while rx != x || ry != y {
                rocks.insert((rx, ry));

                if y > highest_y {
                    highest_y = y;
                }

                if x > rx {
                    rx += 1;
                } else if x < rx {
                    rx -= 1;
                }

                if y > ry {
                    ry += 1;
                } else if y < ry {
                    ry -= 1;
                }
            }

            rocks.insert((x, y));
        }
    }

    for x in 100..900 {
        rocks.insert((x, highest_y + 2));
    }

    let mut n_sand = 0;
    let og_len = rocks.len();

    loop {
        let mut sand = (500, 0);

        match simulate_sand(&rocks, &mut sand, &mut 0) {
            Ok(_) => {
                rocks.insert(sand);
                n_sand += 1;
                if sand == (500, 0) {
                    break;
                }
                continue;
            }
            Err(_) => panic!("Not suppoed to enter here in part 2, was for part 1"),
        }
    }

    println!("{}", rocks.len() - og_len);
}

fn simulate_sand(
    rocks: &HashSet<(i32, i32)>,
    mut sand: &mut (i32, i32),
    fall: &mut u32,
) -> Result<(), ()> {
    // For Part 1
    if *fall > 10_000 {
        return Err(());
    }

    sand.1 += 1;
    if rocks.contains(sand) {
        sand.0 -= 1;
        if rocks.contains(sand) {
            sand.0 += 2;
            if rocks.contains(sand) {
                sand.0 -= 1;
                sand.1 -= 1;
                Ok(())
            } else {
                simulate_sand(rocks, sand, fall)
            }
        } else {
            simulate_sand(rocks, sand, fall)
        }
    } else {
        *fall += 1;
        simulate_sand(rocks, sand, fall)
    }
}
