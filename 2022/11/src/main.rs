use num::integer::lcm;

#[derive(Debug, Copy, Clone)]
enum Op {
    Plus(u128),
    Mul(u128),
    PlusOld,
    MulOld,
}

#[derive(Debug, Default, Copy, Clone)]
struct Item {
    worry: u128,
}

#[derive(Debug, Clone)]
struct Monkey {
    n_inspected: u128,
    items: Vec<Item>,
    operation: Op,
    test: u128,
    if_true: u128,
    if_false: u128,
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>();

    const N_MONKEYS: usize = 8;
    let mut monkeys: Vec<Monkey> = Vec::with_capacity(N_MONKEYS);

    for current_monkey in 0..N_MONKEYS {
        monkeys.push(Monkey {
            items: lines[current_monkey * 7 + 1][17..]
                .split(',')
                .map(|item| Item {
                    worry: item.trim().parse::<u128>().unwrap(),
                })
                .collect::<Vec<Item>>(),
            n_inspected: 0,
            operation: {
                let n = &lines[current_monkey * 7 + 2][25..];
                if n == "old" {
                    match lines[current_monkey * 7 + 2].chars().nth(23).unwrap() {
                        '*' => Op::MulOld,
                        '+' => Op::PlusOld,
                        _ => unreachable!(),
                    }
                } else {
                    let n = n.parse::<u128>().unwrap();
                    match lines[current_monkey * 7 + 2].chars().nth(23).unwrap() {
                        '*' => Op::Mul(n),
                        '+' => Op::Plus(n),
                        _ => unreachable!(),
                    }
                }
            },
            test: lines[current_monkey * 7 + 3][21..].parse::<u128>().unwrap(),
            if_true: lines[current_monkey * 7 + 4]
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as u128,
            if_false: lines[current_monkey * 7 + 5]
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as u128,
        });
    }

    println!("Part 1: {}", solve(monkeys.clone(), 3, 20));

    let mut divisor = monkeys[0].test;
    for monkey in monkeys.iter().skip(1) {
        divisor = lcm(divisor, monkey.test);
    }

    println!("Part 2: {}", solve(monkeys, divisor, 10_000));
}

fn solve(mut monkeys: Vec<Monkey>, divisor: u128, rounds: usize) -> u128 {
    for _ in 0..rounds {
        for current_monkey in 0..monkeys.len() {
            let items = monkeys[current_monkey]
                .items
                .drain(..)
                .collect::<Vec<Item>>();

            for mut item in items {
                monkeys[current_monkey].n_inspected += 1;
                item.worry = match monkeys[current_monkey].operation {
                    Op::Plus(n) => item.worry + n,
                    Op::Mul(n) => item.worry * n,
                    Op::PlusOld => item.worry + item.worry,
                    Op::MulOld => item.worry * item.worry,
                };

                let other = if item.worry % monkeys[current_monkey].test == 0 {
                    monkeys[current_monkey].if_true
                } else {
                    monkeys[current_monkey].if_false
                } as usize;

                item.worry %= divisor;

                monkeys[other].items.push(item);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m1.n_inspected.cmp(&m2.n_inspected));
    monkeys[monkeys.len() - 1].n_inspected * monkeys[monkeys.len() - 2].n_inspected
}
