mod input;

use std::fmt;
use termion::{color, style};

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: u32,
    visible: bool,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self.visible {
            true => write!(
                f,
                "{}{}{}",
                color::Fg(color::Green),
                self.height,
                style::Reset
            ),
            false => write!(
                f,
                "{}{}{}",
                color::Fg(color::LightBlack),
                self.height,
                style::Reset
            ),
        };
    }
}

#[derive(Debug)]
struct Forest(Vec<Vec<Tree>>);

impl Forest {
    fn get_tree(&mut self, x: usize, y: usize) -> Option<Tree> {
        if x > self.0[0].len() || y > self.0.len() {
            return None;
        }

        return Some(*self.0.get(y).unwrap().get(x).unwrap());
    }

    fn mark_visible_trees(&mut self) {
        // left to right
        self.0
            .iter_mut()
            .for_each(|row| mark_visible_trees_in_row(row));
        // top to bottom
        for x in 0..self.0.first().unwrap().len() {
            let mut tree_row: Vec<Tree> = vec![];
            for y in 0..self.0.len() {
                tree_row.push(self.get_tree(y, x).unwrap());
            }
            mark_visible_trees_in_row(&mut tree_row);
            println!("tree_row: {:?}", tree_row);
            let mut tree_row_iter = tree_row.iter();
            for y in 0..self.0.len() {
                self.0[y][x].visible = tree_row_iter.next().unwrap().visible;
            }
        }

        // right to left
        self.0.iter_mut().for_each(|row| {
            row.reverse();
            mark_visible_trees_in_row(row);
            row.reverse();
        });
        // bottom to top
        for x in 0..self.0.first().unwrap().len() {
            let mut tree_row: Vec<Tree> = vec![];
            for y in (0..self.0.len()).rev() {
                tree_row.push(self.get_tree(y, x).unwrap());
            }
            mark_visible_trees_in_row(&mut tree_row);
            println!("tree_row: {:?}", tree_row);
            let mut tree_row_iter = tree_row.iter();
            for y in 0..self.0.len() {
                self.0[y][x].visible = tree_row_iter.next().unwrap().visible;
            }
        }
    }

    fn count_visible(&self) -> usize {
        return self
            .0
            .iter()
            .map(|row| row.iter().filter(|tree| tree.visible).count())
            .sum();
    }

    fn tree_scenic_score(&mut self, x: usize, y: usize) -> Option<usize> {
        let mut score: usize;

        let mut row: Vec<Tree> = vec![];
        for x_ in x..self.0.iter().next().unwrap().len() {
            row.push(self.get_tree(x_, y).unwrap());
        }
        println!("row 1: {:?}", row);
        score = visible_from_a_tree_in_a_row(&row)? as usize;
        println!("score 1: {}", score);

        let mut row: Vec<Tree> = vec![];
        for y_ in std::ops::RangeInclusive::new(0, y).rev() {
            println!("y_: {:?}", y_);
            let t = self.get_tree(x, y_).unwrap();
            println!("t: {:?}", t);
            row.push(t);
        }
        println!("row 2: {:?}", row);
        score = score * visible_from_a_tree_in_a_row(&row)? as usize;
        println!("score 2: {}", score);

        let mut row: Vec<Tree> = vec![];
        for x_ in std::ops::RangeInclusive::new(0, x).rev() {
            row.push(self.get_tree(x_, y).unwrap());
        }
        println!("row 3: {:?}", row);
        score = score * visible_from_a_tree_in_a_row(&row)? as usize;
        println!("score 3: {}", score);

        let mut row: Vec<Tree> = vec![];
        for y_ in y..self.0.iter().next().unwrap().len() {
            row.push(self.get_tree(x, y_).unwrap());
        }
        println!("row 4: {:?}", row);
        score = score * visible_from_a_tree_in_a_row(&row)? as usize;
        println!("score 4: {}", score);

        return Some(score);
    }
}

impl FromIterator<Vec<Tree>> for Forest {
    fn from_iter<T: IntoIterator<Item = Vec<Tree>>>(iter: T) -> Self {
        let mut f = Forest(Vec::new());

        for tree_row in iter {
            f.0.push(tree_row);
        }

        return f;
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "\n{}",
            self.0
                .iter()
                .map(|tree_row| {
                    let mut t_r = tree_row
                        .iter()
                        .map(|tree| tree.to_string())
                        .collect::<String>();
                    t_r.push('\n');
                    return t_r;
                })
                .collect::<String>()
        );
    }
}

fn parse_to_forest(input: &str) -> Forest {
    let mut forest: Forest = input
        .split("\n")
        .map(|row| {
            let tree_row: Vec<Tree> = row
                .chars()
                .map(|c| Tree {
                    height: c.to_digit(10).unwrap(),
                    visible: false,
                })
                .collect();

            return tree_row;
        })
        .collect();

    forest.0.reverse();
    return forest;
}

fn mark_visible_trees_in_row(row: &mut Vec<Tree>) -> () {
    let mut max_height: u32 = 0;
    let mut first_tree = true;
    row.iter_mut().for_each(|tree| {
        if tree.height > max_height || first_tree {
            tree.visible = true;
            max_height = tree.height;
            if first_tree {
                first_tree = false
            };
        }
    });
}
fn visible_from_a_tree_in_a_row(row: &Vec<Tree>) -> Option<u32> {
    if row.len() == 0 {
        return None;
    }

    let tree_height: u32 = row[0].height;
    let mut result = 0;
    let mut first_tree = true;

    for tree in row.iter().skip(1) {
        if (tree_height > tree.height) || first_tree {
            result += 1;
            if first_tree {
                first_tree = false
            };
        } else if tree_height <= tree.height {
            result += 1;
            break;
        } else {
            break;
        }
    }

    return Some(result);
}

#[test]
fn visible_from_a_tree_test() {
    let tree_row = vec![
        Tree {
            height: 5,
            visible: false,
        },
        Tree {
            height: 4,
            visible: false,
        },
        Tree {
            height: 6,
            visible: false,
        },
        Tree {
            height: 1,
            visible: false,
        },
    ];

    assert_eq!(visible_from_a_tree_in_a_row(&tree_row).unwrap(), 2);

    let tree_row_2 = vec![
        Tree {
            height: 5,
            visible: false,
        },
        Tree {
            height: 5,
            visible: false,
        },
        Tree {
            height: 6,
            visible: false,
        },
        Tree {
            height: 1,
            visible: false,
        },
    ];

    assert_eq!(visible_from_a_tree_in_a_row(&tree_row_2).unwrap(), 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_1_test_input() {
        let mut f = parse_to_forest(input::TEST_INPUT);
        f.mark_visible_trees();
        assert_eq!(f.count_visible(), 21);
    }
}

fn main() {
    let mut f = parse_to_forest(input::REAL_INPUT);

    f.mark_visible_trees();

    println!("forest: {}", f);

    println!("problem 1: {}", f.count_visible());

    println!(
        "(2,1) tree scenic score: {}",
        f.tree_scenic_score(2, 1).unwrap_or(0)
    );

    let mut max_scenic_score: usize = 0;
    for x in 0..f.0[0].len() {
        for y in 0..f.0.len() {
            let s = f.tree_scenic_score(x, y);
            if s.unwrap_or(0) > max_scenic_score {
                max_scenic_score = s.unwrap();
            }
        }
    }

    println!("max_scenic_score: {:?}", max_scenic_score);
}
