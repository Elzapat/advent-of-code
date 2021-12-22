#[derive(Debug, Clone)]
struct Instruction {
    start: String,
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let instructions = file
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let start = parts.next().unwrap().to_owned();
            let ranges = parts.next().unwrap();

            let mut ranges_parts = ranges.split(",");
            let x_range_parts = ranges_parts.next().unwrap().replace("x=", "");
            let mut x_range_parts = x_range_parts.split("..");
            let x_range = (x_range_parts.next().unwrap().parse::<i32>().unwrap(), x_range_parts.next().unwrap().parse::<i32>().unwrap());
            let y_range_parts = ranges_parts.next().unwrap().replace("y=", "");
            let mut y_range_parts = y_range_parts.split("..");
            let y_range = (y_range_parts.next().unwrap().parse::<i32>().unwrap(), y_range_parts.next().unwrap().parse::<i32>().unwrap());
            let z_range_parts = ranges_parts.next().unwrap().replace("z=", "");
            let mut z_range_parts = z_range_parts.split("..");
            let z_range = (z_range_parts.next().unwrap().parse::<i32>().unwrap(), z_range_parts.next().unwrap().parse::<i32>().unwrap());

            Instruction {
                start,
                x_range,
                y_range,
                z_range,
            }
        })
        .collect::<Vec<Instruction>>();

    println!("Part 1: {}", part1(instructions.clone()));
    // 150486856
    println!("Part 2: {}", part2(instructions));
}

fn part2(instructions: Vec<Instruction>) -> u64 {
    let mut x_ranges = vec![];
    let mut y_ranges = vec![];
    let mut z_ranges = vec![];

    instructions
        .iter()
        .for_each(|instr| {
            x_ranges.push(instr.x_range.0);
            x_ranges.push(instr.x_range.1 + 1);
            y_ranges.push(instr.y_range.0);
            y_ranges.push(instr.y_range.1 + 1);
            z_ranges.push(instr.z_range.0);
            z_ranges.push(instr.z_range.1 + 1);
        });

    x_ranges.sort();
    y_ranges.sort();
    z_ranges.sort();

    let len = instructions.len() * 2;
    let mut reactor = vec![vec![vec![false; len]; len]; len];

    for instr in &instructions {
        let x_bounds = (x_ranges.iter().position(|x| *x == instr.x_range.0).unwrap(), x_ranges.iter().position(|x| *x == instr.x_range.1 + 1).unwrap());
        let y_bounds = (y_ranges.iter().position(|y| *y == instr.y_range.0).unwrap(), y_ranges.iter().position(|y| *y == instr.y_range.1 + 1).unwrap());
        let z_bounds = (z_ranges.iter().position(|z| *z == instr.z_range.0).unwrap(), z_ranges.iter().position(|z| *z == instr.z_range.1 + 1).unwrap());

        for i in x_bounds.0..x_bounds.1 {
            for j in y_bounds.0..y_bounds.1 {
                for k in z_bounds.0..z_bounds.1 {
                    reactor[i][j][k] = instr.start == "on";
                }
            }
        }
    }

    let mut count = 0;

    for i in 0..x_ranges.len() - 1 {
        for j in 0..y_ranges.len() - 1 {
            for k in 0..z_ranges.len() - 1 {
                if reactor[i][j][k] {
                    count += (x_ranges[i+1] - x_ranges[i]) as u64 * (y_ranges[j+1] - y_ranges[j]) as u64 * (z_ranges[k+1] - z_ranges[k]) as u64;
                }
            }
        }
    }

    count
}

fn part1(instructions: Vec<Instruction>) -> u32 {
    let mut reactor = vec![vec![vec![false; 101]; 101]; 101];

    for instr in instructions {
        if instr.x_range.0 < -50 || instr.x_range.0 > 50 {
            continue;
        }

        for i in instr.x_range.0..=instr.x_range.1 {
            for j in instr.y_range.0..=instr.y_range.1 {
                for k in instr.z_range.0..=instr.z_range.1 {
                    reactor[(i + 50) as usize][(j + 50) as usize][(k + 50) as usize] = instr.start == "on";
                }
            }
        }
    }

    reactor.iter().map(|i| i.iter().map(|j| j.iter().map(|k| *k as u32).sum::<u32>()).sum::<u32>()).sum::<u32>()
}

    /*
    const CHUNK_SIZE: usize = 100;

    let mut x_bounds = (i32::MAX, i32::MIN);
    let mut y_bounds = (i32::MAX, i32::MIN);
    let mut z_bounds = (i32::MAX, i32::MIN);

    for instr in &instructions {
        if instr.x_range.0 < x_bounds.0 {
            x_bounds.0 = instr.x_range.0;
        }
        if instr.y_range.1 > x_bounds.1 {
            x_bounds.1 = instr.y_range.1;
        }

        if instr.y_range.0 < y_bounds.0 {
            y_bounds.0 = instr.y_range.0;
        }
        if instr.y_range.1 > y_bounds.1 {
            y_bounds.1 = instr.y_range.1;
        }

        if instr.z_range.0 < z_bounds.0 {
            z_bounds.0 = instr.z_range.0;
        }
        if instr.y_range.1 > z_bounds.1 {
            z_bounds.1 = instr.y_range.1;
        }
    }
    println!("{:?} {:?} {:?}", x_bounds, y_bounds, z_bounds);

    let mut on: u64 = 0;

    for x in (x_bounds.0..=(x_bounds.1 + CHUNK_SIZE as i32)).step_by(CHUNK_SIZE) {
        for y in (y_bounds.0..=(y_bounds.1 + CHUNK_SIZE as i32)).step_by(CHUNK_SIZE) {
            for z in (z_bounds.0..=(z_bounds.1 + CHUNK_SIZE as i32)).step_by(CHUNK_SIZE) {
                println!("{}", z);
                let reactor = vec![vec![vec![false; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

                for instr in &instructions {
                    // let x_range;
                    // if instr.x_range.0 <
                }
            }
        }
    }

    on
    */
