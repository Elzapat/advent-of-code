use std::collections::{HashMap, HashSet};

fn main() {
    let input = aoc_base::read_input();
    let mut connections = HashMap::new();

    for line in input.lines() {
        let mut split = line.split('-');
        let cp1 = split.next().unwrap().to_string();
        let cp2 = split.next().unwrap().to_string();

        connections
            .entry(cp1.clone())
            .or_insert(vec![])
            .push(cp2.clone());
        connections
            .entry(cp2.clone())
            .or_insert(vec![])
            .push(cp1.clone());
    }

    let mut result = 0;
    let mut visited = vec![];

    for cp1 in connections.keys() {
        for cp2 in &connections[cp1] {
            for cp3 in &connections[cp2] {
                if cp1 == cp2 && cp2 == cp3 {
                    continue;
                }

                if visited.contains(&(cp1, cp2, cp3)) {
                    continue;
                }

                visited.push((cp1, cp2, cp3));
                visited.push((cp1, cp3, cp2));
                visited.push((cp2, cp1, cp3));
                visited.push((cp2, cp3, cp1));
                visited.push((cp3, cp1, cp2));
                visited.push((cp3, cp2, cp1));

                if !cp1.starts_with('t') && !cp2.starts_with('t') && !cp3.starts_with('t') {
                    continue;
                }

                if connections[cp1].contains(cp2) && connections[cp1].contains(cp3) {
                    result += 1;
                }
            }
        }
    }

    println!("Part 1: {result}");

    let mut cliques = vec![];

    bron_kerbosch(
        HashSet::new(),
        connections.keys().cloned().collect(),
        HashSet::new(),
        &mut cliques,
        &connections,
    );

    let mut biggest_network = cliques
        .into_iter()
        .fold(HashSet::new(), |acc, cur| {
            if cur.len() > acc.len() {
                cur
            } else {
                acc
            }
        })
        .into_iter()
        .collect::<Vec<String>>();

    biggest_network.sort();

    println!("Part 2: {}", biggest_network.join(","));
}

fn bron_kerbosch(
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    c: &mut Vec<HashSet<String>>,
    connections: &HashMap<String, Vec<String>>,
) {
    if p.is_empty() && x.is_empty() {
        c.push(r.clone());
    }

    for v in p.clone() {
        bron_kerbosch(
            r.union(&vec![v.clone()].into_iter().collect())
                .cloned()
                .collect::<HashSet<String>>(),
            p.intersection(&connections[&v].iter().cloned().collect())
                .cloned()
                .collect(),
            x.intersection(&connections[&v].iter().cloned().collect())
                .cloned()
                .collect(),
            c,
            connections,
        );

        p.remove(&v);
        x.insert(v);
    }
}
