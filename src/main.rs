mod instructions;
mod memory;
mod runtime;
mod tokenizer;

use std::path::Path;

const HELP: &str = "lmc_emulator: LMC Emulator written in Rust\n\nUsage:\n\tlmc_emulator [--build <filename> | --run <filename>]\n\nCommands:\n\t--build <filename>\tCompile the specified file\n\t--run <filename>\tRun the specified compiled code";

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.contains(&String::from("--build")) {
        let source_index = args.binary_search(&String::from("--build")).unwrap();

        if let Some(source_path) = args.get(source_index + 1) {
            let default_output = String::from("build.lmc");
            let binary_path = args.get(source_index + 2).unwrap_or(&default_output);

            println!("Building source to: {}", binary_path);

            let tokenized_instructions =
                tokenizer::generate_tokenized_instructions(Path::new(source_path)).unwrap();

            tokenizer::generate_binary(tokenized_instructions, Path::new(binary_path)).unwrap();
        } else {
            println!("\"--build\" flag requires a source file. Please provide one");
            return;
        }
    } else if args.contains(&String::from("--run")) {
        let run_index = args.binary_search(&String::from("--run")).unwrap();

        if let Some(binary_path) = args.get(run_index + 1) {
            println!("Running file: {}", binary_path);

            let tokenized_instructions =
                tokenizer::generate_instructions(Path::new(binary_path)).unwrap();
            let mut memory = memory::Memory::new();

            for (i, tokenized_instruction) in tokenized_instructions.clone().into_iter().enumerate()
            {
                memory.items[i] = tokenized_instruction;
            }

            runtime::execute(&mut memory, tokenized_instructions);
        } else {
            println!("\"--run\" flag requires a binary file. Please provide one");
            return;
        }
    } else {
        println!("{}", HELP);
    }
}
