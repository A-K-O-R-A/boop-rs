use crate::loaders;
use crate::plugins::default_plugins;

use crate::plugin::Plugin;
use std::io;
use std::path::Path;

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
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut plugins = loaders::load_path(path)?;

        plugins.append(&mut default_plugins());
        plugins.sort();

        Ok(Self { plugins })
    }
}
