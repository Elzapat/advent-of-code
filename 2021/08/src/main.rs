fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let input = file.lines().map(|line| line.split("|"));
    let patterns = input.clone().map(|mut line| line.nth(0).unwrap().split_whitespace().collect::<Vec<&str>>()).collect::<Vec<_>>();
    let outputs = input.map(|mut line| line.nth(1).unwrap().split_whitespace().collect::<Vec<&str>>()).collect::<Vec<_>>();

    {
        let outputs = outputs.clone();
        let mut occ = 0;

        for outputs_line in &outputs {
            for output in outputs_line {
                if output.len() == 2 || output.len() == 3 || output.len() == 4 || output.len() == 7 {
                    occ += 1;
                }
            }
        }

        println!("Part 1: {}", occ);
    }

    {
        let mut outputs_sum = 0;

        for i in 0..patterns.len() {
            let patterns = &patterns[i];
            let outputs = &outputs[i];

            let mut segments = [""; 10];

            for pattern in patterns {
                match pattern.len() {
                    2 => segments[1] = pattern,
                    3 => segments[7] = pattern,
                    4 => segments[4] = pattern,
                    7 => segments[8] = pattern,
                    _ => (),
                }
            }

            for pattern in patterns {
                match pattern.len() {
                    5 => {
                        if n_match(pattern, segments[1]) == 1 &&
                            n_match(pattern, segments[4]) == 2 &&
                            n_match(pattern, segments[7]) == 2
                        {
                            segments[2] = pattern;
                        } else if n_match(pattern, segments[1]) == 2 &&
                            n_match(pattern, segments[4]) == 3 &&
                            n_match(pattern, segments[7]) == 3
                        {
                            segments[3] = pattern;
                        } else if n_match(pattern, segments[1]) == 1 &&
                            n_match(pattern, segments[4]) == 3 &&
                            n_match(pattern, segments[7]) == 2
                        {
                            segments[5] = pattern;
                        }
                    },
                    6 => {
                        if n_match(pattern, segments[1]) == 2 &&
                            n_match(pattern, segments[4]) == 3 &&
                            n_match(pattern, segments[7]) == 3
                        {
                            segments[0] = pattern;
                        } else if n_match(pattern, segments[1]) == 1 &&
                            n_match(pattern, segments[4]) == 3 &&
                            n_match(pattern, segments[7]) == 2
                        {
                            segments[6] = pattern;
                        } else if n_match(pattern, segments[1]) == 2 &&
                            n_match(pattern, segments[4]) == 4 &&
                            n_match(pattern, segments[7]) == 3
                        {
                            segments[9] = pattern;
                        }
                    }
                    _ => ()
                }
            }

            let mut output_number = String::new();

            for output in outputs {
                for (i, seg) in segments.iter().enumerate() {
                    let mut seg = seg.chars().collect::<Vec<char>>();
                    seg.sort_by(|a, b| b.cmp(a));
                    let seg = seg.into_iter().collect::<String>();

                    let mut output = output.chars().collect::<Vec<char>>();
                    output.sort_by(|a, b| b.cmp(a));
                    let output = output.into_iter().collect::<String>();

                    if seg == output {
                        output_number.push_str(&i.to_string());
                    }
                }
            }

            outputs_sum += output_number.parse::<u32>().unwrap();
        }

        println!("Part 2: {}", outputs_sum);
    }
}

fn n_match(str1: &str, str2: &str) -> usize {
    let mut n = 0;

    for c1 in str1.chars() {
        for c2 in str2.chars() {
            if c1 == c2 {
                n += 1;
            }
        }
    }

    n
}
