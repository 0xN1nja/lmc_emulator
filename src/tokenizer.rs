#![allow(unused)]

use std::collections::HashMap;
use std::fs::{self, *};
use std::io::{self, *};
use std::path::Path;

const INSTRUCTIONS: [&str; 11] = [
    "ADD", "SUB", "STA", "LDA", "BRA", "BRZ", "BRP", "INP", "OUT", "HLT", "DAT",
];

pub fn generate_tokenized_instructions(source_path: &Path) -> Result<Vec<i64>> {
    let source_file = File::open(source_path)?;

    let buffer = BufReader::new(source_file);
    let mut preliminary_buffer = BufReader::new(File::open(source_path)?);

    let mut tokenized_instructions: Vec<i64> = Vec::new();
    let mut labels: HashMap<String, usize> = HashMap::new();

    let is_label = |s: &str| !INSTRUCTIONS.contains(&s);

    let mut memory_location = 0;

    for line in preliminary_buffer.lines() {
        let mut line = line?;

        if let Some(index) = line.find("//") {
            line.truncate(index);
        }

        if line.trim().is_empty() {
            continue;
        }

        let mut parts = line.trim().split_whitespace();

        if let Some(first) = parts.next() {
            if is_label(first) {
                labels.insert(first.to_owned(), memory_location);
            }
        }

        memory_location += 1;
    }

    let resolve_or_parse = move |s: &str, line_number: i32| -> i64 {
        labels.get(s).copied().map(|n| n as i64).unwrap_or_else(|| {
            s.parse().expect(
                format!("Error On Line {}: Label {} Is Not Defined", line_number, s).as_str(),
            )
        })
    };

    let mut line_number = 1;

    for line in buffer.lines() {
        let mut line = line?;

        if let Some(index) = line.find("//") {
            line.truncate(index);
        }

        if line.trim().is_empty() {
            continue;
        }

        let instructions_vector: Vec<&str> = line.trim().split_whitespace().collect();

        let mut idx = 0;

        if is_label(instructions_vector[0]) {
            idx = 1;
        }

        match instructions_vector
            .get(idx)
            .expect(format!("Unknown Instruction: {}", line).as_str())
            .to_uppercase()
            .as_str()
        {
            "ADD" => tokenized_instructions.push(
                100 + resolve_or_parse(
                    instructions_vector.get(idx + 1).unwrap_or(&"0"),
                    line_number,
                ),
            ),
            "SUB" => tokenized_instructions.push(
                200 + resolve_or_parse(
                    instructions_vector.get(idx + 1).unwrap_or(&"0"),
                    line_number,
                ),
            ),
            "STA" => tokenized_instructions.push(
                300 + resolve_or_parse(
                    instructions_vector.get(idx + 1).unwrap_or(&"0"),
                    line_number,
                ),
            ),
            "LDA" => {
                tokenized_instructions.push(
                    500 + resolve_or_parse(
                        instructions_vector.get(idx + 1).unwrap_or(&"0"),
                        line_number,
                    ),
                );
            }
            "BRA" => tokenized_instructions.push(
                600 + resolve_or_parse(
                    instructions_vector.get(idx + 1).unwrap_or(&"0"),
                    line_number,
                ),
            ),
            "BRZ" => tokenized_instructions.push(
                700 + resolve_or_parse(
                    instructions_vector.get(idx + 1).unwrap_or(&"0"),
                    line_number,
                ),
            ),
            "BRP" => tokenized_instructions.push(
                800 + resolve_or_parse(
                    instructions_vector.get(idx + 1).unwrap_or(&"0"),
                    line_number,
                ),
            ),
            "INP" => tokenized_instructions.push(901),
            "OUT" => tokenized_instructions.push(902),
            "HLT" => tokenized_instructions.push(000),
            "DAT" => {
                if instructions_vector.len() > idx + 1 {
                    tokenized_instructions.push(
                        000 + instructions_vector
                            .get(idx + 1)
                            .unwrap_or(&"0")
                            .parse::<i64>()
                            .expect(
                                format!(
                                    "Error on line {}: Invalid Argument For DAT Instruction: {}",
                                    line_number,
                                    line.split_whitespace().nth(idx + 1).unwrap()
                                )
                                .as_str(),
                            ),
                    );
                } else {
                    tokenized_instructions.push(000);
                }
            }
            _ => {
                panic!("Unknown Instruction: {}", line);
            }
        }

        line_number += 1;
    }

    Ok(tokenized_instructions)
}

pub fn generate_binary(tokenized_instructions: Vec<i64>, binary_path: &Path) -> Result<()> {
    let mut content = String::new();

    for tokenized_instruction in tokenized_instructions {
        content.push_str(format!("{:03}", tokenized_instruction).as_str());
    }

    let mut binary = File::create(binary_path)?;

    binary.write_all(content.as_bytes())?;

    Ok(())
}

pub fn generate_instructions(binary_path: &Path) -> Result<Vec<i64>> {
    let binary_file = File::open(binary_path)?;
    let mut buffer = BufReader::new(binary_file);
    let mut content = String::new();

    buffer.read_to_string(&mut content)?;

    let mut tokenized_instructions: Vec<i64> = Vec::new();

    for chunk in content.as_bytes().chunks(3) {
        let instruction_str = std::str::from_utf8(chunk).unwrap();
        let instruction = instruction_str.parse::<i64>().unwrap();

        tokenized_instructions.push(instruction);
    }

    Ok(tokenized_instructions)
}
