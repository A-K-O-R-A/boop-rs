use clap::Parser;
use commands::Commands;
use std::io;

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None, trailing_var_arg=true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug)]
pub enum BoopError {
    PluginNotFound(String),
    IoError(io::Error),
}

fn main() -> Result<(), BoopError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ListPlugins { plugins_folder } => commands::list_plugins(plugins_folder),
        Commands::Run {
            plugins_folder,
            // file_paths,
            inputs,
            plugin_id: command,
        } => commands::run(plugins_folder, inputs, command),
    }
}
