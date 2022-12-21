use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Yell(i64),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Monkey {
    op: Op,
    depends: Option<(String, String)>,
}

impl Monkey {
    fn depends_on_humn(&self, monkeys: &HashMap<String, Monkey>) -> bool {
        match self.depends.as_ref() {
            Some((m1, m2)) => {
                if m1 == "humn" || m2 == "humn" {
                    true
                } else {
                    monkeys[m1].depends_on_humn(monkeys) || monkeys[m2].depends_on_humn(monkeys)
                }
            }
            None => false,
        }
    }
}

fn resolve_number(monkey: &Monkey, monkeys: &HashMap<String, Monkey>) -> i64 {
    // dbg!(monkey);
    match monkey.op {
        Op::Yell(n) => n,
        _ => {
            let (m1, m2) = monkey.depends.as_ref().unwrap();

            match monkey.op {
                Op::Add => {
                    resolve_number(&monkeys[m1], monkeys) + resolve_number(&monkeys[m2], monkeys)
                }
                Op::Sub => {
                    resolve_number(&monkeys[m1], monkeys) - resolve_number(&monkeys[m2], monkeys)
                }
                Op::Div => {
                    resolve_number(&monkeys[m1], monkeys) / resolve_number(&monkeys[m2], monkeys)
                }
                Op::Mul => {
                    resolve_number(&monkeys[m1], monkeys) * resolve_number(&monkeys[m2], monkeys)
                }
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")).unwrap();
    let mut monkeys = HashMap::<String, Monkey>::new();

    let monkey_regex = Regex::new(r"^(....): (.*)$").unwrap();
    let op_regex = Regex::new(r"(....) (.) (....)").unwrap();
    let yell_regex = Regex::new(r"(\d)").unwrap();

    for line in input.lines().map(|l| l.to_owned()) {
        let caps = monkey_regex.captures(&line).unwrap();
        let name = caps[1].to_string();

        if yell_regex.is_match(&caps[2]) {
            monkeys.insert(
                name,
                Monkey {
                    op: Op::Yell(caps[2].parse().unwrap()),
                    depends: None,
                },
            );
        } else if op_regex.is_match(&caps[2]) {
            let caps = op_regex.captures(&caps[2]).unwrap();
            let (m1, m2) = (caps[1].to_string(), caps[3].to_string());
            let op = match &caps[2] {
                "+" => Op::Add,
                "-" => Op::Sub,
                "/" => Op::Div,
                "*" => Op::Mul,
                _ => unreachable!(),
            };

            monkeys.insert(
                name,
                Monkey {
                    op,
                    depends: Some((m1, m2)),
                },
            );
        }
    }

    println!("Part 1: {}", resolve_number(&monkeys["root"], &monkeys));

    let (m1, m2) = monkeys["root"].depends.as_ref().unwrap();
    let (base, to_match_monkey) = if monkeys[m1].depends_on_humn(&monkeys) {
        (resolve_number(&monkeys[m2], &monkeys), m1.to_owned())
    } else {
        (resolve_number(&monkeys[m1], &monkeys), m2.to_owned())
    };

    let mut result = 0;
    let mut human_yell = 3592056845080;
    let mut i = 0;
    while result != base && i <= 12 {
        human_yell += 1;
        monkeys.get_mut("humn").unwrap().op = Op::Yell(human_yell);
        result = resolve_number(&monkeys[&to_match_monkey], &monkeys);
        i += 1;
    }

    println!("Part 2: {}", human_yell);
}
