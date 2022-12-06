use std::io::Read;

fn read_stdin() -> String {
    let mut input = String::new();
    _ = std::io::stdin().read_to_string(&mut input);
    return input;
}

fn part1(input: String) {}

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
