use std::collections::{HashMap, HashSet};

fn main() {
    let grid = aoc_base::read_input_grid_char();
    let mut antennas = HashMap::new();
    let mut antinode_pos_p1 = HashSet::new();
    let mut antinode_pos_p2 = HashSet::new();

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] != '.' {
                antennas.entry(grid[r][c]).or_insert(vec![]).push((r, c));
            }
        }
    }

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                let a1 = positions[i];
                let a2 = positions[j];

                if a1 == a2 {
                    continue;
                }

                for r in 0..grid.len() {
                    for c in 0..grid[r].len() {
                        let pos = (r, c);

                        if (distance(pos, a1) == 2 * distance(pos, a2)
                            || 2 * distance(pos, a1) == distance(pos, a2))
                            && ((a1.0 as i32 - pos.0 as i32) * (a2.1 as i32 - pos.1 as i32)
                                - (a1.1 as i32 - pos.1 as i32) * (a2.0 as i32 - pos.0 as i32))
                                .abs()
                                == 0
                        {
                            antinode_pos_p1.insert(pos);
                        }

                        if ((a1.0 as i32 - pos.0 as i32) * (a2.1 as i32 - pos.1 as i32)
                            - (a1.1 as i32 - pos.1 as i32) * (a2.0 as i32 - pos.0 as i32))
                            .abs()
                            == 0
                        {
                            antinode_pos_p2.insert(pos);
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", antinode_pos_p1.len());
    println!("Part 2: {}", antinode_pos_p2.len());
}

fn distance(pos1: (usize, usize), pos2: (usize, usize)) -> i32 {
    (pos1.1 as i32 - pos2.1 as i32).abs() + (pos1.0 as i32 - pos2.0 as i32).abs()
}
