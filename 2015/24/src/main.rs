use itertools::Itertools;

fn main() {
    let packages = aoc_base::read_input()
        .lines()
        .map(|l| l.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    println!("Part 1: {}", find_groups(packages.clone(), 3));
    println!("Part 2: {}", find_groups(packages, 4));
}

fn find_groups(packages: Vec<u128>, groups: u128) -> u128 {
    let total_weight = packages.iter().sum::<u128>();
    let group_weight = total_weight / groups;
    let mut min_qe = (u128::MAX, u128::MAX);

    for i in 0..packages.len() {
        for combination in packages.iter().combinations(i) {
            if combination.iter().map(|p| **p).sum::<u128>() == group_weight {
                let qe = combination.iter().map(|p| **p).product();

                if (combination.len() as u128) < min_qe.0 {
                    min_qe = (combination.len() as u128, qe);
                } else if combination.len() as u128 == min_qe.0 {
                    min_qe.1 = min_qe.1.min(qe);
                }
            }
        }
    }

    min_qe.1
}
