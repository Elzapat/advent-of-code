use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Comp {
    FlipFlop(Pulse),
    Conjunction(HashMap<String, Pulse>),
    Broadcaster,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

fn main() {
    let input = aoc_base::read_input();

    let mut components: HashMap<String, (Comp, Vec<String>)> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let comp = parts.next().unwrap().to_string();
        let name = comp[1..].to_string();
        let outputs: Vec<String> = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|o| o.to_string())
            .collect();

        if &comp == "broadcaster" {
            components.insert(comp, (Comp::Broadcaster, outputs));
        } else {
            match comp.chars().next().unwrap() {
                '%' => components.insert(name, (Comp::FlipFlop(Pulse::Low), outputs)),
                '&' => components.insert(name, (Comp::Conjunction(HashMap::new()), outputs)),
                _ => unreachable!(),
            };
        }
    }

    for (name, (_, outputs)) in components.clone() {
        for output in outputs {
            if let Some((Comp::Conjunction(inputs), _)) = components.get_mut(&output) {
                inputs.insert(name.clone(), Pulse::Low);
            }
        }
    }

    let mut total_low = 0;
    let mut total_high = 0;

    let mut presses_to_rx = 0;

    // println!("{components:?}");
    for i in 0..1000 {
        let (low_sent, high_sent, found_rx) = propagate_signal(&mut components);

        total_low += low_sent;
        total_high += high_sent;

        if found_rx {}
    }
    // println!("{components:?}");

    println!("Part 1: {}", total_low * total_high);
}

fn propagate_signal(components: &mut HashMap<String, (Comp, Vec<String>)>) -> (u32, u32, bool) {
    let mut low_sent = 1;
    let mut high_sent = 0;
    let mut found_rx = false;

    let (_, outputs) = components["broadcaster"].clone();
    let mut pulses_to_send: Vec<(Pulse, String, String)> = outputs
        .iter()
        .map(|name| (Pulse::Low, name.clone(), "broadcaster".to_string()))
        .collect();

    while !pulses_to_send.is_empty() {
        let pulses = std::mem::take(&mut pulses_to_send);

        for (pulse, name, input_name) in pulses {
            match pulse {
                Pulse::Low => low_sent += 1,
                Pulse::High => high_sent += 1,
            }

            if name == "rx" && pulse == Pulse::Low {
                found_rx = true;
                println!("{pulse:?}");
            }

            match components.get_mut(&name) {
                Some((Comp::FlipFlop(mem), outputs)) => {
                    if let Pulse::Low = pulse {
                        *mem = match *mem {
                            Pulse::Low => Pulse::High,
                            Pulse::High => Pulse::Low,
                        };

                        for output in outputs {
                            pulses_to_send.push((*mem, output.clone(), name.clone()));
                        }
                    }
                }
                Some((Comp::Conjunction(mem), outputs)) => {
                    *mem.entry(input_name.clone()).or_insert(pulse) = pulse;

                    let signal_to_send = match mem.iter().all(|(_, p)| *p == Pulse::High) {
                        true => Pulse::Low,
                        false => Pulse::High,
                    };

                    for output in outputs {
                        pulses_to_send.push((signal_to_send, output.clone(), name.clone()));
                    }
                }
                None => {}
                _ => unreachable!(),
            }
        }
    }

    (low_sent, high_sent, found_rx)
}
