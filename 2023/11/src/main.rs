use itertools::Itertools;

fn find_distances(space: Vec<Vec<char>>, expand_by: i64) -> i64 {
    let mut galaxies = space
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &p)| p == '#')
                .map(|(x, _)| (x as i64, y as i64))
                .collect::<Vec<(i64, i64)>>()
        })
        .collect::<Vec<(i64, i64)>>();

    // Expand lines
    for (i, line) in space.iter().enumerate().rev() {
        if !line.contains(&'#') {
            for galaxy in &mut galaxies {
                if galaxy.1 > i as i64 {
                    galaxy.1 += expand_by;
                }
            }
        }
    }

    // Expand columns
    for i_col in (0..space[0].len()).rev() {
        let mut contains_galaxy = false;

        for line in &space {
            if line[i_col] == '#' {
                contains_galaxy = true;
                break;
            }
        }

        if !contains_galaxy {
            for galaxy in &mut galaxies {
                if galaxy.0 > i_col as i64 {
                    galaxy.0 += expand_by;
                }
            }
        }
    }

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| (a.0 - b.0).abs() + (a.1 - b.1).abs())
        .sum::<i64>()
}

fn main() {
    let space = aoc_base::read_input_grid();

    println!("Part 1: {}", find_distances(space.clone(), 1));
    println!("Part 1: {}", find_distances(space, 999_999));
}
