use std::collections::HashMap;

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let input = file.lines().collect::<Vec<_>>();

    let mut pair_insertions: Vec<(&str, char)> = vec![];
    input[2..]
        .into_iter()
        .for_each(|line| {
            let elems = line.split("->").map(|elem| elem.trim()).collect::<Vec<_>>();
            pair_insertions.push((elems[0], elems[1].chars().nth(0).unwrap()));
        });

    {
        let mut template = input[0].chars().collect::<Vec<char>>();
        let pair_insertions = pair_insertions.clone();

        for _ in 0..10 {
            let mut inserted_elems = vec![];
            for pair in &pair_insertions {
                for i in 0..template.len() - 1 {
                    if format!("{}{}", template[i], template[i + 1]) == pair.0 {
                        inserted_elems.push((i + 1, pair.1));
                    }
                }
            }
            inserted_elems.sort_by(|a, b| a.0.cmp(&b.0));
            for i in (0..inserted_elems.len()).rev() {
                template.insert(inserted_elems[i].0, inserted_elems[i].1);
            }
        }

        let mut frequencies = HashMap::new();
        for elem in &template {
            if let None = frequencies.get(elem) {
                frequencies.insert(elem, 0);
            } else {
                *frequencies.get_mut(elem).unwrap() += 1;
            }
        }

        let mut most_frequent = 0;
        let mut least_frequent = u32::MAX;
        for (_, n) in &frequencies {
            if *n > most_frequent {
                most_frequent = *n;
            }
            if *n < least_frequent {
                least_frequent = *n;
            }
        }

        println!("Part 1: {}", most_frequent - least_frequent);
    }

    {
        let mut pairs = HashMap::<String, u64>::new();
        let pair_insertions = pair_insertions.clone();

        for i in 0..input[0].len() - 1 {
            let pair = &input[0][i..=i+1];
            if let None = pairs.get(pair) {
                pairs.insert(pair.to_owned(), 1);
            } else {
                *pairs.get_mut(pair).unwrap() += 1;
            }
        }

        const STEPS: usize = 40;
        for _ in 0..STEPS {
            let mut new_pairs = HashMap::<String, u64>::new();
            for pair_insert in &pair_insertions {
                if let Some(n) = pairs.get(pair_insert.0) {
                    let pair1 = format!("{}{}", pair_insert.0.chars().nth(0).unwrap(), pair_insert.1);
                    let pair2 = format!("{}{}", pair_insert.1, pair_insert.0.chars().nth(1).unwrap());

                    for pair in [pair1, pair2] {
                        if let None = new_pairs.get(&pair) {
                            new_pairs.insert(pair.clone(), *n);
                        } else {
                            *new_pairs.get_mut(&pair).unwrap() += n;
                        }
                    }
                }
            }
            pairs = new_pairs;
        }

        let mut frequencies = HashMap::new();
        for (pair, n) in pairs.iter() {
            for c in pair.chars() {
                if let None = frequencies.get(&c) {
                    frequencies.insert(c, *n);
                } else {
                    *frequencies.get_mut(&c).unwrap() += *n;
                }
            }
        }
        for (_, n) in frequencies.iter_mut() {
            *n /= 2;
        }
        *frequencies.get_mut(&input[0].chars().nth(0).unwrap()).unwrap() += 1;
        *frequencies.get_mut(&input[0].chars().last().unwrap()).unwrap() += 1;

        let mut most_frequent = 0;
        let mut least_frequent = u64::MAX;
        for (_, n) in &frequencies {
            if *n > most_frequent {
                most_frequent = *n;
            }
            if *n < least_frequent {
                least_frequent = *n;
            }
        }

        println!("Part 2: {}", most_frequent - least_frequent);
    }
}
