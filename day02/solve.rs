use std::io::Read;

fn decode(c: char) -> i32 {
    if c == 'A' {
        1
    } else if c == 'B' {
        2
    } else if c == 'C' {
        3
    } else if c == 'X' {
        1
    } else if c == 'Y' {
        2
    } else if c == 'Z' {
        3
    } else {
        panic!("Invalid character");
    }
}

fn score(them: &i32, me: &i32) -> i32 {
    if (me - them == 1) || (me - them == -2) {
        6
    } else if them == me {
        3
    } else {
        0
    }
}

fn choose_shape(them: &i32, result: &i32) -> i32 {
    for i in 1..4 {
        if score(them, &i) == (result - 1) * 3 {
            return i;
        }
    }
    panic!("Invalid result");
}

fn inputs() -> Vec<(i32, i32)> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input
        .lines()
        .map(|line| {
            let c1 = line.chars().nth(0).unwrap();
            let c2 = line.chars().nth(2).unwrap();

            let them: i32 = decode(c1);
            let me: i32 = decode(c2);
            (them, me)
        })
        .collect()
}

fn part1() {
    let sum: i32 = inputs()
        .iter()
        .map(|(them, me)| {
            let score: i32 = me + score(them, me);
            score
        })
        .sum();

    println!("{}", sum);
}

fn part2() {
    let sum: i32 = inputs()
        .iter()
        .map(|(them, result)| {
            let me = choose_shape(them, result);
            me + score(them, &me)
        })
        .sum();

    println!("{}", sum);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        part1();
    } else if args[1] == "2" {
        part2();
    } else {
        panic!("invalid argument");
    }
}
