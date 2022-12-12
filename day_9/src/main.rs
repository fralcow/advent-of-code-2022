mod input;
use std::fmt::Display;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

struct Command {
    direction: Direction,
    repeat: u32,
}

impl Command {
    fn new(direction: Direction, repeat: u32) -> Self {
        return Command { direction, repeat };
    }

    fn parse_string(input: &str) -> Option<Self> {
        let (direction, repeat) = input.split_once(" ")?;

        let direction = match direction.to_lowercase().as_str() {
            "u" => Direction::Up,
            "r" => Direction::Right,
            "d" => Direction::Down,
            "l" => Direction::Left,
            _ => return None,
        };

        let repeat: u32 = match repeat.parse() {
            Ok(v) => v,
            _ => return None,
        };

        return Some(Command::new(direction, repeat));
    }
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    fn new(x: i32, y: i32) -> Self {
        return Cell { x, y };
    }

    fn touches(&self, other_cell: Cell) -> bool {
        if i32::abs(self.x - other_cell.x) <= 1 && i32::abs(self.y - other_cell.y) <= 1 {
            return true;
        }
        return false;
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            return true;
        }
        return false;
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "[{},{}]", self.x, self.y);
    }
}

#[test]
fn touches_test() {
    let cell_0 = Cell::new(0, 0);
    assert!(cell_0.touches(Cell::new(0, 0)));
    assert!(cell_0.touches(Cell::new(1, 1)));
    assert!(!cell_0.touches(Cell::new(2, 1)));
}

struct Rope {
    head: RopeEnd,
    tail: RopeEnd,
}

impl Rope {
    fn new() -> Self {
        return Rope {
            head: RopeEnd::new(0, 0),
            tail: RopeEnd::new(0, 0),
        };
    }

    fn move_by_head(&mut self, direction: Direction) {
        // move head
        self.head.move_command(direction.clone());

        // check if tail still touches the head
        if self.head.cell.touches(self.tail.cell) {
            return;
        }

        // // if same column/row -> move in same direction as head
        // move tail
        if self.tail.cell.x == self.head.cell.x || self.tail.cell.y == self.head.cell.y {
            self.tail.move_command(direction.clone());
        } else {
            // // if in different column -> move to the cell with the same x or y
            match direction {
                Direction::Up => {
                    if self.tail.cell.x - self.head.cell.x < 0 {
                        self.tail.move_command(Direction::UpRight);
                    } else {
                        self.tail.move_command(Direction::UpLeft);
                    }
                }
                Direction::Right => {
                    if self.tail.cell.y - self.head.cell.y < 0 {
                        self.tail.move_command(Direction::UpRight);
                    } else {
                        self.tail.move_command(Direction::DownRight);
                    }
                }
                Direction::Down => {
                    if self.tail.cell.x - self.head.cell.x < 0 {
                        self.tail.move_command(Direction::DownRight);
                    } else {
                        self.tail.move_command(Direction::DownLeft);
                    }
                }
                Direction::Left => {
                    if self.tail.cell.y - self.head.cell.y < 0 {
                        self.tail.move_command(Direction::UpLeft);
                    } else {
                        self.tail.move_command(Direction::DownLeft);
                    }
                }
                _ => panic!("Unexpected head direction command"),
            }
        }

        // // add the tail cell to the tail visited vector
        if !self.tail.visitedCells.contains(&self.tail.cell) {
            self.tail.visitedCells.push(self.tail.cell.clone());
        }
        return;
    }

    fn execute_command(&mut self, command: &Command) {
        for _i in 0..command.repeat {
            self.move_by_head(command.direction);
        }
    }
}

#[test]
fn test_move_rope_by_head() {
    let mut r = Rope::new();
    assert_eq!(r.tail.visitedCells[0], Cell::new(0, 0));

    r.move_by_head(Direction::Up); // 0,1
    assert_eq!(r.tail.cell, Cell::new(0, 0));

    r.move_by_head(Direction::Up); // 0,2
    assert_eq!(r.tail.cell, Cell::new(0, 1));
    assert_eq!(r.tail.visitedCells[1], Cell::new(0, 1));

    r.move_by_head(Direction::Right); // 1,2
    assert_eq!(r.tail.cell, Cell::new(0, 1));

    r.move_by_head(Direction::Up); // 1,3
    assert_eq!(r.tail.cell, Cell::new(1, 2));
    assert_eq!(r.tail.visitedCells[2], Cell::new(1, 2));
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Head: {}\nTail: {}", self.head.cell, self.tail.cell);
    }
}

struct RopeEnd {
    cell: Cell,
    visitedCells: Vec<Cell>,
}

impl RopeEnd {
    fn new(x: i32, y: i32) -> Self {
        return RopeEnd {
            cell: Cell::new(x, y),
            visitedCells: vec![Cell::new(x, y)],
        };
    }

    fn move_command(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.cell.y += 1,
            Direction::UpRight => {
                self.cell.x += 1;
                self.cell.y += 1
            }
            Direction::Right => self.cell.x += 1,
            Direction::DownRight => {
                self.cell.x += 1;
                self.cell.y -= 1
            }
            Direction::Down => self.cell.y -= 1,
            Direction::DownLeft => {
                self.cell.x -= 1;
                self.cell.y -= 1
            }
            Direction::Left => self.cell.x -= 1,
            Direction::UpLeft => {
                self.cell.x -= 1;
                self.cell.y += 1
            }
        }
    }
}

impl Display for RopeEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.cell);
    }
}

fn main() {
    let commands: Vec<Command> = input::REAL_INPUT
        .split("\n")
        .map(|row| Command::parse_string(row).unwrap_or_else(|| panic!("failed to parse command")))
        .collect();

    let mut r = Rope::new();

    for command in commands {
        r.execute_command(&command);
    }

    println!("r.tail.visitedCells: {:?}", r.tail.visitedCells);
    println!("r.tail.visitedCells.len(): {:?}", r.tail.visitedCells.len());
}
