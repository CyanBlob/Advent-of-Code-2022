use std::fs::File;
use std::io::BufRead;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct Dir<'a> {
    dirs: Vec<Dir<'a>>,
    files: Vec<&'a File_>,
    path: String,
    //parent: Option<&'a Dir<'a>>,
}

impl<'a> Dir<'a> {
    fn navigate_to(&self, path: &str, current_path: &str) -> String {
        match path {
            ".." => {
                let mut _path = current_path.split("/").collect::<Vec<&str>>();
                _path.pop();
                _path.pop();
                _path.join("/") + "/"
            }
            "/" => "/".to_string(),
            sub_dir => current_path.to_owned() + sub_dir + "/",
        }
    }

    fn get_dir(&mut self, path: &str) -> &'a mut Dir {
        let split_path = path.split("/").collect::<Vec<&str>>();

        let mut current_dir: &mut Dir = self;

        for dir in split_path {
            current_dir = current_dir
                .dirs
                .iter_mut()
                .find(|v| v.path.split("/").collect::<Vec<&str>>().first().unwrap() == &dir)
                .unwrap();
        }
        print!("Search path: {}", &path);
        println!("Got dir: {:?}", &current_dir);
        current_dir
    }

    fn add_dir(&mut self, path: &str) {
        let split_path = path.split("/").collect::<Vec<&str>>();

        /*let mut current_dir: &mut Dir = self;
        let mut last_dir = "";

        for dir in split_path {
            last_dir = dir;
        }*/

        self.dirs.push(Dir {
            dirs: vec![],
            files: vec![],
            path: path.to_string(),
            //parent: todo!(),
        });
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct File_ {
    size: usize,
}

enum Command<'a> {
    cd(&'a str),
    ls,
}

fn input_to_command(input: &str) -> Command {
    let split_input: Vec<&str> = input.split(" ").collect();

    match split_input[0] {
        "cd" => Command::cd(split_input[1]),
        "ls" => Command::ls,
        _ => {
            panic!("Invalid command")
        }
    }
}

fn parse_ls() {}

fn main() {
    let file = File::open("input7.txt").expect("Could not open file");

    let mut root = Dir {
        dirs: vec![],
        files: vec![],
        path: "/".to_string(),
        //parent: None,
    };

    //let mut current_dir = &mut root;
    let mut current_path: String;
    current_path = root.path.to_string();

    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if &line[0..1] == "$" {
                match input_to_command(&line[2..line.len()]) {
                    Command::cd(path) => {
                        //println!("CD: {}", path);
                        current_path = root.navigate_to(path, &current_path);
                        /*root.dirs.push(Dir {
                            dirs: vec![],
                            files: vec![],
                            path: current_path.clone(),
                            //parent: todo!(),
                        });
                        println!("{:?}", current_path);*/
                    }
                    Command::ls => {}
                }
            } else {
                if line.contains("dir ") {
                    let dir: &mut Dir = root.get_dir(&current_path);
                    dir.add_dir("test");
                }
            }
        }
    }
}
