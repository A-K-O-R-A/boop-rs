use core::manager::PluginManager;
use std::path::PathBuf;

use crate::BoopError;

pub fn list_plugins(plugins_folder: &Option<PathBuf>) -> Result<(), BoopError> {
    let manager = PluginManager::new(plugins_folder);

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
