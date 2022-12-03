use std::io::Read;

fn char_to_num(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 64 + 26
    }
}

fn get_duplicates2(first: &[u8], second: &[u8]) -> u32 {
    for i in 0..first.len() {
        if second.contains(&first[i]) {
            return char_to_num(first[i] as char);
        }
    }
    panic!("No duplicates found");
}

fn get_duplicates3(first: &[u8], second: &[u8], third: &[u8]) -> u32 {
    for i in 0..first.len() {
        if second.contains(&first[i]) && third.contains(&first[i]) {
            return char_to_num(first[i] as char);
        }
    }
    panic!("No duplicates found");
}

fn get_duplicates3_set(first: &[u8], second: &[u8], third: &[u8]) -> u32 {
    let second_set: std::collections::HashSet<_> = second.iter().collect();
    let third_set: std::collections::HashSet<_> = third.iter().collect();
    for i in 0..first.len() {
        if second_set.contains(&first[i]) && third_set.contains(&first[i]) {
            return char_to_num(first[i] as char);
        }
    }
    panic!("No duplicates found");
}

fn part1() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let sum = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let first = &bytes[..bytes.len() / 2];
            let second = &bytes[bytes.len() / 2..];
            get_duplicates2(first, second)
        })
        .sum::<u32>();
    println!("{}", sum);
}

fn part2() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    /* time _set version */
    let time = std::time::Instant::now();
    for i in 0..1000 {
        let mut lines = input.lines();
        let mut sum = 0;

        loop {
            let (first, second, third) = match (lines.next(), lines.next(), lines.next()) {
                (Some(first), Some(second), Some(third)) => {
                    (first.as_bytes(), second.as_bytes(), third.as_bytes())
                }
                _ => break,
            };
            sum += get_duplicates3_set(first, second, third);
        }
    }
    println!("Time: {}ms", time.elapsed().as_millis());

    /* time O(n^2) version */
    let time2 = std::time::Instant::now();
    for i in 0..1000 {
        let mut lines = input.lines();
        let mut sum = 0;

        loop {
            let (first, second, third) = match (lines.next(), lines.next(), lines.next()) {
                (Some(first), Some(second), Some(third)) => {
                    (first.as_bytes(), second.as_bytes(), third.as_bytes())
                }
                _ => break,
            };
            sum += get_duplicates3(first, second, third);
        }
    }
    println!("Time: {}ms", time2.elapsed().as_millis());
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
