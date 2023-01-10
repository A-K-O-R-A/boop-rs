#[allow(unused_imports)]
use crate::default_plugins;
#[allow(unused_imports)]
use crate::loaders;

use crate::plugin::Plugin;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct PluginManager {
    pub plugins: Vec<Box<dyn Plugin>>,
}

fn default_plugins() -> Vec<Box<dyn Plugin>> {
    #[allow(unused_mut)]
    let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();

    #[cfg(feature = "plugin_base64")]
    plugins.push(Box::new(default_plugins::base64::Base64DecodePlugin));
    #[cfg(feature = "plugin_base64")]
    plugins.push(Box::new(default_plugins::base64::Base64EncodePlugin));

    #[cfg(feature = "plugin_json")]
    plugins.push(Box::new(default_plugins::json::JsonStringifyPlugin));
    #[cfg(feature = "plugin_json")]
    plugins.push(Box::new(default_plugins::json::JsonParsePlugin));

    plugins
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

        Ok(Self { plugins })
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
                            println!("Unsupported plugin type {}, check for disabled features or refer to the documentation", extension);
                        }
                    }
                }
                None => {
                    println!("Unsupported file format {}", entry_path.display());
                }
            }
        }
    }

    Ok(plugins)
}