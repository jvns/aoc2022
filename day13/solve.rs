use std::cmp::Ordering;
use std::fmt::Display;
use std::io::Read;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Item {
    int: Option<i32>,
    list: Option<Vec<Item>>,
}

fn read_stdin() -> String {
    let mut input = String::new();
    _ = std::io::stdin().read_to_string(&mut input);
    return input;
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.int {
            Some(int) => write!(f, "{}", int),
            None => {
                write!(f, "[")?;
                for (i, item) in self.list.as_ref().unwrap().iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
        }
    }
}

fn parse(input: String) -> Item {
    if input.starts_with("[") {
        let input = input[1..input.len() - 1].to_string();
        let mut items: Vec<Item> = Vec::new();
        let mut current_item = String::new();
        let mut depth = 0;
        for c in input.chars() {
            if c == '[' {
                depth += 1;
            } else if c == ']' {
                depth -= 1;
            }
            if depth == 0 && c == ',' {
                items.push(parse(current_item));
                current_item = String::new();
            } else {
                current_item.push(c);
            }
        }
        if current_item != "" {
            items.push(parse(current_item));
        }

        Item {
            int: None,
            list: Some(items),
        }
    } else {
        Item {
            int: Some(input.parse().unwrap()),
            list: None,
        }
    }
}

fn cmp(item1: &Item, item2: &Item) -> Ordering {
    match (item1, item2) {
        (
            Item {
                int: Some(i1),
                list: None,
            },
            Item {
                int: Some(i2),
                list: None,
            },
        ) => i1.cmp(i2),
        (
            Item {
                int: None,
                list: Some(l1),
            },
            Item {
                int: None,
                list: Some(l2),
            },
        ) => {
            for (i1, i2) in l1.iter().zip(l2.iter()) {
                match cmp(i1, i2) {
                    Ordering::Equal => (),
                    other => return other,
                }
            }
            l1.len().cmp(&l2.len())
        }
        (
            Item {
                int: Some(_),
                list: None,
            },
            Item {
                int: None,
                list: Some(_),
            },
        ) => cmp(
            &Item {
                int: None,
                list: Some(vec![item1.clone()]),
            },
            item2,
        ),
        (
            Item {
                int: None,
                list: Some(_),
            },
            Item {
                int: Some(_),
                list: None,
            },
        ) => cmp(
            item1,
            &Item {
                int: None,
                list: Some(vec![item2.clone()]),
            },
        ),
        _ => panic!("invalid input"),
    }
}

fn is_le(item1: &Item, item2: &Item) -> bool {
    match cmp(item1, item2) {
        Ordering::Less | Ordering::Equal => true,
        _ => false,
    }
}

fn part1(input: String) {
    // split into pairs of lists
    let num_greater: usize = input
        .split("\n\n")
        .enumerate()
        .map(|(i, pair)| {
            let mut parts = pair.split("\n");
            let list1 = parse(parts.next().unwrap().to_string());
            let list2 = parse(parts.next().unwrap().to_string());
            let result = is_le(&list1, &list2);
            (i + 1, result)
        })
        .filter(|(_, is_greater)| *is_greater)
        .map(|(i, _)| i)
        .sum();
    println!("{}", num_greater);
}

fn part2(input: String) {
    let mut all: Vec<Item> = input
        .split("\n")
        .filter(|s| *s != "")
        .map(|s| parse(s.to_string()))
        .collect();

    all.push(parse("[[2]]".to_string()));
    all.push(parse("[[6]]".to_string()));

    // sort
    all.sort_by(|a, b| cmp(a, b));

    // find position of [[2]]
    let two_pos = all
        .iter()
        .position(|item| item == &parse("[[2]]".to_string()))
        .unwrap()
        + 1;

    let six_pos = all
        .iter()
        .position(|item| item == &parse("[[6]]".to_string()))
        .unwrap()
        + 1;

    println!("{}", two_pos * six_pos);
    // todo
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
