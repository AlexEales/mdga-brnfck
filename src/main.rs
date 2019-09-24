use std::convert::TryFrom;

#[derive(Debug)]
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
    data_pointer: usize,
    instruction_pointer: usize,
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

    fn execute(&mut self, source: String) {
        // Parse the source into commands.
        let mut commands: Vec<Command> = Vec::new();
        for symbol in source.chars() {
            commands.push(Command::try_from(symbol).unwrap());
        }
        // Execute the commands.
        for command in commands {
            match command {
                // TODO: Consider how we want to handle loops (maybe store the start/end in enum)?
                //       This may be an extra step in the parsing phase, where loops are calculated.
                Command::LoopStart => {},
                Command:: LoopEnd => {},
                _ => self.execute_command(command),
            }
            self.instruction_pointer += 1;
        }
    }

    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Increment => self.memory[self.data_pointer] += 1,
            Command::Decrement => self.memory[self.data_pointer] -= 1,
            Command::ShiftRight => self.data_pointer += 1,
            Command::ShiftLeft => self.data_pointer -= 1,
            Command::Output => print!("{}", self.memory[self.data_pointer]),
            Command::Input => {}, // TODO: IMPLEMENT
            _ => {}, // TODO: Loops hit this should it error?
        }
    }
}

fn main() {
    let mut interpreter = BrnFckInterpreter::new();
    interpreter.execute(String::from("++."));
}
