pub(crate) mod data;
pub(crate) mod structure;

use clap::Parser;
use std::sync::Arc;

use self::structure::StructureConfig;

#[derive(clap::Subcommand, Debug, Clone)]
pub enum SubCommand {
    /// Generates a materials list for a Bedrock .mcstructure file
    Structure(StructureConfig),
}

/// A Drone CI plugin to execute commands on a remote host through Teleport Machine ID
#[derive(Debug, Parser, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Command to execute
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

pub fn parse() -> Arc<Config> {
    let args = Config::parse();
    return Arc::new(args);
}
