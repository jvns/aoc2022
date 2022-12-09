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

struct LongTailPositions {
    tail: Vec<(i32, i32)>,
    tail_history: HashSet<(i32, i32)>,
}

fn update_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let (hx, hy) = head;
    let (tx, ty) = tail;

    if (hx - tx).abs() <= 1 && (hy - ty).abs() <= 1 {
        return tail;
    } else if hx == tx || hy == ty {
        let new_ty = (ty + hy) / 2;
        let new_tx = (tx + hx) / 2;
        return (new_tx, new_ty);
    } else {
        // move tail diagonally in direction of head
        let new_tx = if hx > tx { tx + 1 } else { tx - 1 };
        let new_ty = if hy > ty { ty + 1 } else { ty - 1 };
        return (new_tx, new_ty);
    }
}

fn move_head(head: (i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (head.0, head.1 + 1),
        Direction::Down => (head.0, head.1 - 1),
        Direction::Left => (head.0 - 1, head.1),
        Direction::Right => (head.0 + 1, head.1),
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
            positions.head = move_head(positions.head, &direction);
            positions.tail = update_tail(positions.head, positions.tail);
            positions.tail_history.insert(positions.tail);
        }
    }
    println!("{}", positions.tail_history.len());
}

fn print_grid(positions: &LongTailPositions, bottom_left: (i32, i32), top_right: (i32, i32)) {
    for y in (bottom_left.1..top_right.1 + 1).rev() {
        for x in bottom_left.0..top_right.0 + 1 {
            // h for beginning of tail
            if positions.tail[0] == (x, y) {
                print!("H");
            } else if positions.tail.contains(&(x, y)) {
                print!("T");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn part2(input: String) {
    let directions = parse(input);
    let mut positions = LongTailPositions {
        tail: vec![(0, 0); 10],
        tail_history: HashSet::new(),
    };
    for (direction, distance) in directions {
        for _ in 0..distance {
            positions.tail[0] = move_head(positions.tail[0], &direction);
            for i in 0..positions.tail.len() - 1 {
                positions.tail[i + 1] = update_tail(positions.tail[i], positions.tail[i + 1]);
                // print_grid(&positions, (0, 0), (10, 10));
                // std::thread::sleep(std::time::Duration::from_millis(100));
            }
            positions.tail_history.insert(positions.tail[9]);
        }
    }
    println!("{}", positions.tail_history.len());
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
