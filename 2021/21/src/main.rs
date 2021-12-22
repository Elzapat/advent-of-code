use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Player {
    score: u32,
    position: u32,
}

impl Player {
    fn advance(&mut self, steps: u32) {
        self.position = (self.position + steps - 1) % 10 + 1;
        self.score += self.position;
    }
}

#[derive(Debug, Copy, Clone)]
struct DDice {
    number: u32,
    times_rolled: u32,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct State {
    p1: Player,
    p2: Player,
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = file.lines();

    let p1 = Player {
        score: 0,
        position: lines.next().unwrap().chars().last().unwrap().to_digit(10).unwrap(),
    };

    let p2 = Player {
        score: 0,
        position: lines.next().unwrap().chars().last().unwrap().to_digit(10).unwrap(),
    };

    let ddice = DDice {
        number: 1,
        times_rolled: 0,
    };

    println!("Part 1: {}", part1(p1, p2, ddice));
    println!("Part 2: {}", part2(p1, p2));
}

fn part2(p1: Player, p2: Player) -> u64 {
    let mut cache = HashMap::new();

    let wins = turn(State { p1, p2 }, &mut cache);
    wins.0.max(wins.1)
}

const ROLLS: [u32; 27] = [3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9];

fn turn(state: State, cache: &mut HashMap<State, (u64, u64)>) -> (u64, u64) {
    if let Some(value) = cache.get(&state) {
        return *value;
    }

    let og_state = state;
    let mut wins = (0, 0);

    for roll in ROLLS {
        let mut state = state.clone();
        state.p1.advance(roll);

        if state.p1.score >= 21 {
            cache.insert(og_state, wins);
            wins.0 += 1;
            continue;
        }

        let tot_wins = turn(State { p1: state.p2, p2: state.p1 }, cache);
        wins.0 += tot_wins.1;
        wins.1 += tot_wins.0;
    }

    cache.insert(og_state, wins);
    wins
}

fn part1(mut p1: Player, mut p2: Player, mut ddice: DDice) -> u32 {
    let losing_player_score = loop {
        let rolls = [ddice.number, ddice.number + 1, ddice.number + 2];
        for _ in 0..3 {
            ddice.number += 1;
            if ddice.number > 100 {
                ddice.number = 1;
            }
        }
        ddice.times_rolled += 3;

        for roll in rolls {
            p1.position += roll;
            while p1.position > 10 {
                p1.position -= 10;
            }
        }
        p1.score += p1.position;

        if p1.score >= 1000 {
            break p2.score;
        }

        let rolls = [ddice.number, ddice.number + 1, ddice.number + 2];
        for _ in 0..3 {
            ddice.number += 1;
            if ddice.number > 100 {
                ddice.number = 1;
            }
        }
        ddice.times_rolled += 3;

        for roll in rolls {
            p2.position += roll;
            while p2.position > 10 {
                p2.position -= 10;
            }
        }
        p2.score += p2.position;

        if p2.score >= 1000 {
            break p1.score;
        }
    };

    losing_player_score * ddice.times_rolled
}

#[test]
fn player_advance() {
    let mut player = Player {
        position: 9,
        score: 7,
    };

    player.advance(3);

    assert_eq!(player.position, 2);
    assert_eq!(player.score, 7 + 2);

    player.advance(2);

    assert_eq!(player.position, 4);
    assert_eq!(player.score, 7 + 2 + 4);
}
