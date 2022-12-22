use std::collections::HashMap;
use std::io::Read;

fn read_stdin() -> String {
    let mut input = String::new();
    _ = std::io::stdin().read_to_string(&mut input);
    return input;
}

#[derive(Debug, Clone)]
enum Operation {
    Number(i128),
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

fn find_number(map: &mut HashMap<String, Operation>, name: &str) -> i128 {
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

#[derive(Debug, Clone)]
enum Equation {
    Number(i128),
    Polynomial(i128, i128, i128), // represents (a * x + b) / c
}

fn add(x: &Equation, y: &Equation) -> Equation {
    match (x, y) {
        (Equation::Number(a), Equation::Number(b)) => Equation::Number(a + b),
        (Equation::Number(a), Equation::Polynomial(b, c, d)) => {
            Equation::Polynomial(*b, *d * *a + *c, *d)
        }
        (Equation::Polynomial(a, b, c), Equation::Number(d)) => add(y, x),
        _ => panic!("not implemented"),
    }
}

fn sub(x: &Equation, y: &Equation) -> Equation {
    match (x, y) {
        (Equation::Number(a), Equation::Number(b)) => Equation::Number(a - b),
        (Equation::Number(a), Equation::Polynomial(b, c, d)) => {
            add(x, &Equation::Polynomial(-*b, -*c, *d))
        }
        (Equation::Polynomial(a, b, c), Equation::Number(d)) => add(x, &Equation::Number(-*d)),
        _ => panic!("not implemented"),
    }
}
fn mul(x: &Equation, y: &Equation) -> Equation {
    match (x, y) {
        (Equation::Number(a), Equation::Number(b)) => Equation::Number(a * b),
        (Equation::Number(a), Equation::Polynomial(b, c, d)) => {
            Equation::Polynomial(*a * *b, *a * *c, *d)
        }
        (Equation::Polynomial(a, b, c), Equation::Number(d)) => mul(y, x),
        _ => panic!("not implemented"),
    }
}

fn div(x: &Equation, y: &Equation) -> Equation {
    match (x, y) {
        (Equation::Number(a), Equation::Number(b)) => Equation::Number(a / b),
        (Equation::Polynomial(a, b, c), Equation::Number(d)) => {
            Equation::Polynomial(*a, *b, *c * *d)
        }
        _ => panic!("not implemented"),
    }
}

// format equation
impl std::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Equation::Number(n) => write!(f, "{}", n),
            Equation::Polynomial(a, b, c) => write!(f, "({} * x + {}) / {}", a, b, c),
        }
    }
}

fn reduce(map: &mut HashMap<String, Operation>, name: &str) -> Equation {
    if name == "humn" {
        return Equation::Polynomial(1, 0, 1);
    }
    let op = map.get(name).unwrap().clone();
    match op {
        Operation::Number(x) => Equation::Number(x),
        Operation::Add(a, b) => add(&reduce(map, &a), &reduce(map, &b)),
        Operation::Subtract(a, b) => sub(&reduce(map, &a), &reduce(map, &b)),
        Operation::Multiply(a, b) => mul(&reduce(map, &a), &reduce(map, &b)),
        Operation::Divide(a, b) => div(&reduce(map, &a), &reduce(map, &b)),
    }
}

fn part2(input: String) {
    let mut map = parse(input);
    let op = map.get("root").unwrap().clone();
    match op {
        Operation::Add(a, b) => {
            let eq1 = reduce(&mut map, &a);
            let eq2 = reduce(&mut map, &b);
            println!("{} = {}", eq1, eq2);
        }
        _ => todo!(),
    }
}

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
