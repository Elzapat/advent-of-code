use aoc_base::Point;
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::HashSet;

fn main() {
    let grid = aoc_base::read_input_grid_char();

    println!("Part 1: {}", find_nb_cheats(grid.clone(), 2, 100));
    println!("Part 2: {}", find_nb_cheats(grid.clone(), 20, 100));
}

fn find_nb_cheats(grid: Vec<Vec<char>>, max_cheat_time: usize, min_cheat_save: usize) -> usize {
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 'S' {
                start = Point::new(r, c);
            } else if grid[r][c] == 'E' {
                end = Point::new(r, c);
            }
        }
    }

    let successors = |pos: &Point| {
        let mut neighbours = pos.get_4_neighbours(grid.len(), grid[0].len());
        neighbours.retain(|n| grid[n.r][n.c] != '#');

        neighbours.into_iter().map(|n| (n, 1))
    };

    let og_path = dijkstra(&start, successors, |pos| *pos == end).unwrap();
    let mut cheats = HashSet::new();

    for (part_path_len, pos) in og_path.0.iter().enumerate() {
        let mut cheat_ends = vec![];
        for r in 0..grid.len() {
            for c in 0..grid.len() {
                if grid[r][c] != '#' && pos.manhattan_distance(&Point::new(r, c)) <= max_cheat_time
                {
                    cheat_ends.push(Point::new(r, c));
                }
            }
        }

        for cheat_end in &cheat_ends {
            let path_len = if let Some(index) = og_path.0.iter().position(|pos| pos == cheat_end) {
                part_path_len + cheat_end.manhattan_distance(pos) + og_path.0.len() - index
            } else if let Some((cheated_path, _)) =
                dijkstra(cheat_end, successors, |pos| *pos == end)
            {
                cheated_path.len() + part_path_len + cheat_end.manhattan_distance(pos)
            } else {
                usize::MAX
            };

            if og_path.0.len() > path_len && og_path.0.len() - path_len >= min_cheat_save {
                cheats.insert((*pos, *cheat_end));
            }
        }
    }

    cheats.len()
}
