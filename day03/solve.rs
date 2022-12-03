use std::collections::HashSet;
use std::io::Read;

fn char_to_num(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 64 + 26
    }
}

fn get_duplicates(first: &[u32], second: &[u32]) -> Vec<u32> {
    let first_set: HashSet<u32> = first.iter().cloned().collect();
    let second_set: HashSet<u32> = second.iter().cloned().collect();

    return first_set.intersection(&second_set).cloned().collect();
}

fn part1() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let sum = input
        .lines()
        .map(|line| {
            let nums: Vec<u32> = line.chars().map(char_to_num).collect();
            let dups = get_duplicates(&nums[0..nums.len() / 2], &nums[nums.len() / 2..]);
            dups.iter().sum::<u32>()
        })
        .sum::<u32>();
    println!("{}", sum);
}

fn part2() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let sum = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            let nums1: Vec<u32> = group[0].chars().map(char_to_num).collect();
            let nums2: Vec<u32> = group[1].chars().map(char_to_num).collect();
            let nums3: Vec<u32> = group[2].chars().map(char_to_num).collect();
            let dups = get_duplicates(&nums1, &nums2);
            let dups = get_duplicates(&dups, &nums3);
            dups.iter().sum::<u32>()
        })
        .sum::<u32>();
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
