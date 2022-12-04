use std::fs::File;
use std::io::BufRead;

fn get_char_value(c: &char) -> u32 {
    match c.is_ascii_lowercase() {
        true => *c as u32 - 96,
        false => *c as u32 - 64 + 26,
    }
}

fn main() {
    let file = File::open("input3.txt").expect("Could not open file");

    let mut priority_sum = 0;

    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            line[0..line.len() / 2].chars().any(|c| {
                if line[line.len() / 2..].contains(c) {
                    priority_sum += get_char_value(&c);
                    true
                } else {
                    false
                }
            });
        }
    }

    let file = File::open("input3.txt").expect("Could not open file");

    let mut lines: Vec<String> = vec![];
    let mut shared_chars: Vec<char> = vec![];
    let mut shared_priority_sum = 0;

    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            lines.push(line.clone());

            if lines.len() == 3 {
                
                // find all matches between first two lines
                lines[0].chars().any(|c| {
                    if lines[1].contains(c) {
                        shared_chars.push(c);
                        false // false allows us to check for all matches, not just one
                    } else {
                        false
                    }
                });

                shared_chars.dedup();

                let pass_one_shared_chars = shared_chars.clone();

                shared_chars.clear();

                // find all matches between the above matches and line 3
                lines[2].chars().any(|c| {
                    if pass_one_shared_chars.contains(&c) {
                        shared_chars.push(c);

                        shared_priority_sum += get_char_value(&c);
                        true
                    } else {
                        false
                    }
                });

                println!("Shared chars: {:?}", shared_chars);

                shared_chars.clear();
                lines.clear();
            }
        }
    }

    println!("Priority sum: {}", priority_sum);
    println!("Shared priority sum: {}", shared_priority_sum);
}
