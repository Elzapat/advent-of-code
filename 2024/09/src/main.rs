fn main() {
    let input = aoc_base::read_input()
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();

    let mut blocks = vec![];
    let mut blocks_p2 = vec![];
    let mut file_ids = 0;

    for i in 0..input.len() {
        for _ in 0..input[i] {
            if i % 2 == 0 {
                blocks.push(file_ids);
                blocks_p2.push(file_ids);
            } else {
                blocks.push(-1);
                blocks_p2.push(-1);
            }
        }

        if i % 2 == 0 {
            file_ids += 1;
        }
    }

    let mut i = blocks.len() - 1;
    while (0..=i).any(|idx| blocks[idx] == -1) {
        if blocks[i] != -1 {
            let idx = blocks.iter().position(|&b| b == -1).unwrap();
            blocks.swap(idx, i);
        }

        i -= 1;
    }

    let mut i = blocks_p2.len() - 1;
    while i != 0 {
        if blocks_p2[i] != -1 {
            let file_id = blocks_p2[i];
            let mut whole_file = vec![];

            while blocks_p2[i] == file_id {
                whole_file.push(i);
                i -= 1;
                if i == 0 {
                    break;
                }
            }
            i += 1;

            let mut free_space = (0, 0);
            let mut found = false;
            let mut currently_free_space = false;
            for j in 0..blocks_p2.len() {
                if free_space.1 - free_space.0 + 1 >= whole_file.len() && free_space != (0, 0) {
                    found = true;
                    break;
                }

                if blocks_p2[j] == -1 && currently_free_space {
                    free_space.1 = j;
                } else if blocks_p2[j] != -1 && currently_free_space {
                    free_space = (0, 0);
                    currently_free_space = false;
                } else if blocks_p2[j] == -1 && !currently_free_space {
                    free_space = (j, j);
                    currently_free_space = true;
                }
            }

            if found {
                for j in free_space.0..=free_space.1 {
                    if j >= whole_file[j - free_space.0] {
                        break;
                    }
                    blocks_p2.swap(j, whole_file[j - free_space.0]);
                }
            }
        }

        i -= 1;
    }

    println!(
        "Part 1: {}",
        blocks
            .iter()
            .enumerate()
            .filter(|(_, &b)| b != -1)
            .map(|(i, b)| i as i64 * b)
            .sum::<i64>()
    );

    println!(
        "Part 2: {}",
        blocks_p2
            .iter()
            .enumerate()
            .filter(|(_, &b)| b != -1)
            .map(|(i, b)| i as i64 * b)
            .sum::<i64>()
    );
}
