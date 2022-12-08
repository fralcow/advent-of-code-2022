use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct File {
    size: u64,
}

type Directories<'a> = Vec<Rc<RefCell<Directory<'a>>>>;

impl<'a> Directory<'a> {
    fn size(&'a self) -> usize {
        todo!();
    }
}

#[derive(Default)]
struct Directory<'a> {
    parent: Option<Weak<RefCell<Directory<'a>>>>,
    directories: Directories<'a>,
    name: &'a str,
    files: Vec<File>,
}

#[derive(Default)]
struct Filesystem<'a> {
    root: Directory<'a>,
}

impl<'a> Filesystem<'a> {
    fn parse_to_tree(&'a self, input: &str) -> Filesystem {
        todo!();
    }

    fn get_directories(&'a self) -> Directories<'a> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let fs: Filesystem = Default::default();

        let problem_1_get = fs
            .get_directories()
            .iter()
            .filter(|&&d| d.size() <= 100000)
            .map(|d| d.size())
            .sum();
        assert_eq!(problem_1_get, 95437);
    }
}

fn main() {
    println!("Hello, world!");
}
