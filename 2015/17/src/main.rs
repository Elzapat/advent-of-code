use std::collections::HashSet;

const EGGNOG_VOLUME: u32 = 150;

fn main() {
    let containers = aoc_base::read_input()
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut solutions = HashSet::new();

    find_possible_combination(&containers, 0, 0, Vec::new(), &mut solutions);
    println!("Part 1: {}", solutions.len());

    let mut min_container = usize::MAX;
    let mut nb_containers = 0;

    for sol in &solutions {
        if sol.len() < min_container {
            min_container = sol.len();
            nb_containers = 1;
        } else if sol.len() == min_container {
            nb_containers += 1;
        }
    }

    println!("Part 2: {}", nb_containers);
}

fn find_possible_combination(
    containers: &[u32],
    start_at: usize,
    mut current_sum: u32,
    mut current_solution: Vec<usize>,
    solutions: &mut HashSet<Vec<usize>>,
) {
    if start_at > containers.len() - 1 {
        return;
    }

    find_possible_combination(
        containers,
        start_at + 1,
        current_sum,
        current_solution.clone(),
        solutions,
    );

    if current_sum + containers[start_at] == EGGNOG_VOLUME {
        current_solution.push(start_at);
        solutions.insert(current_solution);
        return;
    }

    if current_sum + containers[start_at] < EGGNOG_VOLUME {
        current_sum += containers[start_at];
        current_solution.push(start_at);
    }

    find_possible_combination(
        containers,
        start_at + 1,
        current_sum,
        current_solution,
        solutions,
    );
}
