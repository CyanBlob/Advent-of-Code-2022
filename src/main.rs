use std::fs::File;
use std::io::BufRead;

#[derive(Debug, Eq, Ord, Clone, Copy)]
struct Tree {
    tree: u32,
    added: bool,
    visible_left: usize,
    visible_right: usize,
    visible_up: usize,
    visible_down: usize,
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.tree.partial_cmp(&other.tree) {
            ord => return ord,
        }
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.tree == other.tree
    }
}

fn count_left_to_right(trees: &mut Vec<Vec<Tree>>) -> usize {
    let mut visible: usize = 0;
    let mut checked_rows = 0;
    let height = trees.len();
    let width = trees.iter().nth(0).unwrap().len();

    let mut max_score = 0;

    for row in trees {
        if checked_rows == 0 || checked_rows == height - 1 {
            checked_rows += 1;
            continue;
        }

        checked_rows += 1;

        let _row = row.clone();
        for (i, tree) in row.iter_mut().enumerate() {
            if i + 1 == _row.len() || i == 0 {
                continue;
            }
            if !tree.added
                && (_row[0..i].iter().max().unwrap() < tree
                    || _row[i + 1..].iter().max().unwrap() < tree)
            {
                visible = visible + 1;
                tree.added = true;
            }

            let mut blocked_left = 0;
            if tree.added {
                let visible_left = _row[0..i]
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(j, other_tree)| {
                        if other_tree.tree >= tree.tree {
                            blocked_left = 1;
                            return true;
                        }
                        false
                    })
                    .unwrap_or((
                        i,
                        &Tree {
                            tree: 9,
                            added: false,
                            visible_left: 0,
                            visible_right: 0,
                            visible_up: 0,
                            visible_down: 0,
                        },
                    ));

                let mut blocked_right = 0;
                let visible_right = _row[i + 1..]
                    .iter()
                    .enumerate()
                    .find(|(j, other_tree)| {
                        if other_tree.tree >= tree.tree {
                            blocked_right = 1;
                            return true;
                        }
                        false
                    })
                    .unwrap_or((
                        width - i - 1,
                        &Tree {
                            tree: 9,
                            added: false,
                            visible_left: 0,
                            visible_right: 0,
                            visible_up: 0,
                            visible_down: 0,
                        },
                    ));

                tree.visible_left = visible_left.0 + blocked_left;
                tree.visible_right = visible_right.0 + blocked_right;

                let tree_score =
                    tree.visible_down * tree.visible_left * tree.visible_right * tree.visible_up;

                if tree_score > max_score {
                    max_score = tree_score;
                }
            }
        }
    }

    println!("Max score: {}", max_score);
    visible
}

fn count_top_to_bottom(trees: &Vec<Vec<Tree>>) -> usize {
    let mut rotated_trees: Vec<Vec<Tree>> = vec![];

    let height = trees.len();
    let width = trees.iter().nth(0).unwrap().len();

    for _ in 0..trees.len() {
        rotated_trees.push(vec![]);
    }

    for row in trees {
        for (i, tree) in row.iter().enumerate() {
            let mut rotated_tree = tree.clone();
            rotated_tree.visible_up = rotated_tree.visible_left;
            rotated_tree.visible_down = rotated_tree.visible_right;
            rotated_tree.visible_left = 0;
            rotated_tree.visible_right = 0;
            rotated_trees[i].push(rotated_tree);
        }
    }

    count_left_to_right(&mut rotated_trees) + height * 2 + width * 2 - 4
}

fn main() {
    let mut trees: Vec<Vec<Tree>> = vec![];

    // left to right
    let file = File::open("input8.txt").expect("Could not open file");
    for (row, line) in std::io::BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            trees.push(vec![]);
            trees[row] = line
                .chars()
                .into_iter()
                .map(|t| Tree {
                    tree: t.to_digit(10).unwrap(),
                    added: false,
                    visible_left: 0,
                    visible_right: 0,
                    visible_up: 0,
                    visible_down: 0,
                })
                .collect::<Vec<Tree>>();

        }
    }

    let mut visible: usize = count_left_to_right(&mut trees);

    visible += count_top_to_bottom(&trees);

    println!("Visible: {}", visible);
}
