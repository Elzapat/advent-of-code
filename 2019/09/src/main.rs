fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let input = file
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    {
        let height_map = input.clone();
        let mut sum = 0;

        for i in 0..height_map.len() {
            for j in 0..height_map[i].len() {
                let mut low_point = true;

                if (j > 0 && height_map[i][j - 1] <= height_map[i][j]) ||
                    (j + 1 < height_map[i].len() && height_map[i][j + 1] <= height_map[i][j]) ||
                    (i > 0 && height_map[i - 1][j] <= height_map[i][j]) ||
                    (i + 1 < height_map.len() && height_map[i + 1][j] <= height_map[i][j])
                {
                    low_point = false;
                } 

                if low_point {
                    sum += 1 + height_map[i][j];
                }
            }
        }

        println!("Part 1: {}", sum);
    }

    {
        let height_map = input.clone();
        let mut marked_map = vec![vec![false; height_map[0].len()]; height_map.len()];
        let mut basins = vec![];

        for i in 0..height_map.len() {
            for j in 0..height_map[i].len() {
                let mut low_point = true;
                let mut basin = 0;

                if (j > 0 && height_map[i][j - 1] <= height_map[i][j]) ||
                    (j + 1 < height_map[i].len() && height_map[i][j + 1] <= height_map[i][j]) ||
                    (i > 0 && height_map[i - 1][j] <= height_map[i][j]) ||
                    (i + 1 < height_map.len() && height_map[i + 1][j] <= height_map[i][j])
                {
                    low_point = false;
                } 

                if low_point {
                    find_basin(&height_map, &mut marked_map, i, j, &mut basin);
                    basins.push(basin);
                }
            }
        }

        let mut maxs = [0; 3];
        for basin in basins {
            let mut lowest_idx = 0;
            let mut lowest = u32::MAX;

            for i in 0..3 {
                if maxs[i] < lowest {
                    lowest = maxs[i];
                    lowest_idx = i;
                }
            }

            if maxs[lowest_idx] < basin {
                maxs[lowest_idx] = basin;
            }
        }

        println!("Part 2: {}", maxs.iter().map(|x| x + 1).product::<u32>());
    }
}

fn find_basin(height_map: &Vec<Vec<u32>>, marked_map: &mut Vec<Vec<bool>>, i: usize, j: usize, basin: &mut u32) {
    marked_map[i][j] = true;

    if j > 0 && height_map[i][j - 1] < 9 && !marked_map[i][j - 1] {
        *basin += 1;
        find_basin(height_map, marked_map, i, j - 1, basin);
    }

    if j + 1 < height_map[i].len() && height_map[i][j + 1] < 9 && !marked_map[i][j + 1] {
        *basin += 1;
        find_basin(height_map, marked_map, i, j + 1, basin);
    }

    if i > 0 && height_map[i - 1][j] < 9 && !marked_map[i - 1][j] {
        *basin += 1;
        find_basin(height_map, marked_map, i - 1, j, basin);
    }

    if i + 1 < height_map.len() && height_map[i + 1][j] < 9 && !marked_map[i + 1][j] {
        *basin += 1;
        find_basin(height_map, marked_map, i + 1, j, basin);
    } 
}
