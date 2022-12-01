use std::fs::File;
use std::io::BufRead;

#[derive(PartialEq, Eq, Ord)]
struct Elf {
    meals: Vec<i32>,
}

impl Elf {
    fn new() -> Self {
        return Elf {
            meals: Vec::<i32>::new(),
        };
    }

    fn with_meals(meals: &Vec::<i32>) -> Self {
        return Elf {
            meals: meals.clone(),
        };
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.meals.iter().sum::<i32>().cmp(&other.meals.iter().sum::<i32>()))
    }
}

fn main() {
    let file = File::open("input1.txt").expect("Could not open file");

    let mut elves: Vec::<Elf> = vec![Elf::new()];
    
    let mut meals: Vec::<i32> = vec![];

    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if line.trim() == "" {
                elves.push(Elf::with_meals(&meals));
                meals.clear();
            }
            else {
                meals.push(line.parse::<i32>().unwrap());
            }
        }
    }
    if elves.len() < 3 {
        println!("Invalid data; must have 3 or more elves. Have {}", elves.len());
        return;
    }

    elves.sort();
    elves.reverse();
    
    let mut sum = 0;
    
    println!("Top 3: ");
    for i in 0..3 {
        let elf_sum = elves[i].meals.iter().sum::<i32>();
        sum += elf_sum;
        println!("{}", elf_sum);
    }
    
    println!("Total of top 3 elves: {}", sum);
}
