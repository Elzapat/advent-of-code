use itertools::Itertools;

fn find_valid_combinations(mut springs_line: String, groups: Vec<usize>) -> u64 {
    let mut result = 0;

    let unknown_parts_indices = springs_line
        .chars()
        .enumerate()
        .map(|(i, c)| if c == '?' { Some(i) } else { None })
        .filter(|i| i.is_some())
        .flatten()
        .collect::<Vec<usize>>();

    for combinations in 0..2_u64.pow(unknown_parts_indices.len() as u32) {
        for (i, &index) in unknown_parts_indices.iter().enumerate() {
            springs_line.replace_range(
                index..index + 1,
                if (combinations >> i) & 1 == 1 {
                    "#"
                } else {
                    "."
                },
            );
        }

        if is_line_valid(&springs_line, &groups) {
            result += 1;
        }
    }

    result
}

fn is_line_valid(line: &str, springs_groups: &[usize]) -> bool {
    let mut is_valid = true;

    let groups = line
        .chars()
        .group_by(|c| *c == '#')
        .into_iter()
        .filter(|(k, _)| *k)
        .map(|(_, g)| g.collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    if groups.len() != springs_groups.len() {
        return false;
    };

    for (i, group) in groups.iter().enumerate() {
        if group.len() != springs_groups[i] {
            is_valid = false;
        }
    }

    is_valid
}

fn main() {
    let input = aoc_base::read_input();
    let mut result_p1 = 0;
    let mut result_p2 = 0;

    for (i, line) in input.lines().enumerate() {
        println!("{i}");

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

        for _ in 0..5 {
            springs_line_p2.push('?');
            springs_line_p2.push_str(&springs_line);

            groups_p2.append(&mut groups.clone());
        }

        result_p1 += find_valid_combinations(springs_line, groups);
        result_p2 += find_valid_combinations(springs_line_p2, groups_p2);
    }

    // 6964: too low
    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}
