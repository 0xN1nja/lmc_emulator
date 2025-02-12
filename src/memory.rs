use tabled::{builder::Builder, settings::Style};

#[derive(Debug)]
pub struct Memory {
    pub items: Vec<i64>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            items: vec![0; 100],
        }
    }
    pub fn print_memory_table(memory: &Memory) {
        let mut table_builder = Builder::default();

        for chunk in memory.items.chunks(10) {
            let row_content: Vec<String> =
                chunk.iter().map(|&item| format!("{:03}", item)).collect();
            table_builder.push_record(row_content);
        }

        let mut table = table_builder.build();
        table.with(Style::modern());

        println!("{}", table);
    }
}
