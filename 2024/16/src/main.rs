use aoc_base::Point;
use pathfinding::directed::{dijkstra::dijkstra, yen::yen};
use std::collections::HashSet;

fn main() {
    let map = aoc_base::read_input_grid_char();

    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == 'S' {
                start.r = r;
                start.c = c;
            } else if map[r][c] == 'E' {
                end.r = r;
                end.c = c;
            }
        }
    }

    let successors = |(pos, cur_dir): &(Point, (isize, isize))| {
        let mut neighbours = vec![];

        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let neighbour = Point::new(
                (pos.r as isize + dir.0) as usize,
                (pos.c as isize + dir.1) as usize,
            );

            if map[neighbour.r][neighbour.c] != '#' {
                neighbours.push((
                    (neighbour, dir),
                    if (cur_dir.0 == 0 && dir.0 != 0) || (cur_dir.0 != 0 && dir.0 == 0) {
                        1001
                    } else {
                        1
                    },
                ));
            }
        }

        neighbours
    };

    let success = |(pos, _): &(Point, (isize, isize))| *pos == end;

    let (_, total_cost) = dijkstra(&(start, (0_isize, 1_isize)), successors, success).unwrap();

    println!("Part 1: {}", total_cost);

    let paths = yen(&(start, (0_isize, 1_isize)), successors, success, 9);

    let unique_tiles = paths
        .iter()
        .flat_map(|(path, _)| path)
        .map(|(p, _)| *p)
        .collect::<HashSet<Point>>();

    println!("Part 2: {}", unique_tiles.len());
}
