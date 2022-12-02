fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let mut score = 0;

    for line in input.lines() {
        let ennemy = line.chars().nth(0).unwrap();
        let me = line.chars().nth(2).unwrap();

        // Part 1
        /*
            match ennemy {
                'A' => match me {
                    'X' => score += 3 + 1,
                    'Y' => score += 6 + 2,
                    'Z' => score += 0 + 3,
                    _ => unreachable!(),
                },
                'B' => match me {
                    'X' => score += 0 + 1,
                    'Y' => score += 3 + 2,
                    'Z' => score += 6 + 3,
                    _ => unreachable!(),
                },
                'C' => match me {
                    'X' => score += 6 + 1,
                    'Y' => score += 0 + 2,
                    'Z' => score += 3 + 3,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        */

        // Part 2
        match ennemy {
            'A' => match me {
                'X' => score += 0 + 3,
                'Y' => score += 3 + 1,
                'Z' => score += 6 + 2,
                _ => unreachable!(),
            },
            'B' => match me {
                'X' => score += 0 + 1,
                'Y' => score += 3 + 2,
                'Z' => score += 6 + 3,
                _ => unreachable!(),
            },
            'C' => match me {
                'X' => score += 0 + 2,
                'Y' => score += 3 + 3,
                'Z' => score += 6 + 1,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    println!("{score}");
}
