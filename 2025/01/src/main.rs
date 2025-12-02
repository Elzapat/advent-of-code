fn main() {
    let input = aoc_base::read_input();

    let mut dial = 50;
    let mut result = 0;
    let mut result_p2 = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let rotation = line.chars().next().unwrap();
        let n = line
            .trim()
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        if rotation == 'L' {
            for _ in 0..n {
                dial -= 1;

                if dial < 0 {
                    dial += 100;
                }

                if dial == 0 {
                    result_p2 += 1;
                }
            }
        } else {
            for _ in 0..n {
                dial += 1;

                if dial > 99 {
                    dial -= 100;
                }

                if dial == 0 {
                    result_p2 += 1;
                }
            }
        }

        if dial == 0 {
            result += 1;
        }
    }

    println!("Part 1: {result}");
    println!("Part 2: {result_p2}");
}
