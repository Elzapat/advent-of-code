use std::collections::HashMap;

fn add_cards(n: u32, cards: &HashMap<u32, u32>) -> u32 {
    let mut result = 0;

    for i in 1..=cards[&n] {
        result += add_cards(n + i, cards);
    }

    result + 1
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();

    let mut result = 0;
    let mut scratchcards_wins = HashMap::<u32, u32>::new();

    for line in input.lines() {
        let start = line.find(':').unwrap() + 2;
        // let card_number = line[start - 3..start - 2].trim().parse::<u32>().unwrap();
        let card_number = line[start - 5..start - 2].trim().parse::<u32>().unwrap();
        let mut winning = Vec::new();
        let mut numbers = Vec::new();
        let mut winning_numbers = true;

        let mut i = start;
        while i < line.len() - 1 {
            let number = &line[i..i + 2];

            if number.contains('|') {
                i += 2;
                winning_numbers = false;
                continue;
            }

            let number = number.trim().parse::<u32>().unwrap();

            if winning_numbers {
                winning.push(number);
            } else {
                numbers.push(number);
            }

            i += 3;
        }

        let mut wins: u32 = 0;
        for number in &numbers {
            if winning.contains(number) {
                wins += 1;
            }
        }

        if wins > 0 {
            result += 2_u32.pow(wins - 1);
        }

        scratchcards_wins.insert(card_number, wins);
    }

    let mut result_p2 = 0;
    for (card_number, _) in scratchcards_wins.iter() {
        result_p2 += add_cards(*card_number, &scratchcards_wins);
    }

    println!("Part 1: {result}");
    println!("Part 2: {}", result_p2);
}
