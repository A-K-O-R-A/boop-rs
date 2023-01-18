use colored::*;

use core::manager::PluginManager;
use core::plugin::Plugin;
use std::{collections::HashSet, path::PathBuf};

use crate::BoopError;

pub fn list_plugins(plugins_folder: &Option<PathBuf>) -> Result<(), BoopError> {
    let manager = PluginManager::new(plugins_folder);

    print!("{}", "Currently loaded plugins:".bold());
    if let Some(plugins_path) = plugins_folder {
        print!(" (from {:?})", &plugins_path);
    }
    print!("\n");

    let mut categories: HashSet<String> = HashSet::new();
    {
        let id_list: Vec<String> = (&manager.plugins)
            .iter()
            .map(|p| p.metadata().id.split_once('.').unwrap().0.to_owned())
            .collect();
        categories.extend(id_list);
    }
    let mut categories: Vec<&String> = categories.iter().collect();
    categories.sort();

    for category in categories {
        println!("\n{}", category.bold().underline());

        for plugin in (&manager.plugins)
            .iter()
            .filter(|p| p.metadata().id.starts_with(category))
        {
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
