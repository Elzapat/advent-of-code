use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::PartialEq;

fn main() {
    let input = aoc_base::read_input();
    let mut cities: HashSet<String> = HashSet::new();
    let mut distances: HashMap<(String, String), u32> = HashMap::new();

    for distance in input.lines() {
        let mut parts = distance.split(" = "); 
        let mut cities_parts = parts.next().unwrap().split(" to ");

        let distance = parts.next().unwrap().parse::<u32>().unwrap();
        let city1 = cities_parts.next().unwrap().to_string();
        let city2 = cities_parts.next().unwrap().to_string();

        cities.insert(city1.clone());
        cities.insert(city2.clone());

        distances.insert((city1.clone(), city2.clone()), distance);
        distances.insert((city2, city1), distance);
    }

    let permutations = get_all_permutations(cities.into_iter().collect());
    let mut shortest_route = u32::MAX;
    let mut longest_route = 0;

    for permutation in &permutations {
        let route_length = (0..permutation.len() - 1)
            .map(|i| distances[&(permutation[i].clone(), permutation[i + 1].clone())])
            .sum::<u32>();

        if route_length < shortest_route {
            shortest_route = route_length;
        }

        if route_length > longest_route {
            longest_route = route_length;
        }
    }

    println!("Part 1: {shortest_route}");
    println!("Part 1: {longest_route}");
}

fn get_all_permutations(cities: Vec<String>) -> Vec<Vec<String>> {
    let mut result = HashSet::new();
    let mut cities = cities.clone();

    cities.sort();
    permute(&mut cities, 0, &mut result);

    result.into_iter().collect()
}

fn permute(cities: &mut Vec<String>, start: usize, result: &mut HashSet<Vec<String>>) {
    if start == cities.len() {
        result.insert(cities.clone());
        return;
    }

    for i in start..cities.len() {
        cities.swap(start, i);
        permute(cities, start + 1, result);
        cities.swap(start, i);
    }
}
