fn main() {
    let input = aoc_base::read_input();
    let mut nb_safe_p1 = 0;
    let mut nb_safe_p2 = 0;

    'main: for line in input.lines() {
        let numbers = line
            .split_whitespace()
            .map(|ns| ns.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if test_sequence(numbers.clone()) {
            nb_safe_p1 += 1;
            nb_safe_p2 += 1;
        } else {
            for d in 0..numbers.len() {
                let mut nc = numbers.clone();
                nc.remove(d);
                if test_sequence(nc.clone()) {
                    nb_safe_p2 += 1;
                    continue 'main;
                }
            }
        }
    }

    println!("Part 1: {nb_safe_p1}");
    println!("Part 2: {nb_safe_p2}");
}

fn test_sequence(numbers: Vec<i32>) -> bool {
    let mut increasing = None;

    for i in 1..numbers.len() {
        let diff = numbers[i - 1] - numbers[i];

        if diff > 0 && diff <= 3 && (increasing.is_none() || increasing == Some(false)) {
            increasing = Some(false);
        } else if diff < 0 && diff >= -3 && (increasing.is_none() || increasing == Some(true)) {
            increasing = Some(true);
        } else {
            return false;
        }
    }

    true
}
