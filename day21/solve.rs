use std::collections::HashMap;
use std::io::Read;

fn read_stdin() -> String {
    let mut input = String::new();
    _ = std::io::stdin().read_to_string(&mut input);
    return input;
}

#[derive(Debug, Clone)]
enum Operation {
    Number(i64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

fn parse(input: String) -> HashMap<String, Operation> {
    let mut map: HashMap<String, Operation> = HashMap::new();
    for line in input.lines() {
        let mut parts1 = line.split(": ");
        let key = parts1.next().unwrap().to_string();
        let value = parts1.next().unwrap();
        let parts = value.split(" ").collect::<Vec<&str>>();
        if parts.len() == 1 {
            map.insert(key, Operation::Number(value.parse().unwrap()));
        } else {
            let a = parts[0].parse().unwrap();
            let op = parts[1];
            let b = parts[2].parse().unwrap();
            match op {
                "+" => map.insert(key, Operation::Add(a, b)),
                "-" => map.insert(key, Operation::Subtract(a, b)),
                "*" => map.insert(key, Operation::Multiply(a, b)),
                "/" => map.insert(key, Operation::Divide(a, b)),
                _ => todo!(),
            };
        }
    }
    map
}

fn find_number(map: &mut HashMap<String, Operation>, name: &str) -> i64 {
    let op = map.get(name).unwrap().clone();
    match op {
        Operation::Number(n) => n,
        Operation::Add(a, b) => find_number(map, &a) + find_number(map, &b),
        Operation::Subtract(a, b) => find_number(map, &a) - find_number(map, &b),
        Operation::Multiply(a, b) => find_number(map, &a) * find_number(map, &b),
        Operation::Divide(a, b) => find_number(map, &a) / find_number(map, &b),
    }
}

fn part1(input: String) {
    let mut map = parse(input);
    let number = find_number(&mut map, "root");
    println!("{}", number);
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
