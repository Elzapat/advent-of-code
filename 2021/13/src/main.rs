fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut paper = [[false; 2000]; 2000];
    let mut folds = vec![];

    for line in input.lines() {
        if line.contains("fold") {
            let mut parts = line.split("=");
            let axis = parts.next().unwrap().chars().last().unwrap();
            let number = parts.next().unwrap().parse::<usize>().unwrap();
            folds.push((axis, number));
        } else if line.contains(",") {
            let mut coords = line.split(",");
            let x = coords.next().unwrap().parse::<usize>().unwrap();
            let y = coords.next().unwrap().parse::<usize>().unwrap();
            paper[x][y] = true;
        }
    }

    {
        let mut paper = paper.clone();
        let folds = folds.clone();

        if folds[0].0 == 'x' {
            for i in (folds[0].1 + 1)..paper.len() {
                for j in 0..paper[i].len() {
                    if paper[i][j] {
                        paper[folds[0].1 - (i - folds[0].1)][j] = true;
                        paper[i][j] = false;
                    }
                }
            }
        } else {
            for i in 0..paper.len() {
                for j in (folds[0].1 + 1)..paper[i].len() {
                    if paper[i][j] {
                        paper[i][folds[0].1 - (j - folds[0].1)] = true;
                        paper[i][j] = false;
                    }
                }
            }
        }

        println!("Part 1: {}", paper.iter().fold(0, |acc, row| acc + row.iter().fold(0, |acc, p| acc + *p as u32)));
    }

    {
        for fold in folds {
            if fold.0 == 'x' {
                for i in (fold.1 + 1)..paper.len() {
                    for j in 0..paper[i].len() {
                        if paper[i][j] {
                            paper[ fold.1 - (i - fold.1)][j] = true;
                            paper[i][j] = false;
                        }
                    }
                }
            } else {
                for i in 0..paper.len() {
                    for j in (fold.1 + 1)..paper[i].len() {
                        if paper[i][j] {
                            paper[i][fold.1 - (j - fold.1)] = true;
                            paper[i][j] = false;
                        }
                    }
                }
            }
        }

        println!("Part 2:");
        for i in 0..6 {
            for j in 0..39 {
                print!("{}", if paper[j][i] { '#' } else { '.' });
            }
            println!("");
        }
    }
}
