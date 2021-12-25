fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let grid = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    {
        let mut grid = grid.clone();
        let mut moves = u32::MAX;
        let mut steps = 0;

        while moves > 0 {
            moves = 0;
            steps += 1;

            let grid_start = grid.clone();
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid_start[i][j] == '>' {
                        if j + 1 >= grid[i].len() && grid_start[i][0] == '.' {
                            grid[i][0] = '>';
                            grid[i][j] = '.';
                            moves += 1;
                        } else if j + 1 < grid[i].len() && grid_start[i][j + 1] == '.' {
                            grid[i][j + 1] = '>';
                            grid[i][j] = '.';
                            moves += 1;
                        }
                    }
                }
            }

            let grid_start = grid.clone();
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid_start[i][j] == 'v' {
                        if i + 1 >= grid.len() && grid_start[0][j] == '.' {
                            grid[0][j] = 'v';
                            grid[i][j] = '.';
                            moves += 1;
                        } else if i + 1 < grid.len() && grid_start[i + 1][j] == '.' {
                            grid[i + 1][j] = 'v';
                            grid[i][j] = '.';
                            moves += 1;
                        }
                    }
                }
            }
        }

        println!("Part 1: {}", steps);
    }
}
