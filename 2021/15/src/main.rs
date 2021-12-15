use pathfinding::prelude::dijkstra;

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let grid = file
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    {
        let grid = grid.clone();

        let result = dijkstra(
            &(0, 0),
            |&(x, y)| {
                let mut successors = vec![];

                if x > 0 {
                    successors.push(((x - 1, y), grid[x - 1][y]));
                }

                if y > 0 {
                    successors.push(((x, y - 1), grid[x][y - 1]));
                }

                if x < grid.len() - 1 {
                    successors.push(((x + 1, y), grid[x + 1][y]));
                }

                if y < grid[x].len() - 1 {
                    successors.push(((x, y + 1), grid[x][y + 1]));
                }

                successors
            },
            |&(x, y)| x == grid.len() - 1 && y == grid[0].len() - 1,
        );

        println!("Part 1: {}", result.unwrap().1);
    }

    {
        let mut grid = grid.clone();

        for row in grid.iter_mut() {
            let orig_row = row.clone();
            for n in 1..=4 {
                row.append(&mut orig_row.clone().iter().map(|x| if *x + n > 9 { (x + n) - 9 } else { *x + n }).collect::<Vec<_>>());
            }
        }

        let orig_len = grid.len();
        for n in 1..=4 {
            for i in 0..orig_len {
                grid.push(grid[i].clone().iter().map(|x| if *x + n > 9 { (x + n) - 9 } else { *x + n }).collect::<Vec<_>>());
            }
        }

        let result = dijkstra(
            &(0, 0),
            |&(x, y)| {
                let mut successors = vec![];

                if x > 0 {
                    successors.push(((x - 1, y), grid[x - 1][y]));
                }

                if y > 0 {
                    successors.push(((x, y - 1), grid[x][y - 1]));
                }

                if x < grid.len() - 1 {
                    successors.push(((x + 1, y), grid[x + 1][y]));
                }

                if y < grid[x].len() - 1 {
                    successors.push(((x, y + 1), grid[x][y + 1]));
                }

                successors
            },
            |&(x, y)| x == grid.len() - 1 && y == grid[0].len() - 1,
        );

        println!("Part 2: {}", result.unwrap().1);
    }
}
