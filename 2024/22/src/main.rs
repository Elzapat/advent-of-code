use std::collections::HashMap;

fn main() {
    let numbers = aoc_base::read_input()
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut result = 0;

    let mut sequences = HashMap::new();

    for mut number in numbers {
        let mut local_sequences = HashMap::new();
        let mut changes = Vec::new();

        for _ in 0..2000 {
            let new_number = get_next_secret(number);

            let last_price = number
                .to_string()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as i64;

            let price = new_number
                .to_string()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as i64;

            let diff = price - last_price;
            changes.push(diff);

            if changes.len() >= 4 {
                local_sequences
                    .entry(changes[changes.len() - 4..].to_vec())
                    .or_insert(price);
            }

            number = new_number;
        }

        for (key, value) in local_sequences {
            sequences
                .entry(key.clone())
                .or_insert(Vec::new())
                .push(value);
        }

        result += number;
    }

    println!("Part 1: {result}");
    println!(
        "Part 2: {}",
        sequences
            .values()
            .map(|v| v.iter().sum::<i64>())
            .max()
            .unwrap()
    );
}

fn get_next_secret(mut secret: i64) -> i64 {
    secret = mix_number(secret * 64, secret);
    secret = mix_number(secret / 32, secret);
    mix_number(secret * 2048, secret)
}

fn mix_number(mut secret: i64, n: i64) -> i64 {
    secret ^= n;
    secret % 16777216
}
