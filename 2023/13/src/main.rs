fn main() {
    let input = aoc_base::read_input();
    let mut patterns = Vec::new();

    let mut current_pattern = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(current_pattern);
            current_pattern = Vec::new();
        } else {
            current_pattern.push(line);
        }
    }

    patterns.push(current_pattern);

    let mut result_p1 = 0;

    for pattern in patterns {
        // for replace_y in 0..pattern.len() {
        //     for replace_x in 0..pattern[replace_y].len() {
        //         let replace_char = pattern[replace_y].chars().nth(replace_x).unwrap();
        //         pattern[replace_y].to_string().replace_range(
        //             replace_x..replace_x + 1,
        //             if replace_char == '#' { "." } else { "#" },
        //         )
        //     }
        // }

        for i in 0..pattern.len() - 1 {
            let mut symmetric_horizontal = true;

            for (before, after) in (0..=i).rev().zip(i + 1..pattern.len()) {
                symmetric_horizontal &= pattern[before] == pattern[after];
            }

            if symmetric_horizontal {
                result_p1 += (i + 1) * 100;
            }
        }

        for i in 0..pattern[0].len() - 1 {
            let mut symmetric_vertical = true;

            for (before, after) in (0..=i).rev().zip(i + 1..pattern[0].len()) {
                let up = pattern
                    .iter()
                    .map(|l| l.chars().nth(before).unwrap())
                    .collect::<Vec<char>>();
                let down = pattern
                    .iter()
                    .map(|l| l.chars().nth(after).unwrap())
                    .collect::<Vec<char>>();

                symmetric_vertical &= up == down;
            }

            if symmetric_vertical {
                result_p1 += i + 1;
            }
        }
    }

    println!("Part 1: {result_p1}");
}
