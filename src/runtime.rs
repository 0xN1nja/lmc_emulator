#![allow(unused)]

use crate::instructions::{self, *};
use crate::memory::Memory;

use std::io::{self, *};

pub fn execute(memory: &mut Memory, tokenized_instructions: Vec<i64>) {
    let mut accumulator = 0;
    let mut program_counter = 0;

    while program_counter < tokenized_instructions.len() {
        let code = tokenized_instructions[program_counter];
        let instruction = Instruction::parse(code).unwrap();
        let mut should_increment = true;

        match instruction.kind {
            InstructionType::ADD => {
                println!(
                    "Adding The Content Of Memory Address {:?} To Accumulator",
                    instruction.address.unwrap()
                );

                accumulator += memory.items[instruction.address.unwrap() as usize];
            }
            InstructionType::SUB => {
                println!(
                    "Subtracting The Content Of Memory Address {:?} From Accumulator",
                    instruction.address.unwrap()
                );

                accumulator -= memory.items[instruction.address.unwrap() as usize];
            }
            InstructionType::STA => {
                println!(
                    "Storing Value Of Accumulator At Memory Address {:?}",
                    instruction.address.unwrap()
                );

                memory.items[instruction.address.unwrap() as usize] = accumulator;
            }
            InstructionType::LDA => {
                println!(
                    "Loading The Contents Of Memory Address {:?} Into Accumulator",
                    instruction.address.unwrap()
                );

                accumulator = memory.items[instruction.address.unwrap() as usize];
            }
            InstructionType::BRA => {
                program_counter = instruction.address.unwrap() as usize;
                should_increment = false;
            }
            InstructionType::BRZ => {
                if accumulator == 0 {
                    program_counter = instruction.address.unwrap() as usize;
                    should_increment = false;
                }
            }
            InstructionType::BRP => {
                if accumulator >= 0 {
                    program_counter = instruction.address.unwrap() as usize;
                    should_increment = false;
                }
            }
            InstructionType::INP => {
                print!("Input: ");

                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let input_number = input.trim().parse::<i64>().unwrap();

                accumulator = input_number;
            }
            InstructionType::OUT => {
                println!("Output: {:?}", accumulator);
            }
            InstructionType::HLT => {
                break;
            }
            InstructionType::DAT => {}
        }

        if should_increment {
            program_counter += 1;
        }
    }

    println!("Program Halted");
    println!("Memory Table:");

    Memory::print_memory_table(&memory);
}
