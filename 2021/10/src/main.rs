#[derive(Clone, Default, Debug)]
struct DelimiterStates {
    parenthesis: Delim,
    brackets: Delim,
    braces: Delim,
    angle_brackets: Delim,
}

#[derive(Clone, Default, Debug)]
struct Delim {
    state: u32,
    open_pos: Vec<usize>,
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let input = file.lines().collect::<Vec<&str>>();

    {
        let lines = input.clone();
        let mut output = 0;

        for line in &lines {
            let _ = all_closed_delimiters(line, &mut output);
        }

        println!("Part 1: {}", output);
    }

    {
        let lines = input.clone();
        let mut scores = vec![];

        for line in &lines {
            let score = complete_unfinished_line(line);

            if score != 0 {
                scores.push(score);
            }
        }

        scores.sort();
        println!("Part 2: {}", scores[scores.len() / 2]);
    }
}

fn complete_unfinished_line(line: &str) -> u64 {
    let mut line = line.to_owned();
    let mut states = DelimiterStates::default();

    for (i, delim) in line.chars().enumerate() {
        match delim {
            '(' => {
                states.parenthesis.state += 1;
                states.parenthesis.open_pos.push(i);
            },
            '{' => {
                states.braces.state += 1;
                states.braces.open_pos.push(i);
            },
            '[' => {
                states.brackets.state += 1;
                states.brackets.open_pos.push(i);
            },
            '<' => {
                states.angle_brackets.state += 1;
                states.angle_brackets.open_pos.push(i);
            },
            ')' => if states.parenthesis.state == 0 ||
                !all_closed_delimiters(&line[states.parenthesis.open_pos.pop().unwrap() + 1..i], &mut 0)
            {
                return 0;
            } else {
                states.parenthesis.state -= 1;
            },
            '}' => if states.braces.state == 0 ||
                !all_closed_delimiters(&line[states.braces.open_pos.pop().unwrap() + 1..i], &mut 0)
            {
                return 0;
            } else {
                states.braces.state -= 1;
            },
            ']' => if states.brackets.state == 0 ||
                !all_closed_delimiters(&line[states.brackets.open_pos.pop().unwrap() + 1..i], &mut 0)
            {
                return 0;
            } else {
                states.brackets.state -= 1;
            },
            '>' => if states.angle_brackets.state == 0 ||
                !all_closed_delimiters(&line[states.angle_brackets.open_pos.pop().unwrap() + 1..i], &mut 0)
            {
                return 0;
            } else {
                states.angle_brackets.state -= 1;
            },
            _ => panic!("Unhandled character: {}", delim),
        }
    }

    let mut score = 0_u64;

    loop {
        let mut earlier_pos = (-1, ' ');

        if states.parenthesis.open_pos.len() > 0 {
            if states.parenthesis.open_pos[states.parenthesis.open_pos.len() - 1] as isize > earlier_pos.0 {
                earlier_pos = (states.parenthesis.open_pos[states.parenthesis.open_pos.len() - 1] as isize, '(');
            }
        }
        if states.braces.open_pos.len() > 0 {
            if states.braces.open_pos[states.braces.open_pos.len() - 1] as isize > earlier_pos.0 {
                earlier_pos = (states.braces.open_pos[states.braces.open_pos.len() - 1] as isize, '{');
            }
        }
        if states.brackets.open_pos.len() > 0 {
            if states.brackets.open_pos[states.brackets.open_pos.len() - 1] as isize > earlier_pos.0 {
                earlier_pos = (states.brackets.open_pos[states.brackets.open_pos.len() - 1] as isize, '[');
            }
        }
        if states.angle_brackets.open_pos.len() > 0 {
            if states.angle_brackets.open_pos[states.angle_brackets.open_pos.len() - 1] as isize > earlier_pos.0 {
                earlier_pos = (states.angle_brackets.open_pos[states.angle_brackets.open_pos.len() - 1] as isize, '<');
            }
        }

        if let (-1, ' ') = earlier_pos {
            break;
        }

        match earlier_pos {
            (_, '(') => {
                score = score * 5 + 1;
                states.parenthesis.open_pos.pop().unwrap();
                line.push(')');
            },
            (_, '{') => {
                score = score * 5 + 3;
                states.braces.open_pos.pop().unwrap();
                line.push('}');
            },
            (_, '[') => {
                score = score * 5 + 2;
                states.brackets.open_pos.pop().unwrap();
                line.push(']');
            },
            (_, '<') => {
                score = score * 5 + 4;
                states.angle_brackets.open_pos.pop().unwrap();
                line.push('>');
            },
            _ => panic!("wtf is this: {:?}", earlier_pos),
        }
    }

    score
}

fn all_closed_delimiters(line: &str, output: &mut u32) -> bool {
    let mut states = DelimiterStates::default();

    for (i, delim) in line.chars().enumerate() {
        match delim {
            '(' => {
                states.parenthesis.state += 1;
                states.parenthesis.open_pos.push(i);
            },
            '{' => {
                states.braces.state += 1;
                states.braces.open_pos.push(i);
            },
            '[' => {
                states.brackets.state += 1;
                states.brackets.open_pos.push(i);
            },
            '<' => {
                states.angle_brackets.state += 1;
                states.angle_brackets.open_pos.push(i);
            },
            ')' => if states.parenthesis.state == 0 ||
                !all_closed_delimiters(&line[states.parenthesis.open_pos.pop().unwrap() + 1..i], output)
            {
                *output += 3;
                return false;
            } else {
                states.parenthesis.state -= 1;
            },
            '}' => if states.braces.state == 0 ||
                !all_closed_delimiters(&line[states.braces.open_pos.pop().unwrap() + 1..i], output)
            {
                *output += 1197;
                return false;
            } else {
                states.braces.state -= 1;
            },
            ']' => if states.brackets.state == 0 ||
                !all_closed_delimiters(&line[states.brackets.open_pos.pop().unwrap() + 1..i], output)
            {
                *output += 57;
                return false;
            } else {
                states.brackets.state -= 1;
            },
            '>' => if states.angle_brackets.state == 0 ||
                !all_closed_delimiters(&line[states.angle_brackets.open_pos.pop().unwrap() + 1..i], output)
            {
                *output += 25137;
                return false;
            } else {
                states.angle_brackets.state -= 1;
            },
                _ => panic!("Unhandled character: {}", delim),
        }
    }

    states.parenthesis.state == 0 &&
        states.braces.state == 0 &&
        states.brackets.state == 0 &&
        states.angle_brackets.state == 0
}

#[test]
fn empty_line_all_closed() {
    assert!(all_closed_delimiters("", &mut 0))
}
