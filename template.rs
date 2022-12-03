use std::io::Read;
fn part1() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
}

fn part2() {}

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
