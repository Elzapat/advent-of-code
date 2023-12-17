use pathfinding::directed::dijkstra::dijkstra;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Right => Dir::Left,
            Dir::Left => Dir::Right,
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Crucible {
    x: u32,
    y: u32,
    last_moves: [Dir; 3],
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct CrucibleP2 {
    x: u32,
    y: u32,
    last_moves: [Dir; 10],
}

fn main() {
    let map = aoc_base::read_input_grid();
    let map: Vec<Vec<u32>> = map
        .iter()
        .map(|r| r.iter().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let start = Crucible {
        x: 0,
        y: 0,
        last_moves: [Dir::Left, Dir::Up, Dir::Left],
    };

    let successors_p1 = |node: &Crucible| {
        let mut successors = Vec::new();
        let mut dir_excludes = Vec::new();

        let straight_line = node
            .last_moves
            .iter()
            .all(|d| *d == node.last_moves[0])
            .then(|| node.last_moves[0]);

        match straight_line {
            Some(dir) => dir_excludes.extend_from_slice(&[dir, dir.opposite()]),
            None => dir_excludes.push(node.last_moves[2].opposite()),
        }

        for (dx, dy, dir) in [
            (1, 0, Dir::Right),
            (-1, 0, Dir::Left),
            (0, 1, Dir::Down),
            (0, -1, Dir::Up),
        ] {
            if dir_excludes.contains(&dir) {
                continue;
            }

            let (x, y) = (node.x as i32 + dx, node.y as i32 + dy);

            if x < 0 || x > map[0].len() as i32 - 1 || y < 0 || y > map.len() as i32 - 1 {
                continue;
            }

            successors.push((
                Crucible {
                    x: x as u32,
                    y: y as u32,
                    last_moves: [node.last_moves[1], node.last_moves[2], dir],
                },
                map[y as usize][x as usize],
            ));
        }

        successors
    };

    let result_p1 = dijkstra(&start, successors_p1, |node| {
        node.x == map[0].len() as u32 - 1 && node.y == map.len() as u32 - 1
    });

    println!("Part 1: {}", result_p1.unwrap().1);

    let start_p2 = CrucibleP2 {
        x: 0,
        y: 0,
        last_moves: [Dir::Up; 10],
    };

    let successors_p2 = |node: &CrucibleP2| {
        let mut successors = Vec::new();
        let mut dir_excludes = Vec::new();

        let straight_line = node
            .last_moves
            .iter()
            .all(|d| *d == node.last_moves[0])
            .then(|| node.last_moves[0]);

        match straight_line {
            Some(dir) => dir_excludes.extend_from_slice(&[dir, dir.opposite()]),
            None => {
                if node.last_moves[9] != node.last_moves[8]
                    || node.last_moves[8] != node.last_moves[7]
                    || node.last_moves[7] != node.last_moves[6]
                {
                    dir_excludes.extend_from_slice(&[Dir::Right, Dir::Left, Dir::Up, Dir::Down]);
                    dir_excludes.retain(|d| *d != node.last_moves[9]);
                } else {
                    dir_excludes.push(node.last_moves[9].opposite());
                }
            }
        }

        for (dx, dy, dir) in [
            (1, 0, Dir::Right),
            (-1, 0, Dir::Left),
            (0, 1, Dir::Down),
            (0, -1, Dir::Up),
        ] {
            if dir_excludes.contains(&dir) {
                continue;
            }

            let (x, y) = (node.x as i32 + dx, node.y as i32 + dy);

            if x < 0 || x > map[0].len() as i32 - 1 || y < 0 || y > map.len() as i32 - 1 {
                continue;
            }

            successors.push((
                CrucibleP2 {
                    x: x as u32,
                    y: y as u32,
                    last_moves: [
                        node.last_moves[1],
                        node.last_moves[2],
                        node.last_moves[3],
                        node.last_moves[4],
                        node.last_moves[5],
                        node.last_moves[6],
                        node.last_moves[7],
                        node.last_moves[8],
                        node.last_moves[9],
                        dir,
                    ],
                },
                map[y as usize][x as usize],
            ));
        }

        successors
    };

    let result_p2 = dijkstra(&start_p2, successors_p2, |node| {
        node.x == map[0].len() as u32 - 1 && node.y == map.len() as u32 - 1
    });

    println!("Part 2: {}", result_p2.unwrap().1);
}
