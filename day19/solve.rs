use std::io::Read;

fn read_stdin() -> String {
    let mut input = String::new();
    _ = std::io::stdin().read_to_string(&mut input);
    return input;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Resource {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

type Resources = Vec<u32>;
type Blueprint = Vec<Vec<(Resource, u32)>>;

fn parse_robot(input: &str) -> (Resource, Vec<(Resource, u32)>) {
    // Each obsidian robot costs 3 ore and 8 clay.
    let parts = input.split(" costs ").map(|s| s.trim()).collect::<Vec<_>>();
    let kind = match parts[0] {
        "Each ore robot" => Resource::Ore,
        "Each clay robot" => Resource::Clay,
        "Each obsidian robot" => Resource::Obsidian,
        "Each geode robot" => Resource::Geode,
        _ => panic!("unknown robot kind"),
    };
    let cost = parts[1]
        .split(" and ")
        .map(|s| s.trim())
        .map(|s| {
            let parts = s.split(" ").map(|s| s.trim()).collect::<Vec<_>>();
            let amount = parts[0].parse::<u32>().unwrap();
            let kind_str = parts[1].replace(".", "").to_string();
            println!("{} {}", amount, parts[1]);
            let kind = match kind_str.as_str() {
                "ore" => Resource::Ore,
                "clay" => Resource::Clay,
                "obsidian" => Resource::Obsidian,
                "geode" => Resource::Geode,
                _ => panic!("unknown resource kind"),
            };
            (kind, amount)
        })
        .collect();
    (kind, cost)
}

fn parse_blueprint(input: &str) -> Blueprint {
    let mut robots = vec![vec![]; 4];
    // skip first lien
    for line in input.lines().skip(1) {
        let (kind, cost) = parse_robot(line);
        robots[kind as usize] = cost;
    }
    robots
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.split("\n\n").map(|s| parse_blueprint(s)).collect()
}

fn part1(input: String) {
    let blueprints = parse(&input);
    let mut resources = vec![0; 4];
    resources[Resource::Ore as usize] = 1;
    println!("{:?}", blueprints);
}

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
