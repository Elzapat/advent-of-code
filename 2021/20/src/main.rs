const IMAGE_SIZE: usize = 500;

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let lines = file.lines().collect::<Vec<&str>>();

    let iea = lines[0].chars().map(|c| c == '#').collect::<Vec<bool>>();
    let mut input_image = vec![];

    for line in &lines[2..] {
        input_image.push(line.chars().map(|c| c == '#').collect::<Vec<bool>>());
    }

    let image_width = input_image[0].len();
    for line in input_image.iter_mut() {
        for _ in 0..((IMAGE_SIZE - image_width) / 2) {
            line.insert(0, false);
            line.push(false);
        }
    }

    for _ in 0..((IMAGE_SIZE - input_image.len()) / 2) {
        input_image.insert(0, vec![false; IMAGE_SIZE]);
        input_image.push(vec![false; IMAGE_SIZE]);
    }

    println!("Part 1: {}", enhance_image(input_image.clone(), iea.clone(), 2).iter().map(|line| line.iter().map(|pix| *pix as u32).sum::<u32>()).sum::<u32>());
    println!("Part 2: {}", enhance_image(input_image.clone(), iea.clone(), 50).iter().map(|line| line.iter().map(|pix| *pix as u32).sum::<u32>()).sum::<u32>());
}

fn enhance_image(mut input_image: Vec<Vec<bool>>, iea: Vec<bool>, steps: usize) -> Vec<Vec<bool>> {
    const STEPS: isize = 50;

    for _ in 0..steps {
        let mut output_image = vec![vec![false; IMAGE_SIZE]; IMAGE_SIZE];

        for i in 1..(input_image.len() + 1) {
            for j in 1..(input_image[i - 1].len() + 1) {
                let mut iea_idx = 0_usize;

                for k in i-1..=i+1 {
                    for l in j-1..=j+1 {
                        match input_image.get(k) {
                            Some(line) => match line.get(l) {
                                Some(pix) => iea_idx = (iea_idx << 1) + *pix as usize,
                                None => iea_idx <<= 1,
                            },
                            None => iea_idx <<= 1,
                        }
                    }
                }

                output_image[i - 1][j - 1] = iea[iea_idx];
            }
        }

        input_image = output_image;
    }

    for line in input_image.iter_mut() {
        for _ in 0..STEPS*2 {
            line.pop();
        }
    }

    for _ in 0..STEPS*2 {
        input_image.pop();
    }

    input_image
}
