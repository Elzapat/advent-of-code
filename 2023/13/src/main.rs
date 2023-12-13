fn find_symmetry_lines(patterns: Vec<Vec<String>>, part1: bool) -> usize {
    let mut result = 0;

    'main: for og_pattern in patterns {
        let mut symmetry_found: Option<(bool, usize)> = None;

        for i in 0..og_pattern.len() - 1 {
            let mut symmetric_horizontal = true;

            for (before, after) in (0..=i).rev().zip(i + 1..og_pattern.len()) {
                symmetric_horizontal &= og_pattern[before] == og_pattern[after];
            }

            if symmetric_horizontal {
                symmetry_found = Some((false, i + 1));
                break;
            }
        }

        for i in 0..og_pattern[0].len() - 1 {
            let mut symmetric_vertical = true;

            for (before, after) in (0..=i).rev().zip(i + 1..og_pattern[0].len()) {
                let up = og_pattern
                    .iter()
                    .map(|l| l.chars().nth(before).unwrap())
                    .collect::<Vec<char>>();
                let down = og_pattern
                    .iter()
                    .map(|l| l.chars().nth(after).unwrap())
                    .collect::<Vec<char>>();

                symmetric_vertical &= up == down;
            }

            if symmetric_vertical {
                symmetry_found = Some((true, i + 1));
            }
        }

        for replace_y in 0..og_pattern.len() {
            for replace_x in 0..og_pattern[replace_y].len() {
                let mut pattern = og_pattern.clone();
                if !part1 {
                    let replace_char = &mut pattern[replace_y].chars().nth(replace_x).unwrap();
                    pattern[replace_y].replace_range(
                        replace_x..replace_x + 1,
                        if *replace_char == '#' { "." } else { "#" },
                    );
                }

                for i in 0..pattern.len() - 1 {
                    let mut symmetric_horizontal = true;

                    for (before, after) in (0..=i).rev().zip(i + 1..pattern.len()) {
                        symmetric_horizontal &= pattern[before] == pattern[after];
                    }

                    if symmetric_horizontal {
                        if part1 {
                            result += (i + 1) * 100;
                            continue 'main;
                        } else if let Some((is_vertical, line_idx)) = symmetry_found {
                            if is_vertical || i + 1 != line_idx {
                                result += (i + 1) * 100;
                                continue 'main;
                            }
                        } else {
                            symmetry_found = Some((false, i + 1));
                        }
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
                        if part1 {
                            result += i + 1;
                            continue 'main;
                        } else if let Some((is_vertical, line_idx)) = symmetry_found {
                            if !is_vertical || i + 1 != line_idx {
                                result += i + 1;
                                continue 'main;
                            }
                        } else {
                            symmetry_found = Some((true, i + 1));
                        }
                    }
                }

                if part1 {
                    continue 'main;
                }
            }
        }
    }

    result
}

fn main() {
    let input = aoc_base::read_input();
    let mut patterns = Vec::new();

    let mut current_pattern = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(current_pattern);
            current_pattern = Vec::new();
        } else {
            current_pattern.push(line.to_string());
        }
    }

    patterns.push(current_pattern);

    println!("Part 1: {}", find_symmetry_lines(patterns.clone(), true));
    println!("Part 2: {}", find_symmetry_lines(patterns, false));
}
