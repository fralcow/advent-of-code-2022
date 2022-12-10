mod input;

use std::fmt;
use termion::{color, style};

#[derive(Debug)]
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
    fn get_tree(&mut self, x: usize, y: usize) -> Option<&mut Tree> {
        if x > self.0[0].len() || y > self.0.len() {
            return None;
        }

        return Some(self.0.get_mut(x).unwrap().get_mut(y).unwrap());
    }

    fn mark_visible_trees(&mut self) {
        // left to right
        self.0
            .iter_mut()
            .for_each(|row| mark_visible_trees_in_row(row));
        // top to bottom
        // right to left
        self.0.iter_mut().for_each(|row| {
            row.reverse();
            mark_visible_trees_in_row(row);
            row.reverse();
        });
        // bottom to top
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
    let forest: Forest = input
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

fn main() {
    let mut f = parse_to_forest(input::TEST_INPUT);

    f.mark_visible_trees();

    println!("forest: {}", f);
}
