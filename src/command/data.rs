use serde::{Deserialize, Serialize};
use snailquote::unescape;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct McStructure {
    pub format_version: i32,
    pub size: Vec<i32>,
    pub structure_world_origin: Vec<i32>,
    pub structure: Structure,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct Structure {
    pub entities: Vec<serde_json::Value>,
    pub palette: Palette,
    pub block_indices: Vec<Vec<i32>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct Palette {
    pub default: BlockPalette,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct BlockPalette {
    pub block_palette: Vec<BlockPaletteItems>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct BlockPaletteItems {
    pub name: String,
    pub version: i32,
    pub states: serde_json::Value,
}

impl BlockPaletteItems {
    pub fn get_attribute(&self) -> String {
        let modifier_states = vec!["color", "sand_stone_type", "stone_slab_type_2"];

        for modifier in modifier_states {
            match self.states.get(modifier) {
                Some(thing) => {
                    return format!(":{}", unescape(thing.to_string().as_str()).unwrap())
                }
                None => {}
            };
        }

        return String::from("");
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct McBlock {
    pub name: String,
    pub count: i32,
}
