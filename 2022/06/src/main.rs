use decoder::{Communication, MarkerType};

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();

    let communication = Communication::from_encoded(input);

    for marker in &communication.markers {
        if marker.ty == MarkerType::StartOfPacket {
            println!("Part 1: {}", marker.index);
            break;
        }
    }

    for marker in &communication.markers {
        if marker.ty == MarkerType::StartOfMessage {
            println!("Part 2: {}", marker.index);
            break;
        }
    }

    // Original solution
    /*
    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>();
    const N_UNIQUE_CHARS: usize = 14;

    for line in &lines {
        for i in 0..line.len() - N_UNIQUE_CHARS {
            if line[i..i + N_UNIQUE_CHARS]
                .chars()
                .collect::<std::collections::HashSet<char>>()
                .len()
                == N_UNIQUE_CHARS
            {
                println!("{}", i + N_UNIQUE_CHARS);
                return;
            }
        }
    }
    */
}
