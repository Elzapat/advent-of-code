use regex::Regex;

#[derive(Debug, Default)]
struct Game {
    green: u32,
    blue: u32,
    red: u32,
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();

    let mut result_p1 = 0;
    let mut result_p2 = 0;

    let re_green = Regex::new(r"(\d*) green").unwrap();
    let re_blue = Regex::new(r"(\d*) blue").unwrap();
    let re_red = Regex::new(r"(\d*) red").unwrap();

    for (i, line) in input.lines().enumerate() {
        let mut game = Game::default();

        re_green
            .captures_iter(line)
            .for_each(|hay| game.green = u32::max(game.green, hay[1].parse::<u32>().unwrap()));

        re_blue
            .captures_iter(line)
            .for_each(|hay| game.blue = u32::max(game.blue, hay[1].parse::<u32>().unwrap()));

        re_red
            .captures_iter(line)
            .for_each(|hay| game.red = u32::max(game.red, hay[1].parse::<u32>().unwrap()));

        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            result_p1 += i + 1;
        }

        result_p2 += game.red * game.green * game.blue;
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}
