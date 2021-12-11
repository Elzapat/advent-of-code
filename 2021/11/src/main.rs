fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let input = file
        .lines()
        .map(|line| line
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
        )
        .collect::<Vec<Vec<u32>>>();

    {
        let mut octopuses = input.clone();
        let mut flashes = 0;

        for _ in 0..100 {
            let mut flashed = vec![vec![false; octopuses[0].len()]; octopuses.len()];

            for i in 0..octopuses.len() {
                for j in 0..octopuses[i].len() {
                    octopuses[i][j] += 1;
                }
            }

            for i in 0..octopuses.len() {
                for j in 0..octopuses[i].len() {
                    if octopuses[i][j] > 9 {
                        flash(&mut octopuses, &mut flashed, i as isize, j as isize);
                    }
                }
            }

            for i in 0..octopuses.len() {
                for j in 0..octopuses[i].len() {
                    if flashed[i][j] {
                        octopuses[i][j] = 0;
                        flashes += 1;
                    }
                }
            }
        }

        println!("Part 1: {}", flashes);
    }

    {
        let mut octopuses = input.clone();
        let mut step = 1;

        loop {
            let mut flashed = vec![vec![false; octopuses[0].len()]; octopuses.len()];

            for i in 0..octopuses.len() {
                for j in 0..octopuses[i].len() {
                    octopuses[i][j] += 1;
                }
            }

            for i in 0..octopuses.len() {
                for j in 0..octopuses[i].len() {
                    if octopuses[i][j] > 9 {
                        flash(&mut octopuses, &mut flashed, i as isize, j as isize);
                    }
                }
            }

            let mut all_flashed = true;

            for i in 0..octopuses.len() {
                for j in 0..octopuses[i].len() {
                    if !flashed[i][j] {
                        all_flashed = false;
                    }
                }
            }

            if all_flashed {
                println!("Part 2: {}", step);
                break;
            }

            for i in 0..octopuses.len() {
                for j in 0..octopuses[i].len() {
                    if flashed[i][j] {
                        octopuses[i][j] = 0;
                    }
                }
            }

            step += 1;
        }
    }
}

fn flash(octopuses: &mut Vec<Vec<u32>>, flashed: &mut Vec<Vec<bool>>, i: isize, j: isize) {
    if i < 0 || j < 0 || i >= octopuses.len() as isize || j >= octopuses.len() as isize {
        return
    }

    octopuses[i as usize][j as usize] += 1;

    if octopuses[i as usize][j as usize] <= 9 || flashed[i as usize][j as usize] {
        return;
    }

    flashed[i as usize][j as usize] = true;

    flash(octopuses, flashed, i - 1, j - 1);
    flash(octopuses, flashed, i - 1, j);
    flash(octopuses, flashed, i - 1, j + 1);
    flash(octopuses, flashed, i, j + 1);
    flash(octopuses, flashed, i + 1, j + 1);
    flash(octopuses, flashed, i + 1, j);
    flash(octopuses, flashed, i + 1, j - 1);
    flash(octopuses, flashed, i, j - 1);
}
