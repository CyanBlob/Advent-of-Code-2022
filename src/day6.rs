use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let mut ring_buf: VecDeque<char> = VecDeque::<char>::with_capacity(14);

    // part 1 and 2
    for part_requirements in vec![3, 13] {
        let file = File::open("input6.txt").expect("Could not open file");
        for line in std::io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                for (i, char) in line.chars().enumerate() {
                    ring_buf.push_back(char);

                    if i > part_requirements {
                        ring_buf.pop_front();

                        let has_duplicates = ring_buf.iter().enumerate().any(|(i, c)| {
                            ring_buf
                                .range(0..i)
                                .any(|c2| if c2 == c { true } else { false })
                        });

                        if !has_duplicates {
                            println!(
                                "({})Start of packet at location {}: {:?}",
                                part_requirements,
                                i + 1,
                                ring_buf
                            );
                            break;
                        }
                    }
                }
            }
        }
        ring_buf.clear();
    }
}
