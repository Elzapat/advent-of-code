fn main() {
    part1();

    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>();
    let mut grid = vec![];

    for line in &lines {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    let mut result = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            let mut total_score = 1;
            // Left
            {
                let mut score = 0;
                let mut index = j;
                while index > 0 {
                    score += 1;

                    if row[index - 1] >= *tree {
                        break;
                    }

                    index -= 1;
                }

                total_score *= score;
            }

            // Right
            {
                let mut score = 0;
                let mut index = j;
                while index < row.len() - 1 {
                    score += 1;

                    if row[index + 1] >= *tree {
                        break;
                    }

                    index += 1;
                }

                total_score *= score;
            }

            // Up
            {
                let mut score = 0;
                let mut index = i;
                while index > 0 {
                    score += 1;

                    if grid[index - 1][j] >= *tree {
                        break;
                    }

                    index -= 1;
                }

                total_score *= score;
            }

            // Down
            {
                let mut score = 0;
                let mut index = i;
                while index < grid.len() - 1 {
                    score += 1;

                    if grid[index + 1][j] >= *tree {
                        break;
                    }

                    index += 1;
                }

                total_score *= score;
            }

            if total_score > result {
                result = total_score;
            }
        }
    }

    println!("Part 2: {result}");
}

fn part1() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>();
    let mut grid = vec![];

    for line in &lines {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    let mut result = 0;

    for (i, row) in grid.iter().enumerate() {
        'row_loop: for (j, tree) in row.iter().enumerate() {
            let mut visible = true;
            // Left
            {
                let mut index = j;
                let mut last = tree;
                while index > 0 {
                    if row[index - 1] >= *last && row[index - 1] >= *tree {
                        visible = false;
                        break;
                    }

                    last = &row[index - 1];
                    index -= 1;
                }

                if visible {
                    result += 1;
                    continue 'row_loop;
                }
            }

            visible = true;
            // Right
            {
                let mut index = j;
                let mut last = tree;
                while index < row.len() - 1 {
                    if row[index + 1] >= *last && row[index + 1] >= *tree {
                        visible = false;
                        break;
                    }

                    last = &row[index + 1];
                    index += 1;
                }

                if visible {
                    result += 1;
                    continue 'row_loop;
                }
            }

            visible = true;
            // Up
            {
                let mut index = i;
                let mut last = tree;
                while index > 0 {
                    if grid[index - 1][j] >= *last && grid[index - 1][j] >= *tree {
                        visible = false;
                        break;
                    }

                    last = &grid[index - 1][j];
                    index -= 1;
                }

                if visible {
                    result += 1;
                    continue 'row_loop;
                }
            }

            visible = true;
            // Down
            {
                let mut index = i;
                let mut last = tree;
                while index < grid.len() - 1 {
                    if grid[index + 1][j] >= *last && grid[index + 1][j] >= *tree {
                        visible = false;
                        break;
                    }

                    last = &grid[index + 1][j];
                    index += 1;
                }

                if visible {
                    result += 1;
                    continue 'row_loop;
                }
            }
        }
    }

    println!("Part 1: {result}");
}
