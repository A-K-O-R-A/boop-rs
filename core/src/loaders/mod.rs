#[cfg(feature = "loader_js")]
pub mod js;

use std::fs;
use std::io;
use std::path::Path;

use crate::plugin::Plugin;

pub fn load_path<P: AsRef<Path>>(path: P) -> io::Result<Vec<Box<dyn Plugin>>> {
    let entries = fs::read_dir(path)?;

    let mut plugins = Vec::new();

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();

        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            //Support recursion in plugins folder
            let mut nested_plugins = load_path(entry_path)?;
            plugins.append(&mut nested_plugins)
        } else {
            match entry_path.extension() {
                Some(extension) => {
                    let extension = extension.to_str().unwrap().to_ascii_lowercase();
                    match extension.as_str() {
                        "js" => {
                            #[cfg(feature = "loader_js")]
                            plugins.push(Box::new(js::JsPlugin::from_path(entry_path)?));
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
