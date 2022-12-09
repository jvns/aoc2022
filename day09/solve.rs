use std::collections::HashSet;
use std::io::Read;

fn read_stdin() -> String {
    let mut input = String::new();
    _ = std::io::stdin().read_to_string(&mut input);
    return input;
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: String) -> Vec<(Direction, u32)> {
    // R 4
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let dist = parts[1].parse::<u32>().unwrap();
            match parts[0] {
                "R" => (Direction::Right, dist),
                "L" => (Direction::Left, dist),
                "U" => (Direction::Up, dist),
                "D" => (Direction::Down, dist),
                _ => panic!("Unknown direction"),
            }
        })
        .collect()
}

struct Positions {
    head: (i32, i32),
    tail: (i32, i32),
    tail_history: HashSet<(i32, i32)>,
}

fn move_head(
    head: (i32, i32),
    tail: (i32, i32),
    direction: &Direction,
) -> ((i32, i32), (i32, i32)) {
    let new_head = match direction {
        Direction::Up => (head.0, head.1 + 1),
        Direction::Down => (head.0, head.1 - 1),
        Direction::Left => (head.0 - 1, head.1),
        Direction::Right => (head.0 + 1, head.1),
    };

    let (hx, hy) = new_head;
    let (tx, ty) = tail;

    if (hx - tx).abs() <= 1 && (hy - ty).abs() <= 1 {
        return (new_head, tail);
    } else if hx == tx || hy == ty {
        let new_ty = (ty + hy) / 2;
        let new_tx = (tx + hx) / 2;
        return (new_head, (new_tx, new_ty));
    } else {
        // move tail diagonally in direction of head
        let new_tx = if hx > tx { tx + 1 } else { tx - 1 };
        let new_ty = if hy > ty { ty + 1 } else { ty - 1 };
        return (new_head, (new_tx, new_ty));
    }
}

fn part1(input: String) {
    let directions = parse(input);
    let mut positions = Positions {
        head: (0, 0),
        tail: (0, 0),
        tail_history: HashSet::new(),
    };
    for (direction, distance) in directions {
        for _ in 0..distance {
            let (new_head, new_tail) = move_head(positions.head, positions.tail, &direction);
            positions.head = new_head;
            positions.tail = new_tail;
            positions.tail_history.insert(new_tail);
        }
    }
    println!("{}", positions.tail_history.len());
}

fn part2(_input: String) {}

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
