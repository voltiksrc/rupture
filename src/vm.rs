// SPDX-License-Identifier: GPL-3.0

use crate::decode::decode;
use crate::instructions::Instruction;

use std::io::{self, Read, Write};

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
            Instruction::PrintNumber => {
                print!("{}", self.memory[self.mp]);
                io::stdout().flush().map_err(|e| e.to_string())?;
            }
            Instruction::Input => {
                let mut buffer = [0u8; 1];
                io::stdin()
                    .read_exact(&mut buffer)
                    .map_err(|err| err.to_string())?;

                self.memory[self.mp] = buffer[0];
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
            Instruction::JumpIfZero(address) => {
                if self.memory[self.mp] == 0 {
                    if address >= self.memory.len() {
                        return Err("Jump address out of bounds".into());
                    }

                    self.pc = address;
                    return Ok(());
                }
            }
            Instruction::LoopStart => {
                if self.memory[self.mp] == 0 {
                    let mut depth = 1;

                    for (address, &byte) in self.memory.iter().enumerate().skip(self.pc + 1) {
                        match byte {
                            b'[' => depth += 1,
                            b']' => depth -= 1,
                            _ => {}
                        }

                        if depth == 0 {
                            self.pc = address + 1;
                            return Ok(());
                        }
                    }

                    // Only reached if no matching bracket was found.
                    return Err("Unmatched opening loop bracket '['".into());
                }
            }
            Instruction::LoopEnd => {
                if self.memory[self.mp] != 0 {
                    let mut depth = 1;

                    for (address, &byte) in self.memory[..self.pc].iter().enumerate().rev() {
                        match byte {
                            b']' => depth += 1,
                            b'[' => depth -= 1,
                            _ => {}
                        }

                        if depth == 0 {
                            self.pc = address;
                            return Ok(());
                        }
                    }

                    return Err("Unmatched closing loop bracket ']'".into());
                }
            }
            Instruction::Comment => {}

            Instruction::Whitespace => {}

            Instruction::Halt => {
                self.halted = true;
            }
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
