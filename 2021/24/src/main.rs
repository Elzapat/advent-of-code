use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Op {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl From<&str> for Op {
    fn from(string: &str) -> Self {
        match string {
            "inp" => Op::Inp,
            "add" => Op::Add,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "mod" => Op::Mod,
            "eql" => Op::Eql,
            _ => panic!("Unknown op: {}", string),
        }
    }
}

#[derive(Debug, Clone)]
struct Instr {
    op: Op,
    left: char,
    right: Option<String>,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Variables {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Alu {
    vars: Variables,
}

impl Alu {
    fn apply_instr(&mut self, instr: Instr) {
        match instr.op {
            Op::Add => {
                let left = self.vars.read(instr.left);
                let right = match instr.right.as_ref().unwrap().parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => self.vars.read(instr.right.unwrap().chars().next().unwrap()),
                };

                self.vars.write(instr.left, left + right);
            },
            Op::Mul => {
                let left = self.vars.read(instr.left);
                let right = match instr.right.as_ref().unwrap().parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => self.vars.read(instr.right.unwrap().chars().next().unwrap()),
                };

                self.vars.write(instr.left, left * right);
            },
            Op::Div => {
                let left = self.vars.read(instr.left);
                let right = match instr.right.as_ref().unwrap().parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => self.vars.read(instr.right.unwrap().chars().next().unwrap()),
                };

                self.vars.write(instr.left, left / right);
            },
            Op::Mod => {
                let left = self.vars.read(instr.left);
                let right = match instr.right.as_ref().unwrap().parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => self.vars.read(instr.right.unwrap().chars().next().unwrap()),
                };

                self.vars.write(instr.left, left % right);
            },
            Op::Eql => {
                let left = self.vars.read(instr.left);
                let right = match instr.right.as_ref().unwrap().parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => self.vars.read(instr.right.unwrap().chars().next().unwrap()),
                };

                self.vars.write(instr.left, (left == right) as i32);
            },
            _ => {},
        }
    }
}

impl Variables {
    fn write(&mut self, var: char, value: i32) {
        match var {
            'w' => self.w = value,
            'x' => self.x = value,
            'y' => self.y = value,
            'z' => self.z = value,
            _ => panic!("Unexpected variable: {}", var),
        }
    }

    fn read(&self, var: char) -> i32 {
        match var {
            'w' => self.w,
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            _ => panic!("Unexpected variable: {}", var),
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let program = file
        .lines()
        .map(|line| {
            let mut elems = line.split(" ");

            Instr {
                op: Op::from(elems.next().unwrap()),
                left: elems.next().unwrap().chars().next().unwrap(),
                right: match elems.next() {
                    Some(s) => Some(s.to_owned()),
                    None => None,
                },
            }
        })
        .collect::<Vec<Instr>>();

    let mut alus: Vec<(Alu, (u64, u64))> = vec![(Alu::default(), (0, 0))];
    for instr in program {
        match instr.op {
            Op::Inp => {
                let mut new_alus: Vec<(Alu, (u64, u64))> = vec![];
                let mut indices: HashMap<Alu, usize> = HashMap::new();

                while !alus.is_empty() {
                    let alu = alus.pop().unwrap();
                    for inp in 1..=9 {
                        let mut new_alu = alu.clone();
                        new_alu.0.vars.write(instr.left, inp);

                        new_alu.1.0 = new_alu.1.0 * 10 + inp as u64;
                        new_alu.1.1 = new_alu.1.1 * 10 + inp as u64;

                        if let Some(index) = indices.get(&new_alu.0) {
                            new_alus[*index].1.0 = new_alu.1.0.max(new_alus[*index].1.0);
                            new_alus[*index].1.1 = new_alu.1.1.min(new_alus[*index].1.1);
                        } else {
                            indices.insert(new_alu.0.clone(), new_alus.len());
                            new_alus.push(new_alu);
                        }
                    }
                }

                alus = new_alus;
            },
            _ => {
                for alu in &mut alus {
                    alu.0.apply_instr(instr.clone())
                }
            }
        }
    }

    println!("Part 1: {}", alus.iter().filter(|alu| alu.0.vars.z == 0).map(|alu| alu.1.0).max().unwrap());
    println!("Part 2: {}", alus.iter().filter(|alu| alu.0.vars.z == 0).map(|alu| alu.1.1).min().unwrap());
}
