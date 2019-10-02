use std::fs;
use std::char;
use std::io::Read;
use std::path::Path;

const VALID_SYMBOLS: [char; 8] = ['>', '<', '+', '-', '.', ',', '[', ']'];
const MEMORY_SIZE: usize = 30000;

enum Instruction {
    ShiftRight,
    ShiftLeft,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart(usize),
    LoopEnd(usize),
}

fn parse(source: &str) -> Vec<Instruction> {
    // Get all the symbols from the source, discarding invalid symbols.
    let symbols: Vec<char> = source.chars()
        .filter(|s| VALID_SYMBOLS.contains(s))
        .collect();
    let mut instructions: Vec<Instruction> = Vec::new();
    // Create closure for finding matching brackets. (by @shritesh from dev.to)
    let find_matching_bracket = |open, close, start_idx, stop_idx| {
        let mut idx = start_idx;
        let mut open_brackets = 1;
        // Loop until the matching bracket is found or a panic is caused.
        loop {
            // If the current symbol is an "open bracket" then increment the number of open brackets.
            if symbols[idx] == open {
                open_brackets += 1;
            } else if symbols[idx] == close { // Else if its a "closing" bracket decrement the bracket count.
                open_brackets -= 1;
            }
            // If there are no brackets left open then return the index.
            if open_brackets == 0 {
                return idx;
            }
            // If the index is the stop index and there are unmatched brackets then error.
            if idx == stop_idx {
                panic!("Unmatched brackets!");
            } else if start_idx < stop_idx { // Otherwise increase the index.
                idx += 1;
            } else { // Or go back.
                idx -= 1;
            }
        }
    };
    // Convert the symbols to instructions.
    for i in 0..symbols.len() {
        match symbols[i] {
            '>' => instructions.push(Instruction::ShiftRight),
            '<' => instructions.push(Instruction::ShiftLeft),
            '+' => instructions.push(Instruction::Increment),
            '-' => instructions.push(Instruction::Decrement),
            '.' => instructions.push(Instruction::Output),
            ',' => instructions.push(Instruction::Input),
            '[' => instructions.push(Instruction::LoopStart(
                find_matching_bracket('[', ']', i + 1, symbols.len()) // Check till end
            )),
            ']' => instructions.push(Instruction::LoopEnd(
                find_matching_bracket(']', '[', i - 1, 0) // Check backwards
            )),
            _ => {},
        }
    }
    instructions
}

// TODO: use these functions in a REPL or simple binary.
// TODO: Abstract this so it has a closure (so it could capture a predefined input) passed in to handle IO?
pub fn run(source: &str) {
    let instructions = parse(source);
    let mut memory = [0u8; MEMORY_SIZE];
    let mut instruction_ptr: usize = 0;
    let mut memory_ptr: usize = 0;
    // Run program
    while let Some(instruction) = instructions.get(instruction_ptr) {
        match *instruction {
            Instruction::ShiftRight => if memory_ptr + 1 == MEMORY_SIZE {
                memory_ptr = 0;
            } else {
                memory_ptr += 1;
            },
            Instruction::ShiftLeft => if memory_ptr == 0 {
                memory_ptr = MEMORY_SIZE - 1;
            } else {
                memory_ptr -= 1;
            },
            Instruction::Increment => memory[memory_ptr] += 1,
            Instruction::Decrement => memory[memory_ptr] -= 1,
            Instruction::Output => print!("{}", char::from_u32(memory[memory_ptr] as u32).unwrap()),
            Instruction::Input => {
                let input: Option<u8> = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8);
                memory[memory_ptr] = input.unwrap();
            },
            Instruction::LoopStart(idx) => if memory[memory_ptr] == 0 {
                instruction_ptr = idx;
            },
            Instruction::LoopEnd(idx) => if memory[memory_ptr] != 0 {
                instruction_ptr = idx;
            }
        }
        // Increment the instruction pointer.
        instruction_ptr += 1;
    }
}

pub fn run_with_input(source: &str, input: &str) -> String {
    let instructions = parse(source);
    let mut input_chars = input.chars();
    let mut memory = [0u8; MEMORY_SIZE];
    let mut instruction_ptr: usize = 0;
    let mut memory_ptr: usize = 0;
    let mut output = String::new();
    // Run program
    while let Some(instruction) = instructions.get(instruction_ptr) {
        match *instruction {
            Instruction::ShiftRight => if memory_ptr + 1 == MEMORY_SIZE {
                memory_ptr = 0;
            } else {
                memory_ptr += 1;
            },
            Instruction::ShiftLeft => if memory_ptr == 0 {
                memory_ptr = MEMORY_SIZE - 1;
            } else {
                memory_ptr -= 1;
            },
            Instruction::Increment => memory[memory_ptr] = memory[memory_ptr].wrapping_add(1),
            Instruction::Decrement => memory[memory_ptr] = memory[memory_ptr].wrapping_sub(1),
            Instruction::Output => output.push(char::from_u32(memory[memory_ptr] as u32).unwrap()),
            Instruction::Input => memory[memory_ptr] = input_chars.next().unwrap_or('\0') as u8,
            Instruction::LoopStart(idx) => if memory[memory_ptr] == 0 {
                instruction_ptr = idx;
            },
            Instruction::LoopEnd(idx) => if memory[memory_ptr] != 0 {
                instruction_ptr = idx;
            }
        }
        // Increment the instruction pointer.
        instruction_ptr += 1;
    }
    output
}

pub fn run_from_file(file_path: &str) {
    // Check file exists.
    if !Path::new(file_path).exists() {
        panic!("File does not exist!");
    }
    // Load file contents and execute the program.
    let file_contents = fs::read_to_string(file_path).expect("Error occured when reading file!");
    run(&file_contents)
}

pub fn run_from_file_with_input(file_path: &str, input: &str) -> String {
    // Check file exists.
    if !Path::new(file_path).exists() {
        panic!("File does not exist!");
    }
    // Load file contents and execute the program.
    let file_contents = fs::read_to_string(file_path).expect("Error occured when reading file!");
    run_with_input(&file_contents, input)
}

#[cfg(test)]
mod test {
    use super::{run_with_input, run_from_file_with_input};

    #[test]
    fn hello_world() {
        let src = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        assert_eq!(run_with_input(src, ""), "Hello World!\n");
    }

    #[test]
    fn hello_world_from_file() {
        assert_eq!(run_from_file_with_input("./programs/hello_world.b", ""), "Hello World!\n");
    }

    #[test]
    fn christmas_tree_from_file() {
        assert_eq!(run_from_file_with_input("./programs/christmas.b", ""), "*\n");
        assert_eq!(run_from_file_with_input("./programs/christmas.b", "12"), "            *\n           ***\n          *****\n         *******\n        *********\n       ***********\n      *************\n     ***************\n    *****************\n   *******************\n  *********************\n ***********************\n            *\n");
    }

    #[test]
    fn bubble_sort_from_file() {
        assert_eq!(run_from_file_with_input("./programs/bubble_sort.b", "192837465"), "123456789");
    }

    #[test]
    fn cat() {
        let src = ",[.,]";
        assert_eq!(run_with_input(src, "hello"), "hello");
    }

    #[test]
    fn quine() {
        // Written by Erik Bosman
        // https://copy.sh/brainfuck/prog/quine505.b
        let src = r">+++++>+++>+++>+++++>+++>+++>+++++>++++++>+>++>+++>++++>++++>+++>+++>+++++>+>+>++++>+++++++>+>+++++>+>+>+++++>++++++>+++>+++>++>+>+>++++>++++++>++++>++++>+++>+++++>+++>+++>++++>++>+>+>+>+>++>++>++>+>+>++>+>+>++++++>++++++>+>+>++++++>++++++>+>+>+>+++++>++++++>+>+++++>+++>+++>++++>++>+>+>++>+>+>++>++>+>+>++>++>+>+>+>+>++>+>+>+>++++>++>++>+>+++++>++++++>+++>+++>+++>+++>+++>+++>++>+>+>+>+>++>+>+>++++>+++>+++>+++>+++++>+>+++++>++++++>+>+>+>++>+++>+++>+++++++>+++>++++>+>++>+>+++++++>++++++>+>+++++>++++++>+++>+++>++>++>++>++>++>++>+>++>++>++>++>++>++>++>++>++>+>++++>++>++>++>++>++>++>++>+++++>++++++>++++>+++>+++++>++++++>++++>+++>+++>++++>+>+>+>+>+++++>+++>+++++>++++++>+++>+++>+++>++>+>+>+>++++>++++[[>>>+<<<-]<]>>>>[<<[-]<[-]+++++++[>+++++++++>++++++<<-]>-.>+>[<.<<+>>>-]>]<<<[>>+>>>>+<<<<<<-]>++[>>>+>>>>++>>++>>+>>+[<<]>-]>>>-->>-->>+>>+++>>>>+[<<]<[[-[>>+<<-]>>]>.[>>]<<[[<+>-]<<]<<]";
        assert_eq!(run_with_input(src, ""), src);
    }
}