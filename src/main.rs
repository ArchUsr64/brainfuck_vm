const MEMORY_SIZE: usize = 30_000;
use std::io::{self, Write};

struct Machine {
    memory: [u8; MEMORY_SIZE],
    pointer: usize,
    instruction_stack: Vec<Instruction>,
    condition_fail_flag: bool,
    brace_balance: i32,
}
impl Machine {
    fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            pointer: 0,
            instruction_stack: vec![],
            condition_fail_flag: false,
            brace_balance: 0,
        }
    }
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::IncrementPointer => {
                self.pointer += 1;
                if self.pointer == MEMORY_SIZE {
                    self.pointer = 0;
                }
            }
            Instruction::DecrementPointer => {
                if self.pointer == 0 {
                    self.pointer = MEMORY_SIZE;
                }
                self.pointer -= 1;
            }
            Instruction::IncrementByteAtPointer => self.memory[self.pointer] += 1,
            Instruction::DecrementByteAtPointer => self.memory[self.pointer] -= 1,
            Instruction::PrintByte => {
                print!("{}", self.memory[self.pointer] as char);
                let _ = io::stdout().flush();
            }
            Instruction::ReadByte => {
                let mut buffer = String::new();
                io::stdin()
                    .read_line(&mut buffer)
                    .expect("Failed to read from STDIN");
                self.memory[self.pointer] = buffer.chars().nth(0).expect("Invalid Input") as u8;
            }
            Instruction::OpeningBlock | Instruction::ClosingBlock => todo!(),
        }
    }
}

#[derive(PartialEq)]
enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementByteAtPointer,
    DecrementByteAtPointer,
    PrintByte,
    ReadByte,
    OpeningBlock,
    ClosingBlock,
}

impl Instruction {
    fn from_char(char: char) -> Option<Self> {
        match char {
            '>' => Some(Self::IncrementPointer),
            '<' => Some(Self::DecrementPointer),
            '+' => Some(Self::IncrementByteAtPointer),
            '-' => Some(Self::DecrementByteAtPointer),
            '.' => Some(Self::PrintByte),
            ',' => Some(Self::ReadByte),
            '[' => Some(Self::OpeningBlock),
            ']' => Some(Self::ClosingBlock),
            _ => None,
        }
    }
}

fn main() {
    let mut machine = Machine::new();
    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read the STDIN");
        buffer
            .chars()
            .map(|char| Instruction::from_char(char))
            .filter_map(|instruction| instruction)
            .for_each(|instruction| machine.execute(instruction));
    }
}
