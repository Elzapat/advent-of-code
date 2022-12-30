use itertools::Itertools;
use regex::Regex;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve {
    rate: u32,
    neighbours: HashSet<String>,
}

#[derive(Eq, PartialEq)]
struct Node<'a> {
    valve: &'a str,
    distance: u32,
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

fn distance(start: &str, end: &str, valves: &HashMap<String, Valve>) -> Option<u32> {
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    queue.push(Node {
        valve: start,
        distance: 0,
    });

    while let Some(Node {
        valve: current,
        distance,
    }) = queue.pop()
    {
        if current == end {
            return Some(distance);
        }

        for n in &valves[current].neighbours {
            if seen.insert(n) {
                queue.push(Node {
                    valve: &n,
                    distance: distance + 1,
                });
            }
        }
    }

    None
}

fn compute_distances<'a>(valves: &'a HashMap<String, Valve>) -> HashMap<(&'a str, &'a str), u32> {
    valves
        .iter()
        .filter(|(_, valve)| valve.rate > 0)
        .map(|(name, _)| name)
        .tuple_combinations()
        .fold(HashMap::new(), |mut dist_map, (v1, v2)| {
            dist_map
                .entry(("AA", v1))
                .or_insert_with(|| distance("AA", v1, valves).unwrap());
            dist_map
                .entry(("AA", v2))
                .or_insert_with(|| distance("AA", v2, valves).unwrap());

            let Some(distance) = distance(v1, v2, valves) else {
                return dist_map;
            };

            dist_map.insert((v1, v2), distance);
            dist_map.insert((v2, v1), distance);

            dist_map
        })
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>();
    let regex =
        Regex::new(r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();

    let mut valves = HashMap::new();

    for line in &lines {
        for cap in regex.captures_iter(&line) {
            let name = cap[1].to_owned();
            let rate = cap[2].parse().unwrap();
            let leads_to = cap[3].split(',').map(|v| v.trim().to_string()).collect();

            valves.insert(
                name,
                Valve {
                    rate,
                    neighbours: leads_to,
                },
            );
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct State {
        pressure_released: u32,
        opened: BTreeSet<String>,
        current: String,
        elapsed: u32,
    }

    const END_TIME: u32 = 26;

    let wait_until_end = |state: &State| {
        let time_left = END_TIME - state.elapsed;
        let release_per_min: u32 = state.opened.iter().map(|name| valves[&*name].rate).sum();
        state.pressure_released + (time_left * release_per_min)
    };

    let dist_map = compute_distances(&valves);
    let flowing_valves: HashSet<&str> = valves
        .iter()
        .filter(|(_, valve)| valve.rate > 0)
        .map(|(name, _)| name.as_str())
        .collect();

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut max_pressure_released_states = HashMap::new();
    let mut max_pressure_released = 0;

    queue.push_back(State {
        pressure_released: 0,
        opened: BTreeSet::new(),
        current: "AA".to_owned(),
        elapsed: 0,
    });
    seen.insert((BTreeSet::new(), 0, 0));

    while let Some(state) = queue.pop_front() {
        if state.opened.len() == flowing_valves.len() || state.elapsed >= END_TIME {
            let released_at_end = wait_until_end(&state);
            max_pressure_released_states
                .entry(state.opened.clone())
                .and_modify(|released| *released = released_at_end.max(*released))
                .or_insert(released_at_end);
            max_pressure_released = max_pressure_released.max(released_at_end);
            continue;
        }

        let unopened = flowing_valves
            .iter()
            .filter(|name| !state.opened.contains(**name));

        for dest in unopened {
            let dist = dist_map[&(state.current.as_str(), *dest)] + 1;
            let new_elapsed = state.elapsed + dist;

            if new_elapsed >= END_TIME {
                let released_at_end = wait_until_end(&state);
                max_pressure_released_states
                    .entry(state.opened.clone())
                    .and_modify(|released| *released = released_at_end.max(*released))
                    .or_insert(released_at_end);
                max_pressure_released = max_pressure_released.max(released_at_end);
                continue;
            }

            let released_per_min: u32 = state.opened.iter().map(|name| valves[&*name].rate).sum();
            let new_released = state.pressure_released + (released_per_min * dist);

            let mut new_opened = state.opened.clone();
            new_opened.insert((*dest).to_owned());

            if seen.insert((new_opened.clone(), new_elapsed, new_released)) {
                queue.push_back(State {
                    opened: new_opened,
                    pressure_released: new_released,
                    current: (*dest).to_owned(),
                    elapsed: new_elapsed,
                });
            }
        }
    }

    println!(
        "Part 2: {}",
        max_pressure_released_states
            .iter()
            .tuple_combinations()
            .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
            .map(|(human, elephant)| human.1 + elephant.1)
            .max()
            .unwrap()
    );
}
