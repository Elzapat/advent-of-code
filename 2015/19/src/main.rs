use std::collections::{HashMap, HashSet};

fn main() {
    let input = aoc_base::read_input();

    let mut replacements = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split(" => ");

        replacements
            .entry(parts.next().unwrap().to_string())
            .or_insert(Vec::new())
            .push(parts.next().unwrap().to_string())
    }

    let molecule = input.lines().last().unwrap().to_string();
    let mut replaced_molecules = HashSet::new();

    for (orig, dests) in &replacements {
        for i in 0..molecule.len() {
            if i + orig.len() > molecule.len() {
                continue;
            }

            if &molecule[i..i + orig.len()] == &orig[..] {
                for dest in dests {
                    let mut new_mol = molecule.clone();
                    new_mol.replace_range(i..i + orig.len(), dest);
                    replaced_molecules.insert(new_mol);
                }
            }
        }
    }

    println!("Part 1: {}", replaced_molecules.len());

    let mut reduced_molecule = molecule.clone();
    let mut steps = 0;

    while reduced_molecule != "e" {
        for (orig, dests) in &replacements {
            for dest in dests {
                if let Some(idx) = reduced_molecule.find(dest) {
                    reduced_molecule.replace_range(idx..idx + dest.len(), orig);
                    steps += 1;
                }
            }
        }
    }

    println!("Part 2: {steps}");
}
