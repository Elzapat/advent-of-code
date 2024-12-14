#[derive(Debug, Copy, Clone)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

fn main() {
    let input = aoc_base::read_input();
    let mut robots = vec![];
    let mut robots_100secs = vec![];
    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;

    for line in input.lines() {
        let re = regex::Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();
        let captures = re.captures(line).unwrap();

        robots.push(Robot {
            pos: (
                captures[1].parse::<i64>().unwrap(),
                captures[2].parse::<i64>().unwrap(),
            ),
            vel: (
                captures[3].parse::<i64>().unwrap(),
                captures[4].parse::<i64>().unwrap(),
            ),
        });
    }

    for second in 0..7687 {
        for robot in &mut robots {
            robot.pos.0 += robot.vel.0;

            if robot.pos.0 < 0 {
                robot.pos.0 += WIDTH;
            }

            if robot.pos.0 > WIDTH - 1 {
                robot.pos.0 -= WIDTH;
            }

            robot.pos.1 += robot.vel.1;

            if robot.pos.1 < 0 {
                robot.pos.1 += HEIGHT;
            }

            if robot.pos.1 > HEIGHT - 1 {
                robot.pos.1 -= HEIGHT;
            }
        }

        if second == 99 {
            robots_100secs = robots.clone();
        }
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for robot in &robots_100secs {
        if robot.pos.0 < WIDTH / 2 && robot.pos.1 < (HEIGHT - 1) / 2 {
            q1 += 1;
        }

        if robot.pos.0 > WIDTH / 2 && robot.pos.1 < HEIGHT / 2 {
            q2 += 1;
        }

        if robot.pos.0 < WIDTH / 2 && robot.pos.1 > HEIGHT / 2 {
            q3 += 1;
        }

        if robot.pos.0 > WIDTH / 2 && robot.pos.1 > HEIGHT / 2 {
            q4 += 1;
        }
    }

    println!("Part 1: {}", q1 * q2 * q3 * q4);
    println!("Part 2:");
    println!("Second 7687:");
    for y in 0..HEIGHT {
        'x_loop: for x in 0..WIDTH {
            for robot in &robots {
                if robot.pos.0 == x && robot.pos.1 == y {
                    print!("#");
                    continue 'x_loop;
                }
            }

            print!(".");
        }
        println!()
    }
    println!();
}
