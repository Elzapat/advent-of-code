#[derive(Debug, Copy, Clone)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

fn compute_energized_cells(grid: Vec<Vec<char>>, start_beam: (i32, i32, Dir)) -> usize {
    let mut beams = vec![start_beam];
    let mut energized: Vec<Vec<bool>> = grid
        .iter()
        .map(|r| r.iter().map(|_| false).collect())
        .collect();

    let mut no_energized = 0;

    while !beams.is_empty() {
        // std::thread::sleep_ms(500);

        let mut new_beams = Vec::new();

        let mut has_energized = false;

        for (bx, by, dir) in &mut beams {
            let (bx, by) = (*bx as usize, *by as usize);
            if !energized[by][bx] {
                energized[by][bx] = true;
                has_energized = true;
            }

            match grid[by][bx] {
                '/' => match dir {
                    Dir::Right => *dir = Dir::Up,
                    Dir::Left => *dir = Dir::Down,
                    Dir::Up => *dir = Dir::Right,
                    Dir::Down => *dir = Dir::Left,
                },
                '\\' => match dir {
                    Dir::Right => *dir = Dir::Down,
                    Dir::Left => *dir = Dir::Up,
                    Dir::Up => *dir = Dir::Left,
                    Dir::Down => *dir = Dir::Right,
                },
                '-' => match dir {
                    Dir::Down | Dir::Up => {
                        *dir = Dir::Right;
                        new_beams.push((bx as i32, by as i32, Dir::Left));
                    }
                    _ => {}
                },
                '|' => match dir {
                    Dir::Right | Dir::Left => {
                        *dir = Dir::Up;
                        new_beams.push((bx as i32, by as i32, Dir::Down));
                    }
                    _ => {}
                },
                '.' => {}
                _ => unreachable!(),
            }
        }

        if has_energized {
            no_energized = 0;
        } else {
            no_energized += 1;
        }

        if no_energized >= 5 {
            break;
        }

        beams.append(&mut new_beams);

        for (bx, by, dir) in &mut beams {
            match *dir {
                Dir::Right => *bx += 1,
                Dir::Left => *bx -= 1,
                Dir::Up => *by -= 1,
                Dir::Down => *by += 1,
            }
        }

        beams.retain(|(bx, by, _)| {
            *bx >= 0 && *by >= 0 && *by < grid.len() as i32 && *bx < grid[0].len() as i32
        });
    }

    energized
        .iter()
        .map(|r| r.iter().filter(|&c| *c).count())
        .sum::<usize>()
}

fn main() {
    let grid = aoc_base::read_input_grid();

    println!(
        "Part 1: {}",
        compute_energized_cells(grid.clone(), (0, 0, Dir::Right))
    );

    let mut max_energized = 0;

    for start_y in 0..grid.len() {
        let res1 = compute_energized_cells(grid.clone(), (0, start_y as i32, Dir::Right));
        let res2 = compute_energized_cells(
            grid.clone(),
            (grid[0].len() as i32 - 1, start_y as i32, Dir::Left),
        );

        if res1 > max_energized {
            max_energized = res1;
        }

        if res2 > max_energized {
            max_energized = res2;
        }
    }

    for start_x in 0..grid.len() {
        let res1 = compute_energized_cells(grid.clone(), (start_x as i32, 0, Dir::Down));
        let res2 = compute_energized_cells(
            grid.clone(),
            (start_x as i32, grid.len() as i32 - 1, Dir::Up),
        );

        if res1 > max_energized {
            max_energized = res1;
        }

        if res2 > max_energized {
            max_energized = res2;
        }
    }

    println!("Part 2: {max_energized}");
}
