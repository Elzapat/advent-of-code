use std::collections::HashSet;

fn main() {
    let grid = aoc_base::read_input_grid_char();
    let mut pos = (0, 0);

    'main: for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '^' {
                pos = (r as isize, c as isize);
                break 'main;
            }
        }
    }

    let visited = find_path(&grid, pos);

    println!("Part 1: {}", visited.unwrap().len());

    let mut nb_loop = 0;

    for r in 0..grid.len() {
        for c in 0..grid.len() {
            let mut new_grid = grid.clone();

            if new_grid[r][c] == '.' {
                new_grid[r][c] = '#';

                if find_path(&new_grid, pos).is_none() {
                    nb_loop += 1;
                }
            }
        }
    }

    println!("Part 2: {nb_loop}");
}

fn find_path(grid: &Vec<Vec<char>>, mut pos: (isize, isize)) -> Option<HashSet<(isize, isize)>> {
    let mut visited = HashSet::new();
    let mut facing = (-1, 0);
    let mut pos_not_added = 0;

    while pos.0 > 0
        && pos.0 < grid.len() as isize - 1
        && pos.1 > 0
        && pos.1 < grid[0].len() as isize - 1
    {
        if visited.insert(pos) {
            pos_not_added = 0;
        } else {
            pos_not_added += 1;
        }

        // stuck in a loop
        if pos_not_added > 100 {
            return None;
        }

        while grid[(pos.0 + facing.0) as usize][(pos.1 + facing.1) as usize] == '#' {
            if facing == (-1, 0) {
                facing = (0, 1)
            } else if facing == (0, 1) {
                facing = (1, 0);
            } else if facing == (1, 0) {
                facing = (0, -1);
            } else if facing == (0, -1) {
                facing = (-1, 0);
            }
        }

        pos.0 += facing.0;
        pos.1 += facing.1;
    }

    visited.insert(pos);

    Some(visited)
}
