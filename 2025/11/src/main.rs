use std::collections::HashMap;

fn main() {
    let input = aoc_base::read_input();
    let mut devices = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        let device = parts.next().unwrap()[0..3].to_string();
        let outputs = parts.map(|o| o.to_string()).collect::<Vec<_>>();

        devices.insert(device, outputs);
    }

    println!(
        "Part 1: {}",
        search_paths_p1(devices.clone(), "you".to_string())
    );

    println!(
        "Part 2: {}",
        search_paths_p2(devices, "svr".to_string(), false, false)
    );
}

#[memoize::memoize(Ignore: devices)]
fn search_paths_p1(devices: HashMap<String, Vec<String>>, current_device: String) -> u64 {
    if current_device == "out" {
        return 1;
    }

    devices[&current_device]
        .iter()
        .map(|output| search_paths_p1(devices.clone(), output.clone()))
        .sum::<u64>()
}

#[memoize::memoize(Ignore: devices)]
fn search_paths_p2(
    devices: HashMap<String, Vec<String>>,
    current_device: String,
    mut found_fft: bool,
    mut found_dac: bool,
) -> u64 {
    match &current_device[..] {
        "out" => {
            if found_fft && found_dac {
                return 1;
            } else {
                return 0;
            }
        }
        "fft" => found_fft = true,
        "dac" => found_dac = true,
        _ => {}
    }

    devices[&current_device]
        .iter()
        .map(|output| search_paths_p2(devices.clone(), output.clone(), found_fft, found_dac))
        .sum::<u64>()
}
