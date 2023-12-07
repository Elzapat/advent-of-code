#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, PartialEq)]
struct Hand {
    cards: Vec<u32>,
    hand_type: HandType,
    bid: u32,
}

fn find_hand_type(card: u32, cards: Vec<u32>, current_hand_type: HandType) -> HandType {
    let mut count = cards.iter().filter(|c| **c == card).count();

    if card != 0 {
        let jokers_count = cards.iter().filter(|c| **c == 0).count();
        count += jokers_count;
    }

    match count {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if current_hand_type == HandType::OnePair {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            if current_hand_type == HandType::ThreeOfAKind {
                HandType::FullHouse
            } else if current_hand_type == HandType::OnePair {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        1 => HandType::HighCard,
        _ => unreachable!("Unexpected count {count}"),
    }
}

fn find_best_hand_type(cards: &Vec<u32>) -> HandType {
    let mut hand_type = HandType::HighCard;
    let mut cards_seen = Vec::new();

    for card in cards {
        if cards_seen.contains(card) {
            continue;
        }

        let temp_hand_type = find_hand_type(*card, cards.clone(), hand_type);

        if temp_hand_type > hand_type {
            hand_type = temp_hand_type;
        }

        cards_seen.push(*card);
    }

    hand_type
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let mut hands = Vec::<Hand>::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let hand = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<u32>().unwrap();

        let cards = hand
            .chars()
            .map(|c| match c {
                'T' => 10,
                'J' => 0,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => c.to_digit(10).unwrap(),
            })
            .collect::<Vec<u32>>();

        let all_jokers_indices = cards
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == 0)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        let hand_type = if all_jokers_indices.len() == 1 {
            let joker_idx = all_jokers_indices[0];
            let mut best_hand_type = HandType::HighCard;

            for replacement_card in 2..=14 {
                let mut new_cards = cards.clone();
                new_cards[joker_idx] = replacement_card;

                let hand_type = find_best_hand_type(&new_cards);

                if hand_type > best_hand_type {
                    best_hand_type = hand_type;
                }
            }

            best_hand_type
        } else if all_jokers_indices.len() == 2 {
            let (joker1_idx, joker2_idx) = (all_jokers_indices[0], all_jokers_indices[1]);
            let mut best_hand_type = HandType::HighCard;
            let mut new_cards = cards.clone();

            for replacement1 in 2..=14 {
                new_cards[joker1_idx] = replacement1;

                for replacement2 in 2..=14 {
                    new_cards[joker2_idx] = replacement2;

                    let hand_type = find_best_hand_type(&new_cards);

                    if hand_type > best_hand_type {
                        best_hand_type = hand_type;
                    }
                }
            }

            best_hand_type
        } else {
            find_best_hand_type(&cards)
        };

        hands.push(Hand {
            hand_type,
            cards,
            bid,
        });
    }

    hands.sort_by(|hand1, hand2| {
        if hand1.hand_type == hand2.hand_type {
            for i in 0..hand1.cards.len() {
                if hand1.cards[i] != hand2.cards[i] {
                    return hand1.cards[i].cmp(&hand2.cards[i]);
                }
            }

            unreachable!();
        } else {
            hand1.hand_type.cmp(&hand2.hand_type)
        }
    });

    println!(
        "Part 2: {}",
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i as u32 + 1))
            .sum::<u32>()
    );
}
