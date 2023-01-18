use clap::{Parser, Subcommand};
use core::manager::PluginManager;
use std::{
    io::{self, BufReader, Read, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None, trailing_var_arg=true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the specified plugin with the provided input
    Run {
        /// Path to plugins folder  
        #[arg(short, long, required = false)]
        plugins_folder: Option<PathBuf>,

        /*
        /// Provide file path to run the plugin on
        #[arg(short = 'f', long = "files", required = false)]
        file_paths: Option<Vec<PathBuf>>,
        */
        /// ID of the plugin you want to run
        #[arg(required = true)]
        plugin_id: String,

        /// The strings you want to run the plugin on
        #[arg(required = false)]
        inputs: Option<Vec<String>>,
    },

    /// Lists the currently loaded plugins
    ListPlugins {
        /// Path to plugins folder  
        #[arg(short, long, required = false)]
        plugins_folder: Option<PathBuf>,
    },
}

#[derive(Debug)]
pub enum BoopError {
    PluginNotFound(String),
    IoError(io::Error),
}

fn manager(plugins_folder: &Option<PathBuf>) -> PluginManager {
    if let Some(plugins_path) = plugins_folder {
        PluginManager::load_plugin_folder(&plugins_path)
            .expect(format!("Unable to load plugins from {:?}", &plugins_path).as_str())
    } else {
        PluginManager::default()
    }
}

fn main() -> Result<(), BoopError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ListPlugins { plugins_folder } => {
            let manager = manager(plugins_folder);

            print!("Currently loaded plugins:");
            if let Some(plugins_path) = plugins_folder {
                print!(" (from {:?})", &plugins_path);
            }
            print!("\n");

            for plugin in &manager.plugins {
                println!(
                    " ({}) {:<30}  {} ({})",
                    plugin.plugin_type(),
                    plugin.metadata().id,
                    plugin.metadata().name,
                    plugin.metadata().description
                )
            }

            Ok(())
        }

        Commands::Run {
            plugins_folder,
            // file_paths,
            inputs,
            plugin_id: command,
        } => {
            let manager = manager(plugins_folder);

            let plugin = manager
                .plugins
                .iter()
                .find(|p| p.metadata().id == *command || p.metadata().name == *command);

            if let Some(plugin) = plugin {
                if let Some(input) = inputs {
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
                    io::stdout().flush().unwrap();
                } else {
                    let mut reader = BufReader::new(io::stdin());
                    let mut input_state = String::new();

                    let res = reader.read_to_string(&mut input_state);
                    if res.is_err() {
                        return Err(BoopError::IoError(res.unwrap_err()));
                    }

                    print!("{}", plugin.run(&input_state));
                    io::stdout().flush().unwrap();
                };

                Ok(())
            } else {
                //panic!("Unable to find plugin with id or name \"{}\"", command);
                Err(BoopError::PluginNotFound(command.clone()))
            }
        }
    }
}
