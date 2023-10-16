#![allow(unused)]

use comfy_table::{self, *};

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
        let mut table = Table::new();

        table
            .load_preset(comfy_table::presets::UTF8_FULL)
            .apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic);

        for chunk in memory.items.chunks(10) {
            let row_content: Vec<Cell> = chunk
                .iter()
                .map(|&item| Cell::new(format!("{:03}", item)))
                .collect();

            table.add_row(row_content);
        }

        println!("{}", table);
    }
}
