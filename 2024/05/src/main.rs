fn main() {
    let input = aoc_base::read_input();

    let mut ordering = vec![];

    let mut parsing_rules = false;

    let mut result_p1 = 0;
    let mut result_p2 = 0;

    for line in input.lines() {
        if line.is_empty() {
            parsing_rules = true;
            continue;
        }

        if !parsing_rules {
            let mut parts = line.split('|');
            let n1 = parts.next().unwrap().parse::<u32>().unwrap();
            let n2 = parts.next().unwrap().parse::<u32>().unwrap();

            ordering.push((n1, n2));
        } else {
            let mut numbers: Vec<u32> =
                line.split(',').map(|n| n.parse::<u32>().unwrap()).collect();

            if is_valid(&numbers, &ordering) {
                result_p1 += numbers[numbers.len() / 2];
            } else {
                while !is_valid(&numbers, &ordering) {
                    for (n1, n2) in &ordering {
                        let p1 = numbers.iter().position(|n| n == n1);
                        let p2 = numbers.iter().position(|n| n == n2);

                        if let Some(p1) = p1 {
                            if let Some(p2) = p2 {
                                if p1 > p2 {
                                    numbers.swap(p1, p2);
                                }
                            }
                        }
                    }
                }

                result_p2 += numbers[numbers.len() / 2];
            }
        }
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}

fn is_valid(numbers: &[u32], ordering: &[(u32, u32)]) -> bool {
    let mut valid = true;

    for (n1, n2) in ordering {
        let p1 = numbers.iter().position(|n| n == n1);
        let p2 = numbers.iter().position(|n| n == n2);

        if p1.is_none() || p2.is_none() {
            continue;
        } else {
            valid &= p1.unwrap() <= p2.unwrap();
        }
    }

    valid
}
