use std::collections::HashSet;

fn main() {
    let input = aoc_base::read_input();
    let mut santa_pos = (0, 0);
    let mut robot_pos = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(santa_pos);

    for (i, dir) in input.trim().chars().enumerate() {
        let pos = if i % 2 == 0 {
            &mut santa_pos
        } else {
            &mut robot_pos
        };

        match dir {
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => unreachable!()
        }

        visited.insert(santa_pos);
        visited.insert(robot_pos);
    }

    println!("Part 2: {}", visited.len());
}
