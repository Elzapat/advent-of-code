fn main() {
    let grid = aoc_base::read_input_grid_char();

    let mut result_p1 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'X' {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        if find_xmas(&grid, (x as isize + dx, y as isize + dy), 'M')
                            && find_xmas(&grid, (x as isize + 2 * dx, y as isize + 2 * dy), 'A')
                            && find_xmas(&grid, (x as isize + 3 * dx, y as isize + 3 * dy), 'S')
                        {
                            result_p1 += 1;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {result_p1}");

    let mut result_p2 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'A'
                && ((find_xmas(&grid, (x as isize + 1, y as isize + 1), 'M')
                    && find_xmas(&grid, (x as isize - 1, y as isize - 1), 'S'))
                    || (find_xmas(&grid, (x as isize + 1, y as isize + 1), 'S')
                        && find_xmas(&grid, (x as isize - 1, y as isize - 1), 'M')))
                && ((find_xmas(&grid, (x as isize + 1, y as isize - 1), 'M')
                    && find_xmas(&grid, (x as isize - 1, y as isize + 1), 'S'))
                    || (find_xmas(&grid, (x as isize + 1, y as isize - 1), 'S')
                        && find_xmas(&grid, (x as isize - 1, y as isize + 1), 'M')))
            {
                result_p2 += 1;
            }
        }
    }

    println!("Part 2: {result_p2}");
}

fn find_xmas(grid: &[Vec<char>], pos: (isize, isize), letter: char) -> bool {
    if pos.1 < 0
        || pos.1 > grid.len() as isize - 1
        || pos.0 < 0
        || pos.0 > grid[pos.1 as usize].len() as isize - 1
    {
        return false;
    }

    grid[(pos.1) as usize][(pos.0) as usize] == letter
}
