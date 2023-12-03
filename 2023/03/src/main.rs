use regex::Regex;

#[derive(Debug, Default)]
struct Gear {
    pos: (usize, usize),
    n1: u32,
    n2: Option<u32>,
}

fn add_gear(pos: (usize, usize), gears: &mut Vec<Gear>, n: u32) {
    for gear in gears.iter_mut() {
        if gear.pos == pos {
            gear.n2 = Some(n);
            return;
        }
    }

    gears.push(Gear {
        pos,
        n1: n,
        ..Default::default()
    });
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut result = 0;
    let mut gears = Vec::new();

    let re = Regex::new(r"(\d+)").unwrap();

    for i in 0..lines.len() {
        result += re
            .captures_iter(&lines[i])
            .map(|hay| {
                let m = hay.get(0).unwrap();

                if m.start() != 0 {
                    let before = lines[i].chars().nth(m.start() - 1).unwrap();

                    if before == '*' {
                        add_gear(
                            (i, m.start() - 1),
                            &mut gears,
                            m.as_str().parse::<u32>().unwrap(),
                        );
                    }

                    if !before.is_alphanumeric() && before != '.' {
                        return m.as_str().parse::<u32>().unwrap();
                    }
                }

                if m.end() != lines[i].len() {
                    let after = lines[i].chars().nth(m.end()).unwrap();

                    if after == '*' {
                        add_gear((i, m.end()), &mut gears, m.as_str().parse::<u32>().unwrap());
                    }

                    if !after.is_alphanumeric() && after != '.' {
                        return m.as_str().parse::<u32>().unwrap();
                    }
                }

                for j in m.start() as isize - 1..=m.end() as isize {
                    if j < 0 || j > lines[i].len() as isize - 1 {
                        continue;
                    }

                    if i != 0 {
                        let up = lines[i - 1].chars().nth(j as usize).unwrap();

                        if up == '*' {
                            add_gear(
                                (i - 1, j as usize),
                                &mut gears,
                                m.as_str().parse::<u32>().unwrap(),
                            );
                        }

                        if !up.is_alphanumeric() && up != '.' {
                            return m.as_str().parse::<u32>().unwrap();
                        }
                    }

                    if i != lines.len() - 1 {
                        let down = lines[i + 1].chars().nth(j as usize).unwrap();

                        if down == '*' {
                            add_gear(
                                (i + 1, j as usize),
                                &mut gears,
                                m.as_str().parse::<u32>().unwrap(),
                            );
                        }

                        if !down.is_alphanumeric() && down != '.' {
                            return m.as_str().parse::<u32>().unwrap();
                        }
                    }
                }

                0
            })
            .sum::<u32>();
    }

    println!("Part 1: {result}");
    println!(
        "Part 2: {}",
        gears
            .iter()
            .map(|g| {
                if let Some(n2) = g.n2 {
                    g.n1 * n2
                } else {
                    0
                }
            })
            .sum::<u32>()
    )
}
