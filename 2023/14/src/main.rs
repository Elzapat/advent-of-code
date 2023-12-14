fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn slide_rocks(rocks: &mut Vec<Vec<char>>) {
    for row in rocks.iter_mut() {
        let round_rocks = row
            .iter()
            .enumerate()
            .filter(|(_, r)| **r == 'O')
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        for rock_index in round_rocks {
            for i in (1..=rock_index).rev() {
                if row[i - 1] == 'O' || row[i - 1] == '#' {
                    break;
                }

                row[i - 1] = 'O';
                row[i] = '.';
            }
        }
    }
}

fn main() {
    let input = aoc_base::read_input();
    let lines = input.lines().collect::<Vec<&str>>();

    let mut rocks = Vec::new();

    for i in 0..lines[0].len() {
        let mut row = Vec::new();

        for j in 0..lines.len() {
            row.push(lines[j].chars().nth(i).unwrap());
        }

        rocks.push(row);
    }

    slide_rocks(&mut rocks);

    let mut result_p1 = 0;

    for row in &rocks {
        for (i, rock) in row.iter().enumerate() {
            if *rock == 'O' {
                result_p1 += row.len() - i;
            }
        }
    }

    println!("Part 1: {result_p1}");

    let mut rocks = rocks.clone();
    let mut result_p2 = 0;
    let mut seen = Vec::new();
    let mut cycle_start = 0;

    for _ in 0..1_000_000_000 {
        for _ in 0..4 {
            slide_rocks(&mut rocks);
            rocks.iter_mut().for_each(|row| row.reverse());
            rocks = transpose(rocks);
        }

        if seen.contains(&rocks) {
            cycle_start = seen.iter().position(|r| *r == rocks).unwrap();
            seen.drain(0..cycle_start);
            break;
        }

        seen.push(rocks.clone());
    }

    for row in &seen[(1_000_000_000 - 1 - cycle_start) % (seen.len())] {
        for (i, rock) in row.iter().enumerate() {
            if *rock == 'O' {
                result_p2 += row.len() - i;
            }
        }
    }

    println!("Part 2: {result_p2}");
}
