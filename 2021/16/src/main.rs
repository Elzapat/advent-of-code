fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let packet = file.trim().chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap())).collect::<String>();

    {
        let mut versions_sum = 0;
        parse_packet(packet.clone(), 0, &mut versions_sum);
        println!("Part 1: {}", versions_sum);
    }

    {
        // 2869723 2869964
        println!("Part 2: {}", parse_packet(packet, 0, &mut 0).1[0]);
    }
}

fn parse_packet(packet: String, mut i: usize, versions_sum: &mut u64) -> (usize, Vec<u64>) {
    if packet[i..].len() < 32 && u64::from_str_radix(&packet[i..], 2).unwrap() == 0 {
        return (0, vec![]);
    }

    let og_i = i;
    let version = u64::from_str_radix(&packet[i..i+3], 2).unwrap();
    i += 3;
    let type_id = u64::from_str_radix(&packet[i..i+3], 2).unwrap();
    i += 3;

    *versions_sum += version;

    match type_id {
        4 => {
            let mut value = 0;
            loop {
                let current_group = u64::from_str_radix(&packet[i..i+5], 2).unwrap();
                i += 5;
                value = (value << 4) + (current_group & 0b01111);
                if current_group & 0b10000 == 0 {
                    return (i - og_i, vec![value]);
                }
            }
        },
        _ => {
            let length_type_id = u64::from_str_radix(&packet[i..i+1], 2).unwrap();
            i += 1;

            let mut values = vec![];

            match length_type_id {
                0 => {
                    let subpacket_length = usize::from_str_radix(&packet[i..i+15], 2).unwrap();
                    i += 15;
                    let mut progress = 0;
                    while progress < subpacket_length {
                        let (prg, mut value) = parse_packet(packet.to_owned(), i + progress, versions_sum);
                        progress += prg;
                        values.append(&mut value);
                    }
                    i += progress;
                },
                1 => {
                    let subpacket_count = usize::from_str_radix(&packet[i..i+11], 2).unwrap();
                    i += 11;
                    for _ in 0..subpacket_count {
                        let (prg, mut value) = parse_packet(packet.to_owned(), i, versions_sum);
                        i += prg;
                        values.append(&mut value);
                    }
                },
                _ => panic!("Unexpected length type ID: {}", length_type_id),
            }

            match type_id {
                0 => values = vec![values.iter().sum()],
                1 => values = vec![values.iter().product()],
                2 => values = vec![*values.iter().min().unwrap()],
                3 => values = vec![*values.iter().max().unwrap()],
                5 => values = vec![if values[0] > values[1] { 1 } else { 0 }],
                6 => values = vec![if values[0] < values[1] { 1 } else { 0 }],
                7 => values = vec![if values[0] == values[1] { 1 } else { 0 }],
                _ => panic!("Unexpected type ID: {}", type_id),
            }

            return (i - og_i, values);
        }
    }
}
