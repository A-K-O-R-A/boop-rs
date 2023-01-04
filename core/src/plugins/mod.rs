use std::fs;
use std::io;

pub mod js_plugin;
use js_plugin::JsPlugin;

pub trait Plugin {
    fn metadata(&self) -> PluginMetadata;
    fn run(&self, state: &str) -> String;
    fn warm(&self) -> bool;

    fn from_path<P: AsRef<std::path::Path>>(path: P) -> io::Result<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub name: String,
}

pub fn load_plugin_folder(path: &str) -> io::Result<Vec<Box<dyn Plugin>>> {
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

                println!("{:?}: {:?}", entry.path(), file_type);
            } else {
                println!("Couldn't get file type for {:?}", entry.path());
            }
        }
    }

    Ok(plugins)
}
