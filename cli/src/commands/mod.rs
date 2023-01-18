use clap::Subcommand;
use std::path::PathBuf;

mod list_plugins;
mod run;
pub use list_plugins::*;
pub use run::*;

#[derive(Subcommand)]
pub enum Commands {
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
