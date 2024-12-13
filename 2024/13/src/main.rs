fn main() {
    let input = aoc_base::read_input()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let mut result_p1 = 0;
    let mut result_p2 = 0;

    let mut i = 0;
    while i < input.len() {
        let machine = input[i..i + 3].join("\n");

        let re = regex::Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .unwrap();
        let captures = re.captures(&machine).unwrap();

        let buttona = (
            captures[1].parse::<u64>().unwrap(),
            captures[2].parse::<u64>().unwrap(),
        );
        let buttonb = (
            captures[3].parse::<u64>().unwrap(),
            captures[4].parse::<u64>().unwrap(),
        );
        let mut goal = (
            captures[5].parse::<u64>().unwrap(),
            captures[6].parse::<u64>().unwrap(),
        );

        // Part 1
        {
            let mut min_token = u64::MAX;

            for apress in 0..100_u64 {
                for bpress in 0..100_u64 {
                    let claw_pos = (
                        apress * buttona.0 + bpress * buttonb.0,
                        apress * buttona.1 + bpress * buttonb.1,
                    );

                    if claw_pos == goal {
                        min_token = min_token.min(3 * apress + bpress);
                    }
                }
            }

            if min_token != u64::MAX {
                result_p1 += min_token;
            }
        }

        // Part 2
        {
            goal.0 += 10000000000000;
            goal.1 += 10000000000000;

            let buttona = (buttona.0 as i64, buttona.1 as i64);
            let buttonb = (buttonb.0 as i64, buttonb.1 as i64);
            let goal = (goal.0 as i64, goal.1 as i64);

            let apress = (goal.0 * buttonb.1 - buttonb.0 * goal.1)
                / (buttona.0 * buttonb.1 - buttonb.0 * buttona.1);

            let bpress = (goal.1 * buttona.0 - buttona.1 * goal.0)
                / (buttona.0 * buttonb.1 - buttonb.0 * buttona.1);

            if apress >= 0
                && bpress >= 0
                && buttona.0 * apress + buttonb.0 * bpress == goal.0
                && buttona.1 * apress + buttonb.1 * bpress == goal.1
            {
                result_p2 += apress * 3 + bpress;
            }
        }

        i += 4;
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}
