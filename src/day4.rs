use std::fs::File;
use std::io::BufRead;

fn main() {
    // part 1
    let mut full_overlap = 0;

    let file = File::open("input4.txt").expect("Could not open file");
    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let line = line.replace(",", "-");
            let split = line.split::<&str>("-").collect::<Vec::<&str>>();
            let elf1: Vec::<i32> = (split[0].parse::<i32>().unwrap()..split[1].parse::<i32>().unwrap() + 1).collect();
            let elf2: Vec::<i32> = (split[2].parse::<i32>().unwrap()..split[3].parse::<i32>().unwrap() + 1).collect();
            
            if elf1.iter().all(|x| elf2.contains(x)) || elf2.iter().all(|x| elf1.contains(x)) {
                full_overlap += 1;
            }
        }
    }
    
    println!("Full overlaps: {}", full_overlap);

    // part 2
    let mut partial_overlap = 0;
    let file = File::open("input4.txt").expect("Could not open file");
    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let line = line.replace(",", "-");
            let split = line.split::<&str>("-").collect::<Vec::<&str>>();
            let elf1: Vec::<i32> = (split[0].parse::<i32>().unwrap()..split[1].parse::<i32>().unwrap() + 1).collect();
            let elf2: Vec::<i32> = (split[2].parse::<i32>().unwrap()..split[3].parse::<i32>().unwrap() + 1).collect();
            
            if elf1.iter().any(|x| elf2.contains(x)) || elf2.iter().any(|x| elf1.contains(x)) {
                partial_overlap += 1;
            }
        }
    }

    println!("partial overlaps: {}", partial_overlap);

}