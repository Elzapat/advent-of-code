fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let crabs = file.split(",").map(|x| x.trim().parse::<u64>().unwrap()).collect::<Vec<u64>>();

    {
        let max_h = crabs.iter().max().unwrap();
        let mut total_fuel = u64::MAX;

        for i in 0..(max_h / 2) {
            let mut fuel = 0_u64;
            for crab in &crabs {
                fuel += std::cmp::max(i, *crab) - std::cmp::min(i, *crab);
            }
            if fuel < total_fuel {
                total_fuel = fuel;
            }
        }

        println!("Part 1: {}", total_fuel);
    }

    {
        let max_h = crabs.iter().max().unwrap();
        let mut total_fuel = u64::MAX;

        for i in 0..(max_h / 2) {
            let mut fuel = 0_u64;
            for crab in &crabs {
                let max = std::cmp::max(i, *crab);
                let min = std::cmp::min(i, *crab);
                fuel += sum_int(min, max);
            }
            if fuel < total_fuel {
                total_fuel = fuel;
            }
        }
        println!("Part 2: {}", total_fuel);
    }
}

fn sum_int(low: u64, high: u64) -> u64 {
    let mut sum = 0;
    let mut i = 1;
    for _ in low..high {
        sum += i;
        i += 1;
    }
    sum
}
