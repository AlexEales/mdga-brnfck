use std::fs;
use std::env;
use std::io::Read;
use std::path::Path;

// TODO: Write tests for this whole thing.

#[derive(Copy, Clone, Debug, PartialEq)]
enum Command {
    Increment,
    Decrement,
    ShiftRight,
    ShiftLeft,
    Output,
    Input,
    LoopStart(usize),
    LoopEnd(usize),
    Ignore,
    End,
}

impl From<char> for Command {
    fn from(value: char) -> Self {
        match value {
            '>' => Command::ShiftRight,
            '<' => Command::ShiftLeft,
            '+' => Command::Increment,
            '-' => Command::Decrement,
            '.' => Command::Output,
            ',' => Command::Input,
            '[' => Command::LoopStart(0),
            ']' => Command::LoopEnd(0),
            _ => Command::Ignore,
        }
    }
}

struct BrnFckInterpreter {
    data_pointer: usize,
    instruction_pointer: usize,
    loops: Vec<usize>,
    memory: [u8; 30000],
}

impl BrnFckInterpreter {
    fn new() -> Self {
        BrnFckInterpreter {
            data_pointer: 0,
            instruction_pointer: 0,
            loops: Vec::new(),
            memory: [0; 30000]
        }
    }

    fn reset(&mut self) {
        self.data_pointer = 0;
        self.instruction_pointer = 0;
        self.loops = Vec::new();
        self.memory = [0; 30000];
    }

    fn interpret(&mut self, source: String) -> Vec<Command> {
        // Parse the source into commands.
        let mut commands: Vec<Command> = Vec::new();
        // Use a stack of loops and pop when one is closed to assign jump indexes.
        for (idx, symbol) in source.chars().enumerate() {
            let mut command = Command::from(symbol);
            match command {
                Command::LoopStart(_) => {
                    // Put onto stack.
                    self.loops.push(idx);
                },
                Command::LoopEnd(_) => {
                    // Pop stack and assign jump index.
                    let loop_idx = self.loops.pop().unwrap();
                    command = Command::LoopEnd(loop_idx - 1);
                    commands[loop_idx] = Command::LoopStart(idx);
                },
                _ => {},
            }
            commands.push(command);
        }
        // If there are any loops left then terminate
        if self.loops.len() > 0 {
            panic!("Mismatched loops!");
        }
        // Append a END command onto the end.
        commands.push(Command::End);
        commands
    }

    fn execute(&mut self, source: String) {
        // Interpret source.
        let commands = self.interpret(source);
        // Execute the commands.
        let mut command = commands[self.instruction_pointer];
        while command != Command::End {
            command = commands[self.instruction_pointer];
            match command {
                Command::LoopStart(jump_idx) => {;
                    if self.memory[self.data_pointer] == 0 {
                        self.instruction_pointer = jump_idx;
                    }
                },
                Command::LoopEnd(start_idx) => {
                    self.instruction_pointer = start_idx;
                },
                _ => self.execute_command(command),
            }
            self.instruction_pointer += 1;
        }
        // Reset the interpreter
        self.reset();
    }

    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Increment => self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_add(1),
            Command::Decrement => self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_sub(1),
            Command::ShiftRight => self.data_pointer = self.data_pointer.wrapping_add(1),
            Command::ShiftLeft => self.data_pointer = self.data_pointer.wrapping_sub(1),
            Command::Output => print!("{}", self.memory[self.data_pointer] as char),
            Command::Input => {
                let input: Option<u8> = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8);
                self.memory[self.data_pointer] = input.unwrap();
            },
            _ => {},
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check file has been specified.
    if args.len() <= 1 {
        panic!("No file specified!");
    }
    let file_path = &args[1];
    // Check file exists.
    if !Path::new(file_path).exists() {
        panic!("File does not exist!");
    }
    // Load file contents and execute the program.
    let file_contents = fs::read_to_string(file_path).expect("Error occured when reading file!");
    let mut interpreter = BrnFckInterpreter::new();
    interpreter.execute(file_contents);
}
