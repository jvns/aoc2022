use std::io::Read;

fn read_stdin() -> String {
    let mut input = String::new();
    _ = std::io::stdin().read_to_string(&mut input);
    return input;
}

#[derive(Debug, Copy, Clone)]
struct Sensor {
    sensor_x: i32,
    sensor_y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

/*

...x....xS..xx.x
....x...xx..x.x..
.....B..xx.x.x.
......x...xxx..
.......xxxxx......
........xxx.......
.........x........


s = (0,0)
B = (-4, 2)

dist = 6

range = (-2, 2)

y = 4

*/

fn voronoi_range(sensor: &Sensor, y: i32) -> Option<(i32, i32)> {
    let min_beacon_distance =
        (sensor.beacon_x - sensor.sensor_x).abs() + (sensor.beacon_y - sensor.sensor_y).abs();

    // find all xs that are within beacon distance of sensor
    let y_dist = (y - sensor.sensor_y).abs();
    if y_dist > min_beacon_distance {
        return None;
    }

    let x_radius = min_beacon_distance - y_dist;
    return Some((sensor.sensor_x - x_radius, sensor.sensor_x + x_radius));
}

fn parse(input: String) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let sensor = parts.next().unwrap();
            let beacon = parts.next().unwrap();
            let mut sensor_parts = sensor.split(", ");
            let sensor_x = sensor_parts
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let sensor_y = sensor_parts
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let mut beacon_parts = beacon.split(", ");
            let beacon_x = beacon_parts
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let beacon_y = beacon_parts
                .next()
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            Sensor {
                sensor_x: sensor_x,
                sensor_y: sensor_y,
                beacon_x: beacon_x,
                beacon_y: beacon_y,
            }
        })
        .collect()
}

fn part1(input: String) {
    // 4411859: too low
    // 4411858: even lower
    let sensors = parse(input);

    let y = 2_000_000;

    let ranges = sensors
        .iter()
        .map(|sensor| voronoi_range(sensor, y))
        .filter(|range| range.is_some())
        .map(|range| range.unwrap())
        .collect::<Vec<(i32, i32)>>();

    // print out the ranges
    ranges
        .iter()
        .for_each(|range| println!("{} - {}", range.0, range.1));

    let xs = sensors
        .iter()
        .filter(|sensor| sensor.beacon_y == y)
        .map(|sensor| sensor.beacon_x)
        .collect::<Vec<i32>>();

    let min_x = sensors
        .iter()
        .map(|sensor| sensor.beacon_x.min(sensor.sensor_x))
        .min()
        .unwrap()
        - 4000000;
    let max_x = sensors
        .iter()
        .map(|sensor| sensor.beacon_x.max(sensor.sensor_x))
        .max()
        .unwrap()
        + 4000000;

    let mut count = 0;
    for x in (min_x - 1)..max_x + 2 {
        if xs.contains(&x) {
        } else if ranges.iter().any(|range| range.0 <= x && x <= range.1) {
            count += 1;
        }
    }

    println!("{}", count);

    // find all beacons that are closer to some sens
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
