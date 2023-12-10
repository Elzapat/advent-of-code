#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    None,
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::None => Direction::None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Directions {
    pub dir1: Direction,
    pub dir2: Direction,
}

impl Directions {
    pub fn new(dir1: Direction, dir2: Direction) -> Self {
        Directions { dir1, dir2 }
    }
}

#[derive(Debug, Copy, Clone)]
struct Pipe {
    dirs: Directions,
}

fn main() {
    let pipes_grid = aoc_base::read_input_grid();
    let start_y = pipes_grid.iter().position(|l| l.contains(&'S')).unwrap() as i32;
    let start_x = pipes_grid[start_y as usize]
        .iter()
        .position(|&p| p == 'S')
        .unwrap() as i32;

    let pipes = pipes_grid
        .iter()
        .map(|l| {
            l.iter()
                .map(|p| Pipe {
                    dirs: match p {
                        '-' => Directions::new(Direction::West, Direction::East),
                        '|' => Directions::new(Direction::North, Direction::South),
                        'J' => Directions::new(Direction::North, Direction::West),
                        'F' => Directions::new(Direction::South, Direction::East),
                        '7' => Directions::new(Direction::South, Direction::West),
                        'L' => Directions::new(Direction::North, Direction::East),
                        'S' => Directions::new(Direction::East, Direction::South),
                        '.' => Directions::new(Direction::None, Direction::None),
                        _ => unreachable!(),
                    },
                })
                .collect::<Vec<Pipe>>()
        })
        .collect::<Vec<Vec<Pipe>>>();

    let mut current_pos = (start_x, start_y);
    let mut previous_pos = (0, 0);
    let mut path = vec![current_pos];

    loop {
        // println!(
        //     "{current_pos:?} {:?} {:?}",
        //     pipes[current_pos.1 as usize][current_pos.0 as usize], path
        // );

        for (target_dir, dx, dy) in [
            (Direction::West, -1, 0),
            (Direction::East, 1, 0),
            (Direction::South, 0, 1),
            (Direction::North, 0, -1),
        ] {
            let target_pos = (current_pos.0 + dx, current_pos.1 + dy);

            // println!("{target_pos:?} {current_pos:?}");
            // if target_pos == previous_pos {
            //     continue;
            // }

            if target_pos.0 < 0
                || target_pos.0 > pipes[0].len() as i32 - 1
                || target_pos.1 < 0
                || target_pos.1 > pipes.len() as i32 - 1
            {
                continue;
            }

            if target_pos == previous_pos {
                continue;
            }

            if target_pos == (start_x, start_y) {
                current_pos = (start_x, start_y);
                path.push(current_pos);
                break;
            }

            let current_pipe = pipes[current_pos.1 as usize][current_pos.0 as usize];
            let target_pipe = pipes[target_pos.1 as usize][target_pos.0 as usize];

            if (target_dir.opposite() == target_pipe.dirs.dir2
                || target_dir.opposite() == target_pipe.dirs.dir1)
                && (target_dir == current_pipe.dirs.dir1 || target_dir == current_pipe.dirs.dir2)
            {
                path.push(target_pos);
                previous_pos = current_pos;
                current_pos = target_pos;
                break;
            }
        }

        if current_pos == (start_x, start_y) {
            break;
        }
    }

    println!(
        "Part 1: {}",
        path.len() / 2 + if path.len() % 2 == 0 { 0 } else { 1 }
    );

    let mut area = 0;

    for (point, next_point) in path.iter().zip(path.iter().skip(1)) {
        // Shoelace formula
        area += point.0 * next_point.1 - next_point.0 * point.1;
    }

    area /= 2;

    // Pick's theorem
    println!("Part 2: {}", -(area + path.len() as i32 / 2 - 1));
}
