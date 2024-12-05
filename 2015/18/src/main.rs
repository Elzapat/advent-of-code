fn main() {
    let mut grid = aoc_base::read_input_grid_char();
    let row_len = grid[0].len();
    let col_len = grid.len();
    grid[0][0] = '#';
    grid[0][row_len - 1] = '#';
    grid[col_len - 1][0] = '#';
    grid[col_len - 1][row_len - 1] = '#';

    let mut next_grid = grid.clone();

    for _ in 0..100 {
        for y in 0..grid.len() as isize {
            for x in 0..grid[0].len() as isize {
                let mut n_on = 0;

                if (y == 0 && x == 0)
                    || (y == 0 && x == row_len as isize - 1)
                    || (y == col_len as isize - 1 && x == 0)
                    || (y == col_len as isize - 1 && x == row_len as isize - 1)
                {
                    continue;
                }

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }

                        if y + dy < 0
                            || y + dy > grid.len() as isize - 1
                            || x + dx < 0
                            || x + dx > grid[0].len() as isize - 1
                        {
                            continue;
                        }

                        if grid[(y + dy) as usize][(x + dx) as usize] == '#' {
                            n_on += 1;
                        }
                    }
                }

                let x = x as usize;
                let y = y as usize;

                if grid[y][x] == '#' && (n_on < 2 || n_on > 3) {
                    next_grid[y][x] = '.';
                } else if grid[y][x] == '.' && n_on == 3 {
                    next_grid[y][x] = '#';
                }
            }
        }

        grid = next_grid.clone();
    }

    println!(
        "Part 2: {}",
        grid.iter()
            .map(|r| r.iter().map(|&l| (l == '#') as u32).sum::<u32>())
            .sum::<u32>()
    );
}
