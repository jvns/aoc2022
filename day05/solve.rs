use std::io::Read;
/*
 *
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
*/

type Move = (usize, usize, usize);
type State = Vec<Vec<char>>;
fn parse_input() -> (State, Vec<Move>) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // split on ("\n\n")
    let mut parts = input.split("\n\n");
    let (stacks, moves) = (parts.next().unwrap(), parts.next().unwrap());

    let lines: Vec<Vec<char>> = stacks.lines().map(|line| line.chars().collect()).collect();
    let length = lines[0].len();
    // transpose lines
    let state: Vec<Vec<char>> = (0..length)
        .map(|i| lines.iter().map(|line| line[i]).collect())
        .enumerate()
        .filter(|(i, _)| i % 4 == 1)
        .map(|(_, v)| v)
        .map(|mut v: Vec<char>| {
            v.pop();
            v.retain(|&c| c != ' ');
            v.reverse();
            v
        })
        .collect();
    let moves: Vec<Move> = moves
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            let num = parts[1].parse().unwrap();
            let from = parts[3].parse().unwrap();
            let to = parts[5].parse().unwrap();
            (num, from, to)
        })
        .collect();
    (state, moves)
}

fn move_one(from: usize, to: usize, state: &mut State) {
    let item = state[from - 1].pop();
    state[to - 1].push(item.unwrap());
}

fn move_many(num: usize, from: usize, to: usize, state: &mut State) {
    let index = state[from - 1].len() - num;
    let items: Vec<char> = state[from - 1].drain(index..).collect();
    state[to - 1].extend(items);
}

fn part1() {
    let (mut state, moves) = parse_input();

    for (num, from, to) in moves {
        for _ in 0..num {
            move_one(from, to, &mut state);
        }
    }
    for stack in state {
        print!("{}", stack.last().unwrap());
    }
    println!();
}

fn part2() {
    let (mut state, moves) = parse_input();

    for (num, from, to) in moves {
        move_many(num, from, to, &mut state);
    }
    for stack in state {
        print!("{}", stack.last().unwrap());
    }
    println!();
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
