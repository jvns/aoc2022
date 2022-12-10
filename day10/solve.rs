use std::io::Read;

fn read_stdin() -> String {
    let mut input = String::new();
    _ = std::io::stdin().read_to_string(&mut input);
    return input;
}

enum Instruction {
    Noop,
    AddX(i32),
}

fn parse(input: String) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let instruction = parts.next().unwrap();
            match instruction {
                "noop" => Instruction::Noop,
                "addx" => Instruction::AddX(parts.next().unwrap().parse().unwrap()),
                _ => panic!("Unknown instruction: {}", instruction),
            }
        })
        .collect()
}

fn get_states(instructions: &Vec<Instruction>) -> impl Iterator<Item = (usize, i32)> + '_ {
    let mut iter = instructions.iter();
    let mut instruction_cycle = 0;
    let mut instruction_count = 0;
    let mut current_instruction: Option<&Instruction> = None;
    let mut x = 1;
    std::iter::from_fn(move || {
        instruction_count += 1;
        match (current_instruction, instruction_cycle) {
            (Some(Instruction::AddX(n)), 0) => {
                instruction_cycle = 1;
            }
            _ => {
                instruction_cycle = 0;
                current_instruction = iter.next();
            }
        }
        match (current_instruction, instruction_cycle) {
            (Some(Instruction::AddX(_)), 0) => Some((instruction_count, x)),
            (Some(Instruction::AddX(delta)), 1) => {
                x += delta;
                Some((instruction_count, x))
            }
            (Some(Instruction::Noop), _) => Some((instruction_count, x)),
            _ => None,
        }
    })
}
fn part1(input: String) {
    let instructions = parse(input);
    let states = get_states(&instructions);
    for (instruction_count, x) in states {
        println!("{}: {}", instruction_count, x);
    }
}

fn part2(input: String) {}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = read_stdin();
    if args[1] == "1" {
        part1(input);
    } else if args[1] == "2" {
        part2(input);
    } else {
        panic!("invalid argument");
    }
}
