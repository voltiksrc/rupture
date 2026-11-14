use crate::decode::{Instruction, decode};
use std::io::{self, Write};

pub struct Vm {
    memory: Vec<u8>,
    pc: usize, // Program counter; moves through the instructions
    mp: usize, // Memory pointer; moves through the memory cells
    halted: bool,
}

impl Vm {
    pub fn new(source: &[u8]) -> Result<Self, String> {
        let mut memory = vec![0u8; 65_536]; // 64 KiB of shared memory

        if source.len() >= memory.len() {
            return Err("Source code exceeds available memory".into());
        }

        memory[..source.len()].copy_from_slice(source);

        Ok(Self {
            memory,
            pc: 0,
            mp: source.len(),
            halted: false,
        })
    }

    /// Executes a Rupture instruction at the current program counter
    ///
    /// Returns an error if the instruction is invalid
    pub fn step(&mut self) -> Result<(), String> {
        let (instruction, length) = decode(&self.memory, self.pc)?;

        match instruction {
            Instruction::Increment => {
                // Increment the selected cell which wraps 255 back to 0.
                self.memory[self.mp] = self.memory[self.mp].wrapping_add(1);
            }
            Instruction::Decrement => {
                self.memory[self.mp] = self.memory[self.mp].wrapping_sub(1);
            }
            Instruction::MoveRight => {
                if self.mp + 1 >= self.memory.len() {
                    return Err("Memory pointer out of bounds.".into());
                }

                self.mp += 1;
            }
            Instruction::MoveLeft => {
                if self.mp == 0 {
                    return Err("Memory pointer out of bounds.".into());
                }

                self.mp -= 1;
            }
            Instruction::Print(text) => {
                print!("{text}");
                // Flush to make the output appear immediately
                io::stdout().flush().map_err(|e| e.to_string())?;
            }
            Instruction::Copy(source, destination) => {
                let byte = *self.memory.get(source).ok_or("Invalid source address")?;

                let cell = self
                    .memory
                    .get_mut(destination)
                    .ok_or("Invalid destination address")?;

                *cell = byte;
            }
            Instruction::Jump(address) => {
                if address >= self.memory.len() {
                    return Err("Jump address out of bounds".into());
                }

                self.pc = address;
                return Ok(());
            }
            Instruction::Halt => {
                self.halted = true;
            }

            _ => return Err("Instruction not implemented yet.".into()),
        }

        self.pc += length;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        while !self.halted {
            self.step()?;
        }

        Ok(())
    }
}
