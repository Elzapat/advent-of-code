use memoize::memoize;

#[memoize]
fn compute_arrangements(line: String, springs_groups: Vec<usize>) -> u64 {
    if springs_groups.is_empty() {
        if !line.contains('#') {
            return 1;
        } else {
            return 0;
        }
    }

    if line == "" {
        return 0;
    }

    let next_character = line.chars().nth(0).unwrap();
    let next_group = springs_groups[0];

    let empty = || compute_arrangements(line[1..].to_string(), springs_groups.clone());

    let spring = || {
        if next_group > line.len() {
            return 0;
        }

        let current_group = line[..next_group].to_string().replace('?', "#");

        if current_group != "#".repeat(next_group) {
            return 0;
        }

        if line.len() == next_group {
            if springs_groups.len() == 1 {
                return 1;
            } else {
                return 0;
            }
        }

        if "?.".contains(line.chars().nth(next_group).unwrap()) {
            return compute_arrangements(
                line[next_group + 1..].to_string(),
                springs_groups[1..].to_vec(),
            );
        }

        return 0;
    };

    match next_character {
        '.' => empty(),
        '#' => spring(),
        '?' => empty() + spring(),
        _ => unreachable!(),
    }
}

fn main() {
    let input = aoc_base::read_input();
    let mut result_p1 = 0;
    let mut result_p2 = 0;

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        let springs_line = parts.next().unwrap().to_string();
        let groups = parts
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut springs_line_p2 = springs_line.clone();
        let mut groups_p2 = groups.clone();

        for _ in 0..4 {
            springs_line_p2.push('?');
            springs_line_p2.push_str(&springs_line);

            groups_p2.append(&mut groups.clone());
        }

        result_p1 += compute_arrangements(springs_line, groups);
        result_p2 += compute_arrangements(springs_line_p2, groups_p2);
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}
