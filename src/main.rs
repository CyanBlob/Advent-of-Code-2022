use std::fs::File;
use std::io::BufRead;

fn main() {
    // part 1
    let mut stacks: Vec<Vec<char>> = Vec::<Vec<char>>::new();

    let mut stack_input: Vec<String> = vec![];
    let mut command_input: Vec<String> = vec![];
    let mut num_stacks: i32 = 0;

    let mut parsing_commands = false;

    // gather stack information
    let file = File::open("input5.txt").expect("Could not open file");
    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if line.trim().starts_with('1') {
                num_stacks = line
                    .trim()
                    .chars()
                    .last()
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
                parsing_commands = true;
            } else if parsing_commands {
                command_input.push(line);
            } else {
                stack_input.push(line);
            }
        }
    }

    println!("{}", num_stacks);

    // create and format stacks
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }

    for row in stack_input {
        for i in 1..num_stacks + 1 {
            let char = row.chars().nth((i * 4 - 3) as usize).unwrap();

            if char != ' ' {
                stacks[(i - 1) as usize].push(char);
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
        println!("{:?}", stack);
    }
    
    let mut part2_stacks = stacks.clone();

    // manipulate stacks
    for command in command_input {
        if command != "" {
            let split_command = command.split(" ").collect::<Vec::<&str>>();
            let count  = split_command[1].parse::<i32>().unwrap();
            let source = split_command[3].parse::<i32>().unwrap();
            let dest   = split_command[5].parse::<i32>().unwrap();
            
            // part 1
            for _ in 0..count {
                let popped = stacks[(source - 1) as usize].pop().unwrap();
                stacks[(dest - 1) as usize].push(popped);
            }

            // part 2
            let mut crates = Vec::<char>::new();
            for _ in 0..count {
                crates.push(part2_stacks[(source - 1) as usize].pop().unwrap());
            }
            for _ in 0..count {
                crates.reverse();
                part2_stacks[(dest - 1) as usize].append(&mut crates);
            }
        }
    }


    println!("Manipulated stacks p1");
    for stack in &mut stacks {
        println!("{:?}", stack);
    }

    println!("Stack tops p1");
    for stack in &mut stacks {
        println!("{:?}", stack.last());
    }

    println!("Manipulated stacks p2");
    for stack in &mut part2_stacks {
        println!("{:?}", stack);
    }

    println!("Stack tops p2");
    for stack in &mut part2_stacks {
        println!("{:?}", stack.last());
    }
}
