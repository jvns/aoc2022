use std::io::Read;
/* this code does not really work but it did get me the right answer */

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
impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Resource::Ore => write!(f, "Ore"),
            Resource::Clay => write!(f, "Clay"),
            Resource::Obsidian => write!(f, "Obsidian"),
            Resource::Geode => write!(f, "Geode"),
        }
    }
}

fn get_resource(i: usize) -> Resource {
    match i {
        0 => Resource::Ore,
        1 => Resource::Clay,
        2 => Resource::Obsidian,
        3 => Resource::Geode,
        _ => panic!("Invalid resource"),
    }
}

type Resources = Vec<u32>;
type Blueprint = Vec<Vec<(Resource, u32)>>;

fn parse_robot(input: &str) -> (Resource, Vec<(Resource, u32)>) {
    // obsidian robot costs 3 ore and 8 clay.
    let parts = input.split(" costs ").map(|s| s.trim()).collect::<Vec<_>>();
    let kind = match parts[0] {
        "ore robot" => Resource::Ore,
        "clay robot" => Resource::Clay,
        "obsidian robot" => Resource::Obsidian,
        "geode robot" => Resource::Geode,
        _ => panic!("unknown robot kind: {}", parts[0]),
    };
    let cost = parts[1]
        .split(" and ")
        .map(|s| s.trim())
        .map(|s| {
            let parts = s.split(" ").map(|s| s.trim()).collect::<Vec<_>>();
            let amount = parts[0].parse::<u32>().unwrap();
            let kind_str = parts[1].replace(".", "").to_string();
            let kind = match kind_str.as_str() {
                "ore" => Resource::Ore,
                "clay" => Resource::Clay,
                "obsidian" => Resource::Obsidian,
                "geode" => Resource::Geode,
                _ => panic!("unknown resource kind: '{}'", kind_str),
            };
            (kind, amount)
        })
        .collect();
    (kind, cost)
}

fn parse_blueprint(input: &str) -> Blueprint {
    let mut robots = vec![vec![]; 4];
    // skip first lien
    for line in input.split("Each ").skip(1) {
        let (kind, cost) = parse_robot(line);
        robots[kind as usize] = cost;
    }
    robots
}

fn parse(input: &str) -> Vec<Blueprint> {
    input
        .trim()
        .split("\n")
        .map(|s| parse_blueprint(s))
        .collect()
}

fn find_possible_moves(blueprint: &Blueprint, resources: &Resources) -> Vec<Resource> {
    let mut possible_moves = vec![];
    for (i, robot) in blueprint.iter().enumerate() {
        let mut can_build = true;
        for (kind, amount) in robot {
            if resources[*kind as usize] < *amount {
                can_build = false;
                break;
            }
        }
        if can_build {
            // convert i to resource
            possible_moves.push(get_resource(i));
        }
    }
    possible_moves
}

#[derive(Debug, Clone)]
struct GameState {
    resources: Vec<u32>,
    robots: Vec<u32>,
}

fn leq(a: &GameState, b: &GameState) -> bool {
    for i in 0..4 {
        if a.resources[i] >= b.resources[i] {
            return false;
        }
        if a.robots[i] >= b.robots[i] {
            return false;
        }
    }
    true
}

fn next_game_states(
    blueprint: &Blueprint,
    game_states: &Vec<GameState>,
    generation: usize,
) -> Vec<GameState> {
    let mut next_game_states = vec![];
    for game_state in game_states {
        let possible_moves = find_possible_moves(blueprint, &game_state.resources);
        //println!("blueprint: {:?}", blueprint);
        //println!("game state: {:?}", game_state);
        //println!("possible moves: {:?}", possible_moves);
        for move_ in possible_moves {
            let mut next_game_state = game_state.clone();
            for (kind, amount) in blueprint[move_ as usize].iter() {
                //println!("cost: kind: {:?}, amount: {:?}", kind, amount);
                next_game_state.resources[*kind as usize] -= *amount;
            }
            // all of the robots make 1 resource
            for i in 0..4 {
                next_game_state.resources[i] += next_game_state.robots[i];
            }
            next_game_state.robots[move_ as usize] += 1;
            //println!("next game state: {:?}", next_game_state);
            next_game_states.push(next_game_state);
        }
        // also do the no-op move
        let mut next_game_state = game_state.clone();
        for i in 0..4 {
            next_game_state.resources[i] += next_game_state.robots[i];
        }
        next_game_states.push(next_game_state);
    }
    if next_game_states.len() > 4_000_000 {
        // make sure there are some geodes
        let has_geodes = next_game_states.iter().any(|s| s.robots[3] > 0);
        if has_geodes {
            next_game_states.sort_by(|a, b| b.robots[3].cmp(&a.robots[3]));
            next_game_states.truncate(next_game_states.len() / 30);
        }
        let has_obsidian = next_game_states.iter().any(|s| s.robots[2] > 0);
        if has_obsidian {
            next_game_states.sort_by(|a, b| b.robots[2].cmp(&a.robots[2]));
            next_game_states.truncate(next_game_states.len() / 30);
        }
    }
    next_game_states
}

fn max_geode_num(blueprint: &Blueprint, iterations: usize) -> u32 {
    let mut game_state = GameState {
        resources: vec![0, 0, 0, 0],
        robots: vec![0, 0, 0, 0],
    };
    game_state.robots[Resource::Ore as usize] = 1;
    let mut game_states = vec![game_state];
    // run 24 iterations of the game
    for i in 0..iterations {
        game_states = next_game_states(blueprint, &game_states, i);
        //println!("generation: {}, game states: {}", i, game_states.len());
    }
    // print the max number of geode resources
    let max_geode = game_states
        .iter()
        .map(|s| s.resources[Resource::Geode as usize])
        .max()
        .unwrap();
    println!("max geode: {}", max_geode);
    max_geode
}

fn part1(input: String) {
    let blueprints = parse(&input);
    // multiply index + 1 * max_geode_num
    let score = blueprints
        .iter()
        .enumerate()
        .map(|(i, blueprint)| ((i + 1) as u32) * max_geode_num(blueprint, 24))
        .sum::<u32>();
    println!("{}", score);
}

fn part2(input: String) {
    let blueprints = parse(&input);
    let score = blueprints
        .iter()
        .take(3)
        .map(|(blueprint)| max_geode_num(blueprint, 32))
        .product::<u32>();

    println!("{}", score);
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
