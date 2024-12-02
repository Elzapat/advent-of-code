#[derive(Default, Copy, Clone, Debug)]
struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,

    distance_flown: u32,
    resting: bool,
    time_flying: u32,
    time_resting: u32,

    points: u32,
}

fn main() {
    let input = aoc_base::read_input();
    let mut reindeers = Vec::new();

    for line in input.lines() {
        let mut words = line.split_whitespace().skip(3);
        let speed = words.next().unwrap().parse::<u32>().unwrap();

        let mut words = words.skip(2);
        let fly_time = words.next().unwrap().parse::<u32>().unwrap();

        let mut words = words.skip(6);
        let rest_time = words.next().unwrap().parse::<u32>().unwrap();

        reindeers.push(Reindeer {
            speed,
            fly_time,
            rest_time,
            ..Default::default()
        });
    }

    let mut best_distance = 0;

    for reindeer in &reindeers {
        let distance = compute_distance_in_time(reindeer, 2503);

        if distance > best_distance {
            best_distance = distance;
        }
    }

    println!("Part 1: {best_distance}");

    for _ in 0..2503 {
        for reindeer in &mut reindeers {
            if reindeer.resting {
                reindeer.time_resting += 1;

                if reindeer.time_resting >= reindeer.rest_time {
                    reindeer.resting = false;
                    reindeer.time_resting = 0;
                }
            } else {
                reindeer.time_flying += 1;
                reindeer.distance_flown += reindeer.speed;

                if reindeer.time_flying >= reindeer.fly_time {
                    reindeer.resting = true;
                    reindeer.time_flying = 0;
                }
            }
        }

        let mut lead_distance = 0;
        let mut leaders = Vec::new();

        for reindeer in &mut reindeers {
            if reindeer.distance_flown > lead_distance {
                lead_distance = reindeer.distance_flown;
                leaders.clear();
                leaders.push(reindeer);
            } else if reindeer.distance_flown == lead_distance {
                leaders.push(reindeer);
            }
        }

        for leader in leaders {
            leader.points += 1;
        }
    }

    println!(
        "Part 2: {}",
        reindeers
            .into_iter()
            .map(|r| r.points)
            .fold(0, |mp, p| if p > mp { p } else { mp })
    );
}

fn compute_distance_in_time(reindeer: &Reindeer, time: u32) -> u32 {
    let mut distance_traveled = 0;
    let mut time_passed = 0;

    while time_passed < time {
        if time_passed + reindeer.fly_time > time {
            distance_traveled += (time - time_passed) * reindeer.speed;
            break;
        }

        time_passed += reindeer.fly_time;
        distance_traveled += reindeer.fly_time * reindeer.speed;

        time_passed += reindeer.rest_time;
    }

    distance_traveled
}
