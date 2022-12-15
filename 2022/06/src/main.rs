use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter, PartialEq, Eq)]
pub enum MarkerType {
    StartOfPacket,
    StartOfMessage,
}

impl MarkerType {
    fn number_unique_characters(&self) -> usize {
        match self {
            MarkerType::StartOfPacket => 4,
            MarkerType::StartOfMessage => 14,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Marker {
    pub ty: MarkerType,
    pub index: usize,
}

#[derive(Default, Debug, Clone)]
pub struct Communication {
    pub markers: Vec<Marker>,
}

impl Communication {
    pub fn from_encoded(comm: impl Into<String>) -> Communication {
        let comm = comm.into();
        let mut markers = vec![];

        for i in 0..comm.len() {
            for marker_type in MarkerType::iter() {
                let n_unique = marker_type.number_unique_characters();

                if i + n_unique < comm.len() && unique_chars(&comm[i..i + n_unique]) {
                    markers.push(Marker {
                        ty: marker_type,
                        index: i + n_unique,
                    });
                }
            }
        }

        Communication { markers }
    }
}

fn unique_chars(string: &str) -> bool {
    let set = string.chars().collect::<HashSet<char>>();
    string.len() == set.len()
}

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
