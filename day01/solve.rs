use std::io::Read;

fn sums() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    // split by \n\n
    let parts = input.split("\n\n");

    let sums: Vec<i32> = parts.map(|part| {
        let mut sum = 0;
        for line in part.lines() {
            let num = line.parse::<i32>().unwrap();
            sum += num;
        }
        sum
    }).collect::<Vec<i32>>();
    sums
}

fn part1() {
    // get max sum
    let sums = sums();
    let max = sums.iter().max().unwrap();
    println!("{}", max);
}

fn part2() {
    let mut sums = sums();
    // sort sums in reverse
    sums.sort_by(|a, b| b.cmp(a));
    // get top 3 sums
    let top3 = sums.iter().take(3);
    // add them up
    let sum = top3.fold(0, |acc, &x| acc + x);
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
