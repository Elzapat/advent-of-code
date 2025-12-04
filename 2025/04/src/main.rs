fn main() {
    let mut input = aoc_base::read_input_grid();

    let mut result_p1 = 0;

    for r in 0..input.len() {
        for c in 0..input[r].len() {
            if input[r][c] != '@' { 
                continue;
            }

            let p = aoc_base::Point { r, c };
            let neighbours = p.get_8_neighbours(input.len(), input[r].len());

            if neighbours.iter().fold(0, |acc, n| if input[n.r][n.c] == '@' { acc + 1 } else { acc }) < 4 {
                result_p1 += 1;
            }
        }
    }

    println!("Part 1: {result_p1}");

    let mut result_p2 = 0;

    loop {
        let mut removed = 0;

        for r in 0..input.len() {
            for c in 0..input[r].len() {
                if input[r][c] != '@' { 
                    continue;
                }

                let p = aoc_base::Point { r, c };
                let neighbours = p.get_8_neighbours(input.len(), input[r].len());

                if neighbours.iter().fold(0, |acc, n| if input[n.r][n.c] == '@' { acc + 1 } else { acc }) < 4 {
                    input[r][c] = '.';
                    removed += 1;
                }
            }
        }

        result_p2 += removed;

        if removed == 0 {
            break;
        }
    }

    println!("Part 2: {result_p2}");
}
