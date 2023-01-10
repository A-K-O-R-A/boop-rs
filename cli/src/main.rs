use clap::{Parser, Subcommand};
use core::plugin_manager::PluginManager;
use std::{io, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None, trailing_var_arg=true)]
struct Cli {
    #[arg(short, long)]
    plugins_folder: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,

    input: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Run plugin on the provided input
    Run {
        #[arg(short, long, value_name = "PLUGIN_ID")]
        command: String,
    },

    /// List the currently loaded plugins
    ListPlugins,
}

fn main() {
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
                    " ({}) {} ({} | {})",
                    plugin.plugin_type(),
                    plugin.metadata().name,
                    plugin.metadata().id,
                    plugin.metadata().description
                )
            }

            return;
        }
        Commands::Run { command } => {
            let plugin = manager
                .plugins
                .iter()
                .find(|p| p.metadata().id == *command || p.metadata().name == *command);

            if let Some(plugin) = plugin {
                //let mut buffer = String::new();
                let state = io::stdin()
                    .lines()
                    .map(|l| l.unwrap())
                    .reduce(|acc, e| acc + &e)
                    .unwrap_or("".into());
                //dbg!(&state);
                println!("{}", plugin.run(&state));
            } else {
                panic!("Unable to find plugin with id or name \"{}\"", command);
            }
        }
    }
}
