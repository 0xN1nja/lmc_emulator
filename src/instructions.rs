#![allow(unused)]

#[derive(Debug)]
pub struct Instruction {
    pub kind: InstructionType,
    pub address: Option<i64>,
}

impl Instruction {
    pub fn parse(code: i64) -> Result<Instruction, &'static str> {
        let instruction_code = format!("{:03}", code);
        let instruction = instruction_code
            .chars()
            .nth(0)
            .unwrap()
            .to_digit(10)
            .unwrap();
        let address = instruction_code[1..].parse().unwrap();

        if instruction_code == "000" {
            return Ok(Instruction {
                kind: InstructionType::HLT,
                address: None,
            });
        }

        let kind = match instruction {
            1 => InstructionType::ADD,
            2 => InstructionType::SUB,
            3 => InstructionType::STA,
            5 => InstructionType::LDA,
            6 => InstructionType::BRA,
            7 => InstructionType::BRZ,
            8 => InstructionType::BRP,
            9 => {
                match instruction_code
                    .chars()
                    .nth(2)
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                {
                    1 => InstructionType::INP,
                    2 => InstructionType::OUT,
                    _ => return Err("Unknown Instruction"),
                }
            }
            0 => InstructionType::DAT,
            _ => return Err("Unknown Instruction"),
        };

        Ok(Instruction {
            kind,
            address: if instruction == 0 || instruction == 9 {
                None
            } else {
                Some(address)
            },
        })
    }
}

#[derive(Debug)]
pub enum InstructionType {
    ADD,
    SUB,
    STA,
    LDA,
    BRA,
    BRZ,
    BRP,
    INP,
    OUT,
    HLT,
    DAT,
}
