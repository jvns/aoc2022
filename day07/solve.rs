use std::io::Read;

#[derive(Debug)]
struct File {
    _name: String,
    size: u64,
}

#[derive(Debug)]
struct Folder {
    _name: String,
    files: Vec<File>,
    folders: Vec<Folder>,
}

impl Folder {
    fn size(&self) -> u64 {
        let file_size: u64 = self.files.iter().map(|f| f.size).sum();
        let folder_size: u64 = self.folders.iter().map(|f| f.size()).sum();
        file_size + folder_size
    }
    fn new(name: &str) -> Folder {
        Folder {
            _name: name.to_string(),
            files: Vec::new(),
            folders: Vec::new(),
        }
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    _ = std::io::stdin().read_to_string(&mut input);
    return input;
}

fn read_folder(root: &mut Folder, lines: &mut std::str::Lines) {
    let mut listing = false;
    while let Some(line) = lines.next() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        // match parts using a `match`
        match (parts[0], parts[1]) {
            ("$", "ls") => {
                listing = true;
            }
            ("dir", _name) => {}
            ("$", "cd") => {
                let name = parts[2];
                if name == ".." {
                    return;
                }
                let mut folder = Folder::new(name);
                read_folder(&mut folder, lines);
                root.folders.push(folder);
            }
            (size, name) => {
                assert!(listing);
                let size_num = size.parse::<u64>().unwrap();
                root.files.push(File {
                    _name: name.to_string(),
                    size: size_num,
                });
            }
        }
    }
}

fn folder_sizes(input: String) -> Vec<u64> {
    let mut folder = Folder::new("/");
    let mut lines = input.lines();
    read_folder(&mut folder, &mut lines);
    let mut sizes: Vec<u64> = vec![];
    let mut queue = vec![folder];
    while let Some(folder) = queue.pop() {
        // print name and size
        sizes.push(folder.size());
        queue.extend(folder.folders);
    }

    sizes
}

fn part1(input: String) {
    let sizes = folder_sizes(input);
    let sum = sizes.iter().filter(|s| **s < 100000).sum::<u64>();
    println!("{}", sum);
}

fn part2(input: String) {
    let sizes = folder_sizes(input);
    let root_size = sizes[0];
    let disk_size = 70000000;
    let min_space = 30000000 - (disk_size - root_size);
    // find the smallest folder that would free at least min_space
    let min = sizes.iter().filter(|s| **s >= min_space).min().unwrap();
    println!("{}", min);
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
