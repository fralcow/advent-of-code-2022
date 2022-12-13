mod input;

use std::fmt::Display;

struct Circut {
    cycle: u32,
    register: i32,
}

impl Circut {
    fn new() -> Self {
        return Circut {
            cycle: 0,
            register: 0,
        };
    }

    fn run_cycle(&mut self, command: CommandType) {
        todo!();
    }
}

enum CommandType {
    Addx { value: i32 },
    Noop,
}

impl Display for CommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            CommandType::Addx { value } => write!(f, "addx {}", value),
            CommandType::Noop => write!(f, "noop"),
        };
    }
}

const ADDX_DURATION: u32 = 2;
const NOOP_DURATION: u32 = 1;

struct Command {
    command_type: CommandType,
    duration: u32,
}

impl Command {
    fn new(command_type: CommandType) -> Self {
        return match command_type {
            CommandType::Addx { .. } => Command {
                command_type,
                duration: ADDX_DURATION,
            },
            CommandType::Noop => Command {
                command_type,
                duration: NOOP_DURATION,
            },
        };
    }

    fn parse_command(input: &str) -> Self {
        return match input.split_once(" ") {
            Some(v) => Command::new(CommandType::Addx {
                value: v.1.parse().unwrap(),
            }),
            None => Command::new(CommandType::Noop),
        };
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}; dur {}", self.command_type, self.duration);
    }
}

fn main() {
    println!("Hello, world!");

    let cmds: Vec<Command> = input::TEST_INPUT
        .split("\n")
        .map(|line| Command::parse_command(line))
        .collect();

    cmds.iter().for_each(|cmd| println!("{cmd}"));
}
