use crate::default_plugins;
use crate::loaders::js::JsPlugin;
use crate::plugin::Plugin;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct PluginManager {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl Default for PluginManager {
    fn default() -> Self {
        let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();

        cfg_if::cfg_if! {
            if #[cfg(feature = "default_plugins")] {
                plugins.push(Box::new(default_plugins::base64::Base64DecodePlugin));
                plugins.push(Box::new(default_plugins::base64::Base64EncodePlugin));
            }
        }

        Self { plugins }
    }
}

impl PluginManager {
    pub fn load_plugin_folder<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut plugins = load_plugin_folder(path)?;

        cfg_if::cfg_if! {
            if #[cfg(feature = "default_plugins")] {
                plugins.push(Box::new(default_plugins::base64::Base64DecodePlugin));
                plugins.push(Box::new(default_plugins::base64::Base64EncodePlugin));
            }
        }

        Ok(Self { plugins })
    }
}

fn load_plugin_folder<P: AsRef<Path>>(path: P) -> io::Result<Vec<Box<dyn Plugin>>> {
    let entries = fs::read_dir(path)?;

    let mut plugins = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            let entry_path = entry_path.to_str().unwrap();

            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let mut nested_plugins = load_plugin_folder(entry_path)?;
                    plugins.append(&mut nested_plugins)
                } else {
                    if entry_path.to_ascii_lowercase().ends_with(".js") {
                        plugins.push(Box::new(JsPlugin::from_path(entry_path)?));
                    } else {
                        println!("Unsupported plugin {}", entry_path);
                    }
                }

                //println!("{:?}: {:?}", entry.path(), file_type);
            } else {
                println!("Couldn't get file type for {:?}", entry.path());
            }
        }
    }

    Ok(plugins)
}
