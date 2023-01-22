use crate::default_plugins::default_plugins;
use crate::loaders;

use crate::plugin::Plugin;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PluginManager {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self {
            plugins: default_plugins(),
        }
    }
}

impl PluginManager {
    pub fn load_plugin_folder<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut plugins = load_plugin_folder(path)?;

        plugins.append(&mut default_plugins());
        plugins.sort();

        Ok(Self { plugins })
    }

    pub fn new(plugins_folder: &Option<PathBuf>) -> PluginManager {
        if let Some(plugins_path) = plugins_folder {
            PluginManager::load_plugin_folder(&plugins_path)
                .expect(format!("Unable to load plugins from {:?}", &plugins_path).as_str())
        } else {
            PluginManager::default()
        }
    }
}

fn load_plugin_folder<P: AsRef<Path>>(path: P) -> io::Result<Vec<Box<dyn Plugin>>> {
    let entries = fs::read_dir(path)?;

    let mut plugins = Vec::new();

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();

        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            //Support recursion in plugins folder
            let mut nested_plugins = load_plugin_folder(entry_path)?;
            plugins.append(&mut nested_plugins)
        } else {
            match entry_path.extension() {
                Some(extension) => {
                    let extension = extension.to_str().unwrap().to_ascii_lowercase();
                    match extension.as_str() {
                        "js" => {
                            #[cfg(feature = "loader_js")]
                            plugins.push(Box::new(loaders::js::JsPlugin::from_path(entry_path)?));
                        }
                        _ => {
                            eprintln!("Unsupported plugin type {}, check for disabled features or refer to the documentation", extension);
                        }
                    }
                }
                None => {
                    eprintln!("Unsupported file format {}", entry_path.display());
                }
            }
        }
    }

    Ok(plugins)
}
