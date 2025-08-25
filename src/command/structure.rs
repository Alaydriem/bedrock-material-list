use clap::Parser;

use super::data::{McBlock, McStructure};
use colored::Colorize;
use nbt::{Blob, Endianness};

use std::collections::HashMap;
use std::fs::File;
use std::process::exit;

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum OutputFormat {
    Json,
    Text,
}

#[derive(Debug, Parser, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct StructureConfig {
    #[clap(short, long)]
    pub file: String,

    #[clap(long, value_enum, default_value_t=OutputFormat::Text)]
    pub format: OutputFormat,
}

impl StructureConfig {
    /// Generates a material list for a given .mcstructure
    pub async fn run<'a>(&'a self, _cfg: &super::Config) {
        // Read the file
        let mut file = match File::open(&self.file) {
            Ok(file) => file,
            Err(error) => {
                println!(
                    "{}",
                    format!("Unable to read file: {} - {}", &self.file, error)
                        .bold()
                        .red()
                );
                exit(1);
            }
        };

        // Parse the NBT
        let blob = match Blob::from_reader(&mut file, Endianness::LittleEndian) {
            Ok(blob) => blob,
            Err(error) => {
                println!(
                    "{}",
                    format!("Unable to read .mcstructure file: {}", error)
                        .bold()
                        .red()
                );
                exit(1);
            }
        };

        // Verify this is an actual .mcstructure file for Bedrock edition
        // This isn't comprehensive, just some quick checks
        // https://wiki.bedrock.dev/nbt/mcstructure.html
        if !blob.get("format_version").eq(&Some(&nbt::Value::Int(1))) {
            println!(
                "{}",
                format!("Incorrect .mcstructure file format").bold().red()
            );
            exit(1)
        }

        if blob.get("structure").is_none() {
            println!(
                "{}",
                format!("Incorrect .mcstructure file format").bold().red()
            );
            exit(1)
        }

        if blob.get("structure_world_origin").is_none() {
            println!(
                "{}",
                format!("Incorrect .mcstructure file format").bold().red()
            );
            exit(1)
        }

        let json_string = match serde_json::to_string(&blob) {
            Ok(string) => string,
            Err(error) => {
                println!(
                    "{}",
                    format!("Incorrect .mcstructure file format - {}", error)
                        .bold()
                        .red()
                );
                exit(1)
            }
        };

        // Convert the NBT to Json
        let mcstructure: McStructure = match serde_json::from_str(&json_string) {
            Ok(s) => s,
            Err(error) => {
                println!(
                    "{}",
                    format!("Unable to parse .mcstructure - {}", error)
                        .bold()
                        .red()
                );
                exit(1)
            }
        };

        // All blocks in the mcstructure stored as their name, and the count
        let mut blocks = Vec::<McBlock>::new();
        for block in mcstructure.structure.palette.default.block_palette {
            let attribute = block.get_attribute();
            let b = McBlock {
                name: format!("{}{}", block.name, attribute),
                count: 0,
            };
            blocks.push(b);
        }

        // The block_indicies have two sublists. Each index corresponds to the index in the block palette
        for i in mcstructure.structure.block_indices[0].clone() {
            let idx = i as usize;
            blocks[idx].count += 1;
        }

        // The blocks include various states, so collapse that down into a single list ignoring the state
        let mut real_blocks = HashMap::<String, i32>::new();
        for block in blocks {
            real_blocks.insert(
                block.name.clone(),
                block.count
                    + if real_blocks.contains_key(&block.name) {
                        real_blocks[&block.name]
                    } else {
                        0
                    },
            );
        }

        let unreal_blocks = vec![
            "minecraft:air",
            "minecraft:stickyPistonArmCollision",
            "minecraft:pistonArmCollision",
            "minecraft:structure_block",
            "minecraft:command_block",
            "minecraft:repeating_command_block",
            "minecraft:chain_command_block"
        ];
        for block in unreal_blocks {
            if real_blocks.contains_key(block) {
                real_blocks.remove(block);
            }
        }

        let raw_block_map = include_bytes!("../../blocks.json");

        let block_map_string = String::from_utf8_lossy(raw_block_map);
        let block_map: HashMap<String, String> = match serde_json::from_str(&block_map_string) {
            Ok(json) => json,
            Err(err) => {
                println!(
                    "{}",
                    format!("Blocks.json missing {}", err.to_string())
                        .bold()
                        .red()
                );
                exit(1);
            }
        };

        let mut output = HashMap::<String, i32>::new();
        for (k, v) in &real_blocks {
            let key = match block_map.get_key_value(k) {
                Some((_, o_v)) => o_v.to_owned(),
                None => k.to_string(),
            };
            // Sum counts for duplicate mapped names
            let entry = output.entry(key).or_insert(0);
            *entry += v;
        }

        println!(
            "{}: {}w x {}d x {}h",
            "Materials List", mcstructure.size[0], mcstructure.size[2], mcstructure.size[1]
        );
        println!("------------------------\n");
        match &self.format {
            &OutputFormat::Json => {
                match serde_json::to_string_pretty(&output) {
                    Ok(text) => println!("{}", text),
                    Err(_) => exit(1),
                };
            }
            &OutputFormat::Text => {
                for (k, v) in output {
                    println!("    {}: {}", k, v);
                }
            }
        }
    }
}
