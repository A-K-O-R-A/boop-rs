use colored::*;

use core::manager::PluginManager;
use core::plugin::Plugin;
use std::path::PathBuf;

use crate::BoopError;

pub fn list_plugins(plugins_folder: &Option<PathBuf>) -> Result<(), BoopError> {
    let manager = if let Some(path) = plugins_folder {
        PluginManager::from_path(path).map_err(|e| BoopError::IoError(e))?
    } else {
        PluginManager::default()
    };

    print!("{}", "Currently loaded plugins:".bold());
    if let Some(plugins_path) = plugins_folder {
        print!(" (from {:?})", &plugins_path);
    }
    print!("\n");

    for category in manager.categories() {
        println!("\n{}", category.bold().underline());

        for plugin in manager.plugins_in_category(&category) {
            println!(
                " {} {:<25}  {} {}",
                colored_plugin_type(&plugin),
                plugin.metadata().id,
                plugin.metadata().name,
                format!("({})", plugin.metadata().description)
                    .dimmed()
                    .italic()
            )
        }
    }

    Ok(())
}

fn colored_plugin_type(plugin: &Box<dyn Plugin>) -> ColoredString {
    let plugin_type = plugin.plugin_type();

    match plugin_type.as_str() {
        "rs" => "rs".bright_red(),
        "js" => "js".bright_yellow(),
        _ => plugin_type.normal(),
    }
}
