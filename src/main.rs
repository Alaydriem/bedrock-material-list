mod command;

extern crate tokio;

/// Generates a materials list for a given .mcstructure file
#[tokio::main]
async fn main() {
    // Parse arguments with clap => config::Config struct
    let cfg = command::parse();

    match &cfg.cmd {
        command::SubCommand::Structure(cmd) => {
            cmd.run(&cfg).await;
        },
        command::SubCommand::BlockGen(cmd) => {
            cmd.run(&cfg).await;
        }
    }
}
