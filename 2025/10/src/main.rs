use good_lp::{
    solvers::highs::highs, variable, variables, IntoAffineExpression, Solution, SolverModel,
};

#[derive(Default, Debug, Clone, Hash)]
struct Machine {
    indicator: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn main() {
    let input = aoc_base::read_input();
    let mut machines = vec![];

    for line in input.lines() {
        let mut machine = Machine::default();
        for part in line.split_whitespace() {
            let data: String = part.chars().skip(1).take(part.len() - 2).collect();

            match part.chars().next().unwrap() {
                '[' => machine.indicator = data.chars().map(|c| c == '#').collect(),
                '(' => machine.buttons.push(
                    data.split(',')
                        .map(|b| b.parse::<usize>().unwrap())
                        .collect(),
                ),
                '{' => {
                    machine.joltage = data
                        .split(',')
                        .map(|j| j.parse::<usize>().unwrap())
                        .collect()
                }
                _ => unreachable!(),
            }
        }

        machines.push(machine);
    }

    let mut result_p1 = 0;
    for machine in machines.clone() {
        result_p1 += test_buttons_indicator(
            machine.indicator.iter().map(|_| false).collect(),
            machine.indicator,
            machine.buttons,
            0,
        );
    }
    println!("Part 1: {result_p1}");

    let mut result_p2 = 0;
    for machine in machines {
        let mut vars = variables!();
        for i in 0..machine.buttons.len() {
            vars.add(variable().min(0).integer().name(format!("x{i}")));
        }

        let buttons_vars = vars
            .iter_variables_with_def()
            .map(|(v, _)| v)
            .collect::<Vec<_>>();

        let mut problem = highs(vars.minimise(
            (0..machine.buttons.len()).fold(0.into_expression(), |acc, i| acc + buttons_vars[i]),
        ));

        let mut expressions = vec![0.into_expression(); machine.joltage.len()];
        for i in 0..machine.buttons.len() {
            for &x in &machine.buttons[i] {
                expressions[x] += buttons_vars[i];
            }
        }

        for (e, j) in expressions.into_iter().zip(machine.joltage) {
            problem.add_constraint(e.eq(j as f64));
        }

        let solution = problem.solve().unwrap();
        result_p2 += buttons_vars.iter().map(|&v| solution.value(v)).sum::<f64>() as u64;
    }
    println!("Part 2: {result_p2}");
}

#[memoize::memoize]
fn test_buttons_indicator(
    indicator: Vec<bool>,
    goal: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    depth: u32,
) -> u32 {
    if indicator == goal {
        return 0;
    }

    if depth >= 10 {
        return 999;
    }

    let mut presses = vec![];

    for button in &buttons {
        let mut indicator = indicator.clone();

        for &i in button {
            indicator[i] = !indicator[i];
        }

        presses
            .push(test_buttons_indicator(indicator, goal.clone(), buttons.clone(), depth + 1) + 1);
    }

    *presses.iter().min().unwrap()
}
