mod input;

use std::fmt::Display;

struct Circut {
    cycle: u32,
    remaining_cycles: u32,
    command: Option<Command>,
    register: i32,
}

impl Circut {
    fn new() -> Self {
        return Circut {
            cycle: 1,
            remaining_cycles: 0,
            command: None,
            register: 1,
        };
    }

    fn assign_command(&mut self, cmd: Command) {
        self.command = Some(cmd);
        self.remaining_cycles = cmd.duration;
    }

    fn run_cycle(&mut self) {
        match self.command {
            Some(cmd) => match cmd.command_type {
                CommandType::Noop => (),
                CommandType::Addx { value } => {
                    if self.remaining_cycles == 1 {
                        self.register += value
                    }
                }
            },
            None => return,
        }
        self.cycle += 1;

        self.remaining_cycles -= 1;
        if self.remaining_cycles == 0 {
            self.command = None;
        }
        return;
    }
}

impl Display for Circut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Cycle: {}; reg {}", self.cycle, self.register);
    }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Copy, Clone, Debug)]
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
    let cmds: Vec<Command> = input::REAL_INPUT
        .split("\n")
        .map(|line| Command::parse_command(line))
        .collect();

    let mut c = Circut::new();

    let signal_strenghts: i32 = cmds.iter().fold(0, |mut accum: i32, cmd| {
        if c.command.is_none() {
            c.assign_command(cmd.clone())
        };
        while c.command.is_some() {
            println!("cmd: {:?}", c.command);
            c.run_cycle();
            println!("c from the cycle: {}", c);
            if c.cycle == 20 || ((c.cycle > 21) && (((c.cycle - 20) % 40) == 0)) {
                let signal_strength = i32::try_from(c.cycle).ok().unwrap() * c.register;
                println!("signal_strength: {signal_strength}");
                accum += signal_strength;
            }
        }
        return accum;
    });

    println!("c: {}", c);
    println!("signal_strenghts: {}", signal_strenghts);
}
