use std::fs::File;
use std::io::BufRead;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Selection {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcomes {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl std::ops::Add<Selection> for Outcomes {
    type Output = i32;

    fn add(self, rhs: Selection) -> Self::Output {
        self as i32 + rhs as i32
    }
}

fn get_score(opponent: Selection, player: Selection) -> i32 {
    match opponent {
        Selection::Rock => match player {
            Selection::Rock => Outcomes::Draw + player,
            Selection::Paper => Outcomes::Win + player,
            Selection::Scissors => Outcomes::Loss + player,
        },
        Selection::Paper => match player {
            Selection::Rock => Outcomes::Loss + player,
            Selection::Paper => Outcomes::Draw + player,
            Selection::Scissors => Outcomes::Win + player,
        },
        Selection::Scissors => match player {
            Selection::Rock => Outcomes::Win + player,
            Selection::Paper => Outcomes::Loss + player,
            Selection::Scissors => Outcomes::Draw + player,
        },
    }
}

fn input_to_selection(input: &str) -> Selection {
    match input {
        "A" | "X" => Selection::Rock,
        "B" | "Y" => Selection::Paper,
        "C" | "Z" => Selection::Scissors,
        _ => panic!(),
    }
}

fn input_to_choice(input: &str) -> Selection {
    match input {
        "A X" => Selection::Scissors,
        "A Y" => Selection::Rock,
        "A Z" => Selection::Paper,
        "B X" => Selection::Rock,
        "B Y" => Selection::Paper,
        "B Z" => Selection::Scissors,
        "C X" => Selection::Paper,
        "C Y" => Selection::Scissors,
        "C Z" => Selection::Rock,
        _ => panic!(),
    }
}

fn main() {
    let file = File::open("input2.txt").expect("Could not open file");

    let mut total = 0;

    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            total += get_score(
                input_to_selection(&line[0..1]),
                input_to_selection(&line[line.len() - 1..]),
            );
        }
    }

    let file = File::open("input2.txt").expect("Could not open file");
    let mut part_2_total = 0;

    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            part_2_total += get_score(input_to_selection(&line[0..1]), input_to_choice(&line));
        }
    }

    println!("Part 1 total: {}", total);
    println!("Part 2 total: {}", part_2_total);
}
