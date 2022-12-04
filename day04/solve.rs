use std::io::Read;

fn parse_range(input: &str) -> (u32, u32) {
    // 51-88 -> (51, 88)
    let mut parts = input.split('-');
    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();
    (start, end)
}

fn get_ranges() -> Vec<((u32, u32), (u32, u32))> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            let (first, second) = (parts.next().unwrap(), parts.next().unwrap());
            (parse_range(first), parse_range(second))
        })
        .collect()
}

fn part1() {
    let sum: u32 = get_ranges()
        .iter()
        .map(|((start1, end1), (start2, end2))| {
            // check if one range is inside the other
            if (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("{}", sum);
}

fn part2() {
    let sum: u32 = get_ranges()
        .iter()
        .map(|((start1, end1), (start2, end2))| {
            // check if the ranges overlap at all
            if (start1 >= start2 && start1 <= end2) || (start2 >= start1 && start2 <= end1) {
                1
            } else {
                0
            }
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
