use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = aoc_base::read_input();
    let mut sues = vec![];

    for line in input.lines() {
        let re = Regex::new(r"Sue \d+: (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();
        let mut sue = HashMap::new();

        for cap in re.captures_iter(line) {
            let mut i = 1;

            while i < cap.len() - 1 {
                sue.insert(cap[i].to_string(), cap[i + 1].parse::<u32>().unwrap());
                i += 2;
            }
        }

        sues.push(sue);
    }

    let mut goal = HashMap::new();
    goal.insert("children".to_string(), 3);
    goal.insert("cats".to_string(), 7);
    goal.insert("samoyeds".to_string(), 2);
    goal.insert("pomeranians".to_string(), 3);
    goal.insert("akitas".to_string(), 0);
    goal.insert("vizslas".to_string(), 0);
    goal.insert("goldfish".to_string(), 5);
    goal.insert("trees".to_string(), 3);
    goal.insert("cars".to_string(), 2);
    goal.insert("perfumes".to_string(), 1);

    for (i, sue) in sues.iter().enumerate() {
        let mut found_p1 = true;
        let mut found_p2 = true;

        for (prop, &amount) in sue {
            found_p1 &= goal[prop] == amount;

            found_p2 &= match &prop[..] {
                "cats" | "trees" => goal[prop] < amount,
                "pomeranians" | "goldfish" => goal[prop] > amount,
                _ => goal[prop] == amount,
            }
        }

        if found_p1 {
            println!("Part 1: {}", i + 1);
        }

        if found_p2 {
            println!("Part 2: {}", i + 1);
        }
    }
}
