use std::fs::File;
use std::io::BufRead;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Dir {
    dirs: Vec<Dir>,
    files: Vec<File_>,
    path: String,
    parent_path: String,
}

impl Dir {
    fn navigate_to(&mut self, path: &str) -> &mut Dir {
        if !self.contains_dir(path) {
            self.add_dir(path);
        }

        self.get_dir(path).unwrap()
    }

    fn navigate_to_from_root(&mut self, path: &str) -> &mut Dir {
        self.get_dir_from_root(path)
    }

    fn contains_dir(&self, path: &str) -> bool {
        self.dirs.iter().any(|d| d.path == path)
    }

    fn get_dir(&mut self, path: &str) -> Option<&mut Dir> {
        let current_dir: &mut Dir = self;

        current_dir
            .dirs
            .iter_mut()
            .find(|v| v.path == path.to_string())
    }

    fn get_dir_from_root(&mut self, path: &str) -> &mut Dir {
        let split_path = path.split("/").collect::<Vec<&str>>();

        let mut current_dir: &mut Dir = self;

        for dir in split_path {
            if dir == "" {
                continue;
            }
            current_dir = current_dir
                .dirs
                .iter_mut()
                .find(|v| v.path.split("/").collect::<Vec<&str>>().first().unwrap() == &dir)
                .unwrap();
        }
        current_dir
    }

    fn add_dir(&mut self, path: &str) {
        self.dirs.push(Dir {
            dirs: vec![],
            files: vec![],
            path: path.to_string(),
            parent_path: self.parent_path.to_string() + &self.path.to_string() + "/",
            //parent: None,
        })
    }

    fn add_file(&mut self, size: usize) {
        self.files.push(File_ { size });
    }

    fn get_size(&self, total: &mut usize) -> usize {
        let file_size = self
            .files
            .iter()
            .fold(0, |sum, val| sum + val.size as usize);
        let dir_size = self.dirs.iter().fold(0, |sum, val| sum + val.get_size(total));

        if file_size + dir_size <= 100000 {
            *total += file_size + dir_size;
        }
        
        
        file_size + dir_size
    }

    fn get_smallest_for_target(&self, total: &mut usize, smallest_for_target: &mut usize, target: &usize) -> usize {
        
        let file_size = self
            .files
            .iter()
            .fold(0, |sum, val| sum + val.size as usize);
        let dir_size = self.dirs.iter().fold(0, |sum, val| sum + val.get_smallest_for_target(total, smallest_for_target, target));

        let total_size = file_size + dir_size;

        *total += total_size;

        if total_size > *target && total_size < *smallest_for_target {
            println!("Old selection: {}", smallest_for_target);
            *smallest_for_target = total_size;
            println!("New selection: {}", smallest_for_target);
        }

        total_size
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct File_ {
    size: usize,
}

enum Command<'a> {
    Cd(&'a str),
    Ls,
}

fn input_to_command(input: &str) -> Command {
    let split_input: Vec<&str> = input.split(" ").collect();

    match split_input[0] {
        "cd" => Command::Cd(split_input[1]),
        "ls" => Command::Ls,
        _ => {
            panic!("Invalid command")
        }
    }
}

fn main() {
    let file = File::open("input7.txt").expect("Could not open file");

    let mut root = Dir {
        dirs: vec![],
        files: vec![],
        path: "/".to_string(),
        parent_path: "/".to_string(),
    };

    let mut current_dir = &mut root;

    let mut current_path: String;

    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if &line[0..1] == "$" {
                match input_to_command(&line[2..line.len()]) {
                    Command::Cd(path) => {
                        //println!("CD: {}", path);
                        current_path = current_dir.parent_path.to_string();
                        match path {
                            "/" => current_dir = &mut root,
                            ".." => current_dir = root.navigate_to_from_root(&current_path),
                            _sub_path => current_dir = current_dir.navigate_to(path),
                        }
                    }
                    Command::Ls => {} // woops, forgot to use *shrug*
                }
            } else {
                if line.contains("dir ") {
                    current_dir.add_dir(line.split(" ").collect::<Vec<&str>>().last().unwrap());
                } else {
                    current_dir.add_file(
                        line.split(" ")
                            .collect::<Vec<&str>>()
                            .first()
                            .unwrap()
                            .parse()
                            .unwrap(),
                    );
                }
            }
        }
    }

    let mut sum = 0;
    current_dir = &mut root;
    let total = current_dir.get_size(&mut sum);
    let free = 70000000 - total;
    let target = 30000000 - free;
    
    let mut smallest = 70000000;
    
    current_dir.get_smallest_for_target(&mut sum, &mut smallest, &target);
    
    println!("Free: {}", free);
    println!("Target: {}", target);
    println!("Smallest for target: {}", smallest);
}
