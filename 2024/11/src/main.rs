fn main() {
    let og_stones = aoc_base::read_input()
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    let mut stones = og_stones.clone();
    let mut next_state = og_stones.clone();

    let mut polyfit_x_values = vec![];
    let mut polyfit_y_values = vec![];

    for blink in 0..25 {
        polyfit_x_values.push(blink as f64);
        polyfit_y_values.push(stones.len() as f64);

        let mut next_state_idx = 0;

        for stone in &stones {
            if *stone == 0 {
                next_state[next_state_idx] = 1;

                next_state_idx += 1;
            } else if stone.to_string().len() % 2 == 0 {
                let ns = stone.to_string();
                next_state[next_state_idx] = ns[..ns.len() / 2].parse::<u128>().unwrap();
                next_state.insert(
                    next_state_idx + 1,
                    ns[ns.len() / 2..].parse::<u128>().unwrap(),
                );

                next_state_idx += 2;
            } else {
                next_state[next_state_idx] *= 2024;

                next_state_idx += 1;
            }
        }

        stones = next_state.clone();
    }

    println!("Part 1: {}", stones.len());
    println!(
        "Part 2: {}",
        og_stones.iter().map(|s| count_stones(*s, 0)).sum::<u128>()
    );
}

#[memoize::memoize]
fn count_stones(stone: u128, blink: u128) -> u128 {
    if blink == 75 {
        return 1;
    }

    if stone == 0 {
        return count_stones(1, blink + 1);
    }

    let ns = stone.to_string();
    if ns.len() % 2 == 0 {
        return count_stones(ns[..ns.len() / 2].parse::<u128>().unwrap(), blink + 1)
            + count_stones(ns[ns.len() / 2..].parse::<u128>().unwrap(), blink + 1);
    }

    count_stones(stone * 2024, blink + 1)
}
