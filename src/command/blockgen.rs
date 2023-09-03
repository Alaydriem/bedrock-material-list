use clap::Parser;

use std::collections::HashMap;
use std::process::exit;
use convert_case::{Case, Casing};
#[derive(Debug, Parser, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct BlockgenConfig {
    #[clap(short, long)]
    pub file: String,
}

impl BlockgenConfig {
    /// Generates a material list for a given .mcstructure
    pub async fn run<'a>(&'a self, _cfg: &super::Config) {
        let mut map = match reqwest::get(&self.file).await {
            Ok(response) => match response.json::<HashMap<String, serde_json::Value>>().await {
                Ok(json) => {
                    let mut map = HashMap::<String, String>::new();
                    for (k, _) in &json {
                        let key = k.clone().to_owned();
                        let value = key.clone().replace("minecraft:", "").to_case(Case::Title);
                        map.insert(key, value);
                    }

                    map
                }
                Err(err) => {
                    println!("{}", err.to_string());
                    exit(1);
                }
            },
            Err(err) => {
                println!("{}", err.to_string());
                exit(1);
            }
        };
    
        match std::fs::read_to_string("mcstructure_block_states.json") {
            Ok(text) => match serde_json::from_str::<HashMap<String, String>>(&text) {
                Ok(json) => {
                    for (k, v) in json {
                        map.insert(k, v);
                    }
  
                    std::fs::write("blocks.json", serde_json::to_string_pretty(&map).unwrap_or(String::from(""))).unwrap();
                }
                Err(err) => {
                    println!("{}", err.to_string());
                    exit(1);
                }
            },
            Err(err) => {
                println!("{}", err.to_string());
                exit(1);
            }
        };

    }
}
