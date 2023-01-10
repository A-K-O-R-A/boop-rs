use clap::{Parser, Subcommand};
use core::manager::PluginManager;
use std::{
    io::{self, BufReader, Read},
    path::PathBuf,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None, trailing_var_arg=true)]
struct Cli {
    #[arg(short, long)]
    plugins_folder: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run plugin on the provided input
    Run {
        #[arg(required = true, value_name = "PLUGIN_ID")]
        command: String,

        /// Stuff to add
        #[arg(required = false)]
        input: Option<Vec<String>>,
    },

    /// List the currently loaded plugins
    ListPlugins,
}

#[derive(Debug)]
pub enum BoopError {
    PluginNotFound(String),
    IoError(io::Error),
}

fn main() -> Result<(), BoopError> {
    let cli = Cli::parse();

    let manager = if let Some(plugins_path) = &cli.plugins_folder {
        PluginManager::load_plugin_folder(&plugins_path)
            .expect(format!("Unable to load plugins from {:?}", &plugins_path).as_str())
    } else {
        PluginManager::default()
    };

    match &cli.command {
        Commands::ListPlugins => {
            print!("Currently loaded plugins:");
            if let Some(plugins_path) = &cli.plugins_folder {
                print!(" (from {:?})", &plugins_path);
            }
            print!("\n");

            for plugin in &manager.plugins {
                println!(
                    " ({} - {}) {} ({})",
                    plugin.plugin_type(),
                    plugin.metadata().id,
                    plugin.metadata().name,
                    plugin.metadata().description
                )
            }

            Ok(())
        }

        Commands::Run { command, input } => {
            let plugin = manager
                .plugins
                .iter()
                .find(|p| p.metadata().id == *command || p.metadata().name == *command);

            if let Some(plugin) = plugin {
                if let Some(input) = input {
                    if input.len() > 1 {
                        for i in 0..input.len() {
                            println!("Input {}:", i + 1);
                            println!("{}", plugin.run(&input[i]));

                            if i != input.len() - 1 {
                                print!("\n")
                            }
                        }
                    } else {
                        print!("{}", plugin.run(&input[0]));
                    }
                } else {
                    let mut reader = BufReader::new(io::stdin());
                    let mut input_state = String::new();

                    let res = reader.read_to_string(&mut input_state);
                    if res.is_err() {
                        return Err(BoopError::IoError(res.unwrap_err()));
                    }

                    print!("{}", plugin.run(&input_state));
                };

                Ok(())
            } else {
                //panic!("Unable to find plugin with id or name \"{}\"", command);
                Err(BoopError::PluginNotFound(command.clone()))
            }
        }
    }
}
