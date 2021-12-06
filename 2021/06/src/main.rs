#[derive(Clone)]
struct Fish {
    time: u32,
    marked: bool,
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let fishes = file
        .split(",")
        .map(|n| Fish { time: n.trim().parse::<u32>().unwrap(), marked: false })
        .collect::<Vec<Fish>>();

    let fishes_part1 = {
        let mut fishes = fishes.clone();

        for _ in 0..80 {
            for mut i in 0..fishes.len() {
                if fishes[i].marked {
                    continue;
                }

                match fishes[i].time {
                    0 => {
                        fishes.push(Fish { time: 8, marked: false });
                        fishes[i].marked = true;
                        fishes[i].time = 6;
                        i = 0;
                    },
                    _ => fishes[i].time -= 1,
                }
            }

            for fish in fishes.iter_mut() {
                fish.marked = false;
            }
        }

        println!("Part 1: {}", fishes.len());
        fishes.len()
    };

    /*
    {
        // Compute exponential growth from example
        let n: f64 = 5.0;
        let k = (26984457539.0 / n).ln() / 256.0;
        println!("{}", k);
        let ex_value = n * (256.0 * k).exp();
        println!("example value: {}", ex_value);
        println!("example value: {}", n * ((26984457539.0_f64 / n).powf(1.0 / 256.0)).powf(256.0));

        // Compute exponential growth for my input?
        let k = (fishes_part1 as f64 / fishes.len() as f64).ln() / 80.0;
        let k_noam = (1653559299811.0 / fishes.len() as f64).ln() / 256.0;

        println!("{}", k);
        println!("{}", fishes_part1);
        println!("Part 2 noam: {}", fishes.len() as f64 * (256.0 * k_noam).exp());
        println!("Part 2: {}", fishes.len() as f64 * (256.0 * k).exp());
        println!("Part 2: {}", fishes.len() as f64 * ((fishes_part1 as f64 / fishes.len() as f64).powf(1.0 / 80.0)).powf(256.0));
    }
    */
    
    {
        let fishes_input = fishes.clone();
        let mut fishes = [0_u64; 9];
        for fish in &fishes_input {
            fishes[fish.time as usize] += 1;
        }

        for _ in 0..256 {
            let fishes_hatch = fishes[0];
            fishes[0] = fishes[1];
            fishes[1] = fishes[2];
            fishes[2] = fishes[3];
            fishes[3] = fishes[4];
            fishes[4] = fishes[5];
            fishes[5] = fishes[6];
            fishes[6] = fishes[7] + fishes_hatch;
            fishes[7] = fishes[8];
            fishes[8] = fishes_hatch;
        }

        let mut sum = 0;
        for i in 0..=8 {
            sum += fishes[i];
        }

        println!("Part 2: {}", sum);
    }
}
