use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Eq, Hash)]
struct Elf {
    pos: (i32, i32),
    proposes: Option<(i32, i32)>,
}

impl PartialEq for Elf {
    fn eq(&self, other: &Elf) -> bool {
        self.pos == other.pos
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

fn get_neighbours(pos: (i32, i32)) -> HashMap<Dir, (i32, i32)> {
    let dirs: HashMap<Dir, (i32, i32)> = [
        (Dir::N, (0, -1)),
        (Dir::NE, (1, -1)),
        (Dir::E, (1, 0)),
        (Dir::SE, (1, 1)),
        (Dir::S, (0, 1)),
        (Dir::SW, (-1, 1)),
        (Dir::W, (-1, 0)),
        (Dir::NW, (-1, -1)),
    ]
    .into_iter()
    .collect();

    let mut neighbours = HashMap::new();

    for (dir, (dx, dy)) in &dirs {
        neighbours.insert(*dir, (pos.0 + dx, pos.1 + dy));
    }

    neighbours
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let mut elves = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    for round in 0.. {
        let mut props: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        for elf in &elves {
            let ns = get_neighbours(*elf);

            if ns.iter().all(|(_, n)| !elves.contains(n)) {
                continue;
            }

            let conds = [
                (
                    !elves.contains(&ns[&Dir::N])
                        && !elves.contains(&ns[&Dir::NE])
                        && !elves.contains(&ns[&Dir::NW]),
                    ns[&Dir::N],
                ),
                (
                    !elves.contains(&ns[&Dir::S])
                        && !elves.contains(&ns[&Dir::SE])
                        && !elves.contains(&ns[&Dir::SW]),
                    ns[&Dir::S],
                ),
                (
                    !elves.contains(&ns[&Dir::W])
                        && !elves.contains(&ns[&Dir::NW])
                        && !elves.contains(&ns[&Dir::SW]),
                    ns[&Dir::W],
                ),
                (
                    !elves.contains(&ns[&Dir::E])
                        && !elves.contains(&ns[&Dir::NE])
                        && !elves.contains(&ns[&Dir::SE]),
                    ns[&Dir::E],
                ),
            ];

            for i in 0..4 {
                let (free, prop) = conds[(round + i) % 4];

                if free {
                    props.entry(prop).or_insert(vec![]).push(*elf);
                    break;
                }
            }
        }

        if props.is_empty() {
            println!("Part 2: {}", round + 1);
            break;
        }

        for (prop, pos) in props {
            if pos.len() == 1 {
                elves.remove(&pos[0]);
                elves.insert(prop);
            }
        }

        if round == 9 {
            let min_x = elves.iter().map(|elf| elf.0).min().unwrap();
            let max_x = elves.iter().map(|elf| elf.0).max().unwrap();
            let min_y = elves.iter().map(|elf| elf.1).min().unwrap();
            let max_y = elves.iter().map(|elf| elf.1).max().unwrap();
            println!(
                "Part 1: {}",
                (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32
            );
        }
    }
}
