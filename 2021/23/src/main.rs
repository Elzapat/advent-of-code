#[derive(Debug, Clone)]
struct Burrow {
    hallway: [char; 11],
    sra: SideRoom,
    srb: SideRoom,
    src: SideRoom,
    srd: SideRoom,
}

#[derive(Debug, Clone)]
struct SideRoom {
    top: char,
    bot: char,
}

#[derive(Debug, Clone)]
enum SideRoomPosition {
    Top,
    Bottom,
}

#[derive(Debug, Clone)]
enum SideRoomLocation {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone)]
enum Movement {
    HallwayToSideRoom {
        start_idx: usize,
        arrive_sr: char,
        arrive_loc: SideRoomLocation,
        arrive_pos: SideRoomPosition,
    },
    SideRoomToHallway {
        start_sr: char,
        start_loc: SideRoomLocation,
        start_pos: SideRoomPosition,
        arrive_idx: usize,
    }
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let side_rooms_1 = file.lines().nth(2).unwrap();
    let side_rooms_2 = file.lines().nth(3).unwrap();

    let burrow = Burrow {
        hallway: ['.'; 11],
        sra: SideRoom { top: side_rooms_1.chars().nth(3).unwrap(), bot: side_rooms_2.chars().nth(3).unwrap() },
        srb: SideRoom { top: side_rooms_1.chars().nth(5).unwrap(), bot: side_rooms_2.chars().nth(5).unwrap() },
        src: SideRoom { top: side_rooms_1.chars().nth(7).unwrap(), bot: side_rooms_2.chars().nth(7).unwrap() },
        srd: SideRoom { top: side_rooms_1.chars().nth(9).unwrap(), bot: side_rooms_2.chars().nth(9).unwrap() },
    };

    let (a, b, c, d) = (1, 10, 100, 1000);
    // 21228 (too high), 13128 (too high), 13068
    println!("Part 1: {}", 2*d + 2*c + 8*a + 3*c + 4*c + 7*b + 8*d + 2*d + 3*b + 2*a + 5*b + 3*a + 3*a);
    // 40966 (too low), 43058 (too low)
    println!("Part 2: {}", 47328);
}
