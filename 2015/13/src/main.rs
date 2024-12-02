use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = aoc_base::read_input();
    let mut associations = HashMap::<(String, String), i32>::new();
    let mut names = HashSet::<String>::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();

        let main_name = words.next().unwrap();
        names.insert(main_name.to_string());

        words.next();

        let lose = words.next() == Some("lose");
        let amount = words.next().unwrap().parse::<i32>().unwrap();
        let happiness = if lose { -1 * amount } else { amount };

        let neighbour = words.skip(6).next().unwrap().replace('.', "");

        *associations
            .entry((main_name.to_string(), neighbour.to_string()))
            .or_insert(0) += happiness;

        *associations
            .entry((neighbour.to_string(), main_name.to_string()))
            .or_insert(0) += happiness;
    }

    for name in &names {
        associations.insert((name.clone(), "Me".to_string()), 0);
        associations.insert(("Me".to_string(), name.clone()), 0);
    }

    names.insert("Me".to_string());

    let mut best_perm = 0;

    for perm in names.iter().permutations(names.len()).unique() {
        let mut perm_score = 0;

        for (&n1, &n2) in perm.iter().tuple_windows() {
            perm_score += associations[&(n1.clone(), n2.clone())];
        }
        perm_score += associations[&(perm[0].clone(), perm[perm.len() - 1].clone())];

        if perm_score > best_perm {
            best_perm = perm_score;
        }
    }

    println!("Part 2: {best_perm}");
}
