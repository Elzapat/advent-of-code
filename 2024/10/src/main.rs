use std::collections::HashSet;

fn main() {
    let grid = aoc_base::read_input_grid_u64();
    let mut starts = vec![];
    let mut result = 0;
    let mut result_p2 = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 0 {
                starts.push((r, c));
            }
        }
    }

    for start in &starts {
        let mut trails = vec![];
        let mut nines_reached = HashSet::new();

        explore(&grid, *start, &mut trails, vec![], &mut nines_reached);

        result += nines_reached.len();
        result_p2 += trails.len();
    }

    println!("Part 1: {result}");
    println!("Part 2: {result_p2}");
}

fn explore(
    grid: &Vec<Vec<u64>>,
    pos: (usize, usize),
    trails: &mut Vec<Vec<(usize, usize)>>,
    mut current_trail: Vec<(usize, usize)>,
    nines_reached: &mut HashSet<(usize, usize)>,
) {
    if current_trail.contains(&pos) {
        return;
    }

    current_trail.push(pos);

    if grid[pos.0][pos.1] == 9 {
        trails.push(current_trail);
        nines_reached.insert(pos);
        return;
    }

    let current_height = grid[pos.0][pos.1];

    if pos.0 < grid.len() - 1 && grid[pos.0 + 1][pos.1] == current_height + 1 {
        explore(
            grid,
            (pos.0 + 1, pos.1),
            trails,
            current_trail.clone(),
            nines_reached,
        );
    }
    if pos.1 < grid.len() - 1 && grid[pos.0][pos.1 + 1] == current_height + 1 {
        explore(
            grid,
            (pos.0, pos.1 + 1),
            trails,
            current_trail.clone(),
            nines_reached,
        );
    }
    if pos.1 > 0 && grid[pos.0][pos.1 - 1] == current_height + 1 {
        explore(
            grid,
            (pos.0, pos.1 - 1),
            trails,
            current_trail.clone(),
            nines_reached,
        );
    }
    if pos.0 > 0 && grid[pos.0 - 1][pos.1] == current_height + 1 {
        explore(
            grid,
            (pos.0 - 1, pos.1),
            trails,
            current_trail,
            nines_reached,
        );
    }
}
