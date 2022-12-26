use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
enum Tile {
    Wall,
    Free(Vec<char>),
}

fn advance_blizzards(map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut result = map.clone();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if let Tile::Free(bs) = &mut result[i][j] {
                bs.clear();
            }
        }
    }

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if let Tile::Free(blizzards) = &map[i][j] {
                for blizzard in blizzards {
                    match blizzard {
                        b @ '>' => match &mut result[i][j + 1] {
                            Tile::Wall => {
                                if let Tile::Free(bs) = &mut result[i][1] {
                                    bs.push(*b);
                                }
                            }
                            Tile::Free(bs) => bs.push(*b),
                        },
                        b @ 'v' => match &mut result[i + 1][j] {
                            Tile::Wall => {
                                if let Tile::Free(bs) = &mut result[1][j] {
                                    bs.push(*b);
                                }
                            }
                            Tile::Free(bs) => bs.push(*b),
                        },
                        b @ '^' => match &mut result[i - 1][j] {
                            Tile::Wall => {
                                if let Tile::Free(bs) = &mut result[map.len() - 2][j] {
                                    bs.push(*b);
                                }
                            }
                            Tile::Free(bs) => bs.push(*b),
                        },
                        b @ '<' => match &mut result[i][j - 1] {
                            Tile::Wall => {
                                if let Tile::Free(bs) = &mut result[i][map[i].len() - 2] {
                                    bs.push(*b);
                                }
                            }
                            Tile::Free(bs) => bs.push(*b),
                        },
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    result
}

fn print_map(map: &[Vec<Tile>]) {
    for row in map {
        for tile in row {
            print!(
                "{}",
                match tile {
                    Tile::Wall => '#',
                    Tile::Free(bs) => {
                        match bs.len() {
                            0 => '.',
                            1 => bs[0],
                            n => char::from_digit(n as u32, 10).unwrap(),
                        }
                    }
                }
            );
        }
        println!();
    }
}

fn find_neighbours(map: &[Vec<Tile>], pos: (i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbours = vec![];

    for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)] {
        let (x, y) = ((pos.0 + dx) as usize, (pos.1 + dy) as usize);

        if let Some(row) = map.get(y) {
            if let Some(Tile::Free(bs)) = row.get(x) {
                if bs.is_empty() {
                    neighbours.push((x as i32, y as i32));
                }
            }
        }
    }

    neighbours
}

fn find_shortest(
    states: &[Vec<Vec<Tile>>],
    start: (i32, i32),
    end: (i32, i32),
    start_time: usize,
) -> Option<usize> {
    let repeat_len = states[0].len() * states[0][0].len();

    let mut parents = HashMap::new();
    let mut explored = HashSet::new();
    let mut queue = VecDeque::new();

    explored.insert((start, start_time));
    queue.push_back((start, start_time));

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if current.0 == end {
            return Some(current.1);
        }

        for n in find_neighbours(&states[current.1 % repeat_len], current.0) {
            if !explored.contains(&(n, current.1 + 1)) {
                explored.insert((n, current.1 + 1));
                queue.push_back((n, current.1 + 1));
                parents.insert((n, current.1 + 1), current);
            }
        }
    }

    None
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let start = (lines[0].chars().position(|c| c == '.').unwrap() as i32, 0);
    let end = (
        lines[lines.len() - 1]
            .chars()
            .position(|c| c == '.')
            .unwrap() as i32,
        lines.len() as i32 - 1,
    );

    let mut map = vec![];

    for line in lines.iter() {
        let mut row = vec![];

        for c in line.chars() {
            row.push(match c {
                '#' => Tile::Wall,
                '.' => Tile::Free(vec![]),
                b => Tile::Free(vec![b]),
            });
        }

        map.push(row);
    }

    let mut states = vec![];

    for _ in 0..map.len() * map[0].len() {
        map = advance_blizzards(map);
        states.push(map.clone());
    }

    println!("Part 1: {:?}", find_shortest(&states, start, end, 0));
    println!(
        "Part 2: {:?}",
        find_shortest(
            &states,
            start,
            end,
            find_shortest(
                &states,
                end,
                start,
                find_shortest(&states, start, end, 0).unwrap()
            )
            .unwrap()
        )
    );
}
