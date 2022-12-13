mod input;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Debug)]
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
    knots: Vec<Knot>,
}

impl Rope {
    fn new(knots_number: u32) -> Self {
        let knots: Vec<Knot> = (0..knots_number).map(|_v| Knot::new(0, 0)).collect();

        return Rope { knots };
    }

    fn get_tail(&self) -> &Knot {
        return &self.knots[self.knots.len() - 1];
    }

    fn move_knot(&mut self, direction: Direction, knot_index: usize) {
        // -1 for len = max_index + 1
        if knot_index > self.knots.len() - 1 {
            return;
        }

        // move head
        self.knots[knot_index].move_command(direction.clone());

        // case for the last element
        if knot_index > self.knots.len() - 2 {
            return;
        }

        // check if tail still touches the head
        if self.knots[knot_index]
            .cell
            .touches(self.knots[knot_index + 1].cell)
        {
            return;
        }

        // // if same column/row -> move in same direction as knots[knot_index]
        // move tail
        if (self.knots[knot_index + 1].cell.x == self.knots[knot_index].cell.x
            || self.knots[knot_index + 1].cell.y == self.knots[knot_index].cell.y)
            && [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ]
            .contains(&direction)
        {
            self.move_knot(direction.clone(), knot_index + 1);
        } else {
            // // if in different column -> move to the cell with the same x or y
            match direction {
                Direction::Up => {
                    if self.knots[knot_index + 1].cell.x - self.knots[knot_index].cell.x < 0 {
                        self.move_knot(Direction::UpRight, knot_index + 1);
                    } else {
                        self.move_knot(Direction::UpLeft, knot_index + 1);
                    }
                }
                Direction::UpRight => {
                    if self.knots[knot_index + 1].cell.x == self.knots[knot_index].cell.x {
                        self.move_knot(Direction::Up, knot_index + 1)
                    } else if self.knots[knot_index + 1].cell.y == self.knots[knot_index].cell.y {
                        self.move_knot(Direction::Right, knot_index + 1)
                    } else {
                        self.move_knot(Direction::UpRight, knot_index + 1)
                    };
                }
                Direction::Right => {
                    if self.knots[knot_index + 1].cell.y - self.knots[knot_index].cell.y < 0 {
                        self.move_knot(Direction::UpRight, knot_index + 1);
                    } else {
                        self.move_knot(Direction::DownRight, knot_index + 1);
                    }
                }
                Direction::DownRight => {
                    if self.knots[knot_index + 1].cell.x == self.knots[knot_index].cell.x {
                        self.move_knot(Direction::Down, knot_index + 1)
                    } else if self.knots[knot_index + 1].cell.y == self.knots[knot_index].cell.y {
                        self.move_knot(Direction::Right, knot_index + 1)
                    } else {
                        self.move_knot(Direction::DownRight, knot_index + 1)
                    };
                }
                Direction::Down => {
                    if self.knots[knot_index + 1].cell.x - self.knots[knot_index].cell.x < 0 {
                        self.move_knot(Direction::DownRight, knot_index + 1);
                    } else {
                        self.move_knot(Direction::DownLeft, knot_index + 1);
                    }
                }
                Direction::DownLeft => {
                    if self.knots[knot_index + 1].cell.x == self.knots[knot_index].cell.x {
                        self.move_knot(Direction::Down, knot_index + 1)
                    } else if self.knots[knot_index + 1].cell.y == self.knots[knot_index].cell.y {
                        self.move_knot(Direction::Left, knot_index + 1)
                    } else {
                        self.move_knot(Direction::DownLeft, knot_index + 1)
                    };
                }
                Direction::Left => {
                    if self.knots[knot_index + 1].cell.y - self.knots[knot_index].cell.y < 0 {
                        self.move_knot(Direction::UpLeft, knot_index + 1);
                    } else {
                        self.move_knot(Direction::DownLeft, knot_index + 1);
                    }
                }
                Direction::UpLeft => {
                    if self.knots[knot_index + 1].cell.x == self.knots[knot_index].cell.x {
                        self.move_knot(Direction::Up, knot_index + 1)
                    } else if self.knots[knot_index + 1].cell.y == self.knots[knot_index].cell.y {
                        self.move_knot(Direction::Left, knot_index + 1)
                    } else {
                        self.move_knot(Direction::UpLeft, knot_index + 1)
                    };
                }
            }
        }

        // // add the tail cell to the tail visited vector
        if !self.knots[knot_index + 1]
            .visited_cells
            .contains(&self.knots[knot_index + 1].cell)
        {
            let new_cell = self.knots[knot_index + 1].cell.clone();
            self.knots[knot_index + 1].visited_cells.push(new_cell);
        }
        return;
    }

    fn execute_command(&mut self, command: &Command) {
        println!("command: {:?}", command);
        for _i in 0..command.repeat {
            self.move_knot(command.direction, 0);
            println!("rope:\n{}", self);
        }
    }
}

#[test]
fn test_move_rope_by_head() {
    let mut r = Rope::new(2);
    println!("r: {}", r);
    assert_eq!(r.knots[1].visited_cells[0], Cell::new(0, 0));

    r.move_knot(Direction::Up, 0); // 0,1
    assert_eq!(r.knots[1].cell, Cell::new(0, 0));

    r.move_knot(Direction::Up, 0); // 0,2
    assert_eq!(r.knots[1].cell, Cell::new(0, 1));
    assert_eq!(r.knots[1].visited_cells[1], Cell::new(0, 1));

    r.move_knot(Direction::Right, 0); // 1,2
    assert_eq!(r.knots[1].cell, Cell::new(0, 1));

    r.move_knot(Direction::Up, 0); // 1,3
    assert_eq!(r.knots[1].cell, Cell::new(1, 2));
    assert_eq!(r.knots[1].visited_cells[2], Cell::new(1, 2));
}

#[test]
fn test_diagonal_moves() {
    let mut r = Rope::new(2);
    r.execute_command(&Command::new(Direction::UpRight, 1));
    r.execute_command(&Command::new(Direction::UpLeft, 1));
    assert_eq!(Cell::new(0, 1), r.knots[1].cell);

    let mut r = Rope::new(2);
    r.execute_command(&Command::new(Direction::UpRight, 1));
    r.execute_command(&Command::new(Direction::DownRight, 1));
    assert_eq!(Cell::new(1, 0), r.knots[1].cell);

    let mut r = Rope::new(2);
    r.execute_command(&Command::new(Direction::DownRight, 1));
    r.execute_command(&Command::new(Direction::DownLeft, 1));
    assert_eq!(Cell::new(0, -1), r.knots[1].cell);

    let mut r = Rope::new(2);
    r.execute_command(&Command::new(Direction::DownLeft, 1));
    r.execute_command(&Command::new(Direction::UpLeft, 1));
    assert_eq!(Cell::new(-1, 0), r.knots[1].cell);
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            self.knots
                .iter()
                .enumerate()
                .map(|(i, knot)| [i.to_string(), knot.to_string()].join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}

struct Knot {
    cell: Cell,
    visited_cells: Vec<Cell>,
}

impl Knot {
    fn new(x: i32, y: i32) -> Self {
        return Knot {
            cell: Cell::new(x, y),
            visited_cells: vec![Cell::new(x, y)],
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

impl Display for Knot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.cell);
    }
}

fn main() {
    let commands: Vec<Command> = input::REAL_INPUT
        .split("\n")
        .map(|row| Command::parse_string(row).unwrap_or_else(|| panic!("failed to parse command")))
        .collect();

    let mut r = Rope::new(10);

    for command in commands {
        r.execute_command(&command);
    }

    println!("tail.visitedCells: {:?}", r.get_tail().visited_cells);
    println!(
        "tail.visitedCells.len(): {:?}",
        r.get_tail().visited_cells.len()
    );
}
