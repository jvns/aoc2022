use std::io::Read;

fn all_different(s: &[u8]) -> bool {
    let mut seen = [false; 256];
    for &c in s {
        if seen[c as usize] {
            return false;
        }
        seen[c as usize] = true;
    }
    return true;
}

fn solve(num_chars: usize) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let bytes = input.as_bytes();

    for i in num_chars..bytes.len() {
        if all_different(&bytes[i - num_chars..i]) {
            println!("{}", i);
            return;
        }
    }
}

fn part1() {
    solve(4)
}

fn part2() {
    solve(14)
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
