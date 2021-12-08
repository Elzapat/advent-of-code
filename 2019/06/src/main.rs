use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {

    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut planets_and_orbits: HashMap<String, String> = HashMap::new();

    for line in lines {
        let planets: Vec<String> = line.unwrap().split(')')
            .map(|s| s.to_string()).collect();

        planets_and_orbits.insert(planets[1].clone(), planets[0].clone());
    }

    println!("Part 1: {}", part_1(&planets_and_orbits));
    println!("Part 2: {}", part_2(&planets_and_orbits));
}

fn part_1(planets_and_orbits: &HashMap<String, String>) -> i32 {

    let mut nb_orbits = 0;
    for (orbit, _planet) in planets_and_orbits {
        let mut orbit_chain = orbit;
        while *orbit_chain != "COM".to_string() {
            orbit_chain = planets_and_orbits.get(orbit_chain).unwrap();
            nb_orbits += 1;
        }
    }

    nb_orbits
}

fn part_2(planets_and_orbits: &HashMap<String, String>) -> i32 {

    let mut you_to_com = vec![];
    let mut san_to_com = vec![];
    let mut common_to_com = vec![];
    let mut starting_planets: [String; 3] = ["YOU".to_string(), "SAN".to_string(), "toto".to_string()];

    for i in 0..2 {
        let mut planet = &starting_planets[i];
        while *planet != "COM".to_string() {
            planet = planets_and_orbits.get(planet).unwrap();
            match i {
                0 => you_to_com.push(planet.clone()),
                1 => san_to_com.push(planet.clone()),
                _ => panic!("Problem"),
            }
        }
    }

    'outer: for you_path in &you_to_com {
        for san_path in &san_to_com {
            if you_path == san_path {
                starting_planets[2] = you_path.to_string();
                break 'outer;
            }
        }
    }

    let mut planet = &starting_planets[2];
    while *planet != "COM".to_string() {
        planet = planets_and_orbits.get(planet).unwrap();
        common_to_com.push(planet.clone());
    }

    ((you_to_com.len() - common_to_com.len()) + (san_to_com.len() - common_to_com.len())) as i32 - 2
}
