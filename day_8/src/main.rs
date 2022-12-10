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
    fn get_tree(&self, x: usize, y: usize) -> &mut Tree {
        todo!();
    }

    fn mark_visible_trees(&self) -> () {
        todo!();
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

fn main() {
    let mut f = parse_to_forest(input::TEST_INPUT);

    println!("forest: {}", f);
}
