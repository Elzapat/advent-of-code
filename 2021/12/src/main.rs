use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Cave {
    is_big: bool,
    visited: u32,
    connected_to: Vec<String>
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut caves = HashMap::<String, Cave>::new();
    input
        .lines()
        .map(|line| line.split("-"))
        .collect::<Vec<_>>()
        .iter_mut()
        .map(|it| {
            let first = it.next().unwrap().to_owned();
            let second = it.next().unwrap().to_owned();

            if let Some(cave) = caves.get_mut(&first) {
                cave.connected_to.push(second.clone())
            } else {
                caves.insert(first.clone(), Cave {
                    is_big: first.chars().any(|c| matches!(c, 'A'..='Z')),
                    visited: 0,
                    connected_to: vec![second.clone()]
                });
            }

            if let Some(cave) = caves.get_mut(&second) {
                cave.connected_to.push(first)
            } else {
                caves.insert(second.clone(), Cave {
                    is_big: second.chars().any(|c| matches!(c, 'A'..='Z')),
                    visited: 0,
                    connected_to: vec![first.clone()]
                });
            }
        })
        .for_each(drop);

    {
        let caves = caves.clone();

        let mut n_paths = 0;
        compute_path_p1("start".to_owned(), &mut caves.clone(), &mut n_paths);

        println!("Part 1: {}", n_paths);
    }

    {
        let caves = caves.clone();

        let mut n_paths = 0;
        compute_path_p2("start".to_owned(), &mut caves.clone(), &mut n_paths);

        println!("Part 2: {}", n_paths);
    }
}

fn compute_path_p2(name: String, caves: &mut HashMap<String, Cave>, n_paths: &mut u32) {
    if caves[&name].visited >= 1 && name == "start" {
        return;
    }

    let mut found_two_times_visited_small_cave = false;
    for (_, cave) in caves.iter() {
        if cave.visited > 1 && !cave.is_big {
            found_two_times_visited_small_cave = true;
        }
    }

    if caves[&name].visited >= 1 && !caves[&name].is_big && found_two_times_visited_small_cave {
        return;
    }

    if name == "end" {
        *n_paths += 1;
        return;
    }

    caves.get_mut(&name).unwrap().visited += 1;

    for cave in &caves.clone()[&name].connected_to {
        compute_path_p2(cave.to_owned(), &mut caves.clone(), n_paths);
    }
}

fn compute_path_p1(name: String, caves: &mut HashMap<String, Cave>, n_paths: &mut u32) {
    if caves[&name].visited >= 1 && !caves[&name].is_big {
        return;
    }

    if name == "end" {
        *n_paths += 1;
        return;
    }

    caves.get_mut(&name).unwrap().visited += 1;

    for cave in &caves.clone()[&name].connected_to {
        compute_path_p1(cave.to_owned(), &mut caves.clone(), n_paths);
    }
}
