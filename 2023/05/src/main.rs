#[derive(Debug, Clone, PartialEq)]
#[repr(u64)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl From<u64> for Category {
    fn from(source: u64) -> Self {
        match source {
            0 => Self::Seed,
            1 => Self::Soil,
            2 => Self::Fertilizer,
            3 => Self::Water,
            4 => Self::Light,
            5 => Self::Temperature,
            6 => Self::Humidity,
            7 => Self::Location,
            _ => panic!("Unexpected number from Category"),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    source_cat: Category,
    dest_cat: Category,
    maps: Vec<(u64, u64, u64)>,
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let mut lines = input.lines();

    let mut seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut seeds_pair = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        seeds_pair.push((seeds[i], seeds[i + 1]))
    }

    let mut maps: Vec<Map> = Vec::new();

    let mut cat_counter = 0;

    let mut current_map = Map {
        source_cat: Category::Seed,
        dest_cat: Category::Seed,
        maps: Vec::new(),
    };

    let _ = lines.next();

    for line in lines {
        if line == "" {
            maps.push(current_map.clone());
            current_map.maps.drain(..);
            continue;
        } else if line.contains(':') {
            cat_counter += 1;

            current_map.source_cat = (cat_counter - 1).into();
            current_map.dest_cat = cat_counter.into();

            continue;
        }

        let maps = line
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        current_map.maps.push((maps[0], maps[1], maps[2]));
    }

    maps.push(current_map.clone());

    for i in 0..7 {
        let source: Category = i.into();
        let dest: Category = (i + 1).into();

        let map = maps
            .iter()
            .find(|m| m.source_cat == source && m.dest_cat == dest)
            .unwrap();

        for seed in &mut seeds {
            for map in &map.maps {
                if *seed >= map.1 && *seed < map.1 + map.2 {
                    *seed = map.0 + (*seed - map.1);
                    break;
                }
            }
        }
    }

    let mut min_seed = u64::MAX;

    for seed_pair in &seeds_pair {
        for mut seed in seed_pair.0..seed_pair.0 + seed_pair.1 {
            for i in 0..7 {
                let map = &maps[i as usize];

                for map in &map.maps {
                    if seed >= map.1 && seed < map.1 + map.2 {
                        seed = map.0 + (seed - map.1);
                        break;
                    }
                }
            }

            if seed < min_seed {
                min_seed = seed;
            }
        }
    }

    println!("Part 1: {}", seeds.iter().min().unwrap());
    println!("Part 2: {}", min_seed);
}
