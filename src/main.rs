use std::convert::TryFrom;

enum Command {
    Increment,
    Decrement,
    ShiftRight,
    ShiftLeft,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

impl TryFrom<char> for Command {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Command::ShiftRight),
            '<' => Ok(Command::ShiftLeft),
            '+' => Ok(Command::Increment),
            '-' => Ok(Command::Decrement),
            '.' => Ok(Command::Output),
            ',' => Ok(Command::Input),
            '[' => Ok(Command::LoopStart),
            ']' => Ok(Command::LoopEnd),
            _ => Err(format!("Cannot convert {} to Command!", value)),
        }
    }
}

struct BrnFckInterpreter {
    data_pointer: u16,
    instruction_pointer: u16,
    memory: [u8; 30000],
}

impl BrnFckInterpreter {
    fn new() -> Self {
        BrnFckInterpreter {
            data_pointer: 0,
            instruction_pointer: 0,
            memory: [0; 30000]
        }
    }
}

fn main() {
    let mut interpreter = BrnFckInterpreter::new();
}
