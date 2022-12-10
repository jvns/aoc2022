use std::io::Read;

fn read_stdin() -> String {
    let mut input = String::new();
    _ = std::io::stdin().read_to_string(&mut input);
    return input;
}

#[derive(Debug, Clone)]
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

fn part1(input: String) {
    let mut x = 1;
    let sum: i32 = parse(input)
        .iter()
        .flat_map(|instruction| match instruction {
            Instruction::Noop => vec![Instruction::Noop],
            Instruction::AddX(n) => vec![Instruction::Noop, Instruction::AddX(*n)],
        })
        .enumerate()
        .map(|(i, instruction)| {
            match instruction {
                Instruction::Noop => {}
                Instruction::AddX(n) => x += n,
            }
            (i + 1, x)
        })
        .filter(|(count, _)| count % 40 == 20)
        // print out
        .map(|(count, x)| {
            println!("{}: {}", count, x);
            (count, x)
        })
        .map(|(x, y)| (x as i32) * (y as i32))
        .sum();
    println!("Sum: {}", sum);
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
