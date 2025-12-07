use aoc_base::Point;
use std::collections::HashSet;

fn main() {
    let grid = aoc_base::read_input_grid();
    let mut start = Point::default();
    let mut result_p1 = 0;

    'main: for c in 0..grid.len() {
        for r in 0..grid[c].len() {
            if grid[r][c] == 'S' {
                start = Point { r, c };
                break 'main;
            }
        }
    }

    let mut beams = HashSet::new();
    beams.insert(start);

    for _ in 0..grid.len() - 1 {
        let old_beams = beams.drain().collect::<HashSet<Point>>();
        for b in old_beams {
            let new_pos = Point { r: b.r + 1, c: b.c };

            if grid[new_pos.r][new_pos.c] == '^' {
                result_p1 += 1;
                if new_pos.c != 0 {
                    beams.insert(Point {
                        r: new_pos.r,
                        c: new_pos.c - 1,
                    });
                }
                if new_pos.c != grid[0].len() - 1 {
                    beams.insert(Point {
                        r: new_pos.r,
                        c: new_pos.c + 1,
                    });
                }
            } else {
                beams.insert(new_pos);
            }
        }
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {}", timelines(grid, start));
}

#[memoize::memoize]
fn timelines(grid: Vec<Vec<char>>, beam: Point) -> u64 {
    if beam.r >= grid.len() - 1 {
        return 1;
    }

    if grid[beam.r][beam.c] == '^' {
        timelines(
            grid.clone(),
            Point {
                r: beam.r + 1,
                c: beam.c - 1,
            },
        ) + timelines(
            grid,
            Point {
                r: beam.r + 1,
                c: beam.c + 1,
            },
        )
    } else {
        timelines(
            grid,
            Point {
                r: beam.r + 1,
                c: beam.c,
            },
        )
    }
}
