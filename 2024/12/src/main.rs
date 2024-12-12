use std::collections::HashSet;

type Pos = (usize, usize);

fn main() {
    let grid = aoc_base::read_input_grid_char();
    let mut regions = vec![];
    let mut result_p1 = 0;
    let mut result_p2 = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if regions
                .iter()
                .any(|region: &HashSet<Pos>| region.contains(&(r, c)))
            {
                continue;
            }

            let region = find_region(&grid, (r, c), HashSet::new());

            result_p1 += region.len() * compute_perimeter(&grid, &region);
            result_p2 += region.len() * compute_sides(&region);

            regions.push(region);
        }
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}

fn find_region(
    grid: &Vec<Vec<char>>,
    current_pos: Pos,
    mut current_region: HashSet<Pos>,
) -> HashSet<Pos> {
    current_region.insert(current_pos);
    let mut neighbours = get_neighbours(current_pos, grid.len(), grid[0].len(), false);

    if neighbours.iter().all(|n| current_region.contains(n)) {
        return current_region;
    }

    neighbours.retain(|n| !current_region.contains(n));

    for n in &neighbours {
        if grid[n.0][n.1] == grid[current_pos.0][current_pos.1] {
            current_region.extend(find_region(grid, *n, current_region.clone()));
        }
    }

    current_region
}

fn compute_sides(region: &HashSet<Pos>) -> usize {
    let mut sides = 0;
    let region = region
        .iter()
        .map(|p| (p.0 as isize, p.1 as isize))
        .collect::<Vec<(isize, isize)>>();

    for p in &region {
        sides += (!region.contains(&(p.0 - 1, p.1)) && !region.contains(&(p.0, p.1 - 1))) as usize;
        sides += (!region.contains(&(p.0 + 1, p.1)) && !region.contains(&(p.0, p.1 - 1))) as usize;
        sides += (!region.contains(&(p.0 - 1, p.1)) && !region.contains(&(p.0, p.1 + 1))) as usize;
        sides += (!region.contains(&(p.0 + 1, p.1)) && !region.contains(&(p.0, p.1 + 1))) as usize;

        sides += (region.contains(&(p.0 - 1, p.1))
            && region.contains(&(p.0, p.1 - 1))
            && !region.contains(&(p.0 - 1, p.1 - 1))) as usize;
        sides += (region.contains(&(p.0 + 1, p.1))
            && region.contains(&(p.0, p.1 - 1))
            && !region.contains(&(p.0 + 1, p.1 - 1))) as usize;
        sides += (region.contains(&(p.0 - 1, p.1))
            && region.contains(&(p.0, p.1 + 1))
            && !region.contains(&(p.0 - 1, p.1 + 1))) as usize;
        sides += (region.contains(&(p.0 + 1, p.1))
            && region.contains(&(p.0, p.1 + 1))
            && !region.contains(&(p.0 + 1, p.1 + 1))) as usize;
    }

    sides
}

fn compute_perimeter(grid: &Vec<Vec<char>>, region: &HashSet<Pos>) -> usize {
    let mut perimeter = 0;

    for p in region {
        perimeter += get_neighbours(*p, grid.len(), grid[0].len(), false)
            .iter()
            .map(|n| (grid[n.0][n.1] != grid[p.0][p.1]) as usize)
            .sum::<usize>();

        perimeter += (p.0 == 0) as usize;
        perimeter += (p.0 == grid.len() - 1) as usize;
        perimeter += (p.1 == 0) as usize;
        perimeter += (p.1 == grid[0].len() - 1) as usize;
    }

    perimeter
}

fn get_neighbours(pos: Pos, r_bound: usize, c_bound: usize, corners: bool) -> Vec<Pos> {
    let mut n = vec![];

    for dr in -1..=1 {
        for dc in -1..=1 {
            if (dr == 0 && dc == 0) || (!corners && dr != 0 && dc != 0) {
                continue;
            }

            if pos.0 as isize + dr >= 0
                && pos.1 as isize + dc >= 0
                && pos.0 as isize + dr < r_bound as isize
                && pos.1 as isize + dc < c_bound as isize
            {
                n.push((
                    (pos.0 as isize + dr) as usize,
                    (pos.1 as isize + dc) as usize,
                ))
            }
        }
    }

    n
}
