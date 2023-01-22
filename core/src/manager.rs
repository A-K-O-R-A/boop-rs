use crate::loaders;
use crate::plugins::default_plugins;

use crate::plugin::Plugin;
use std::collections::HashSet;
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

    pub fn find_plugin(&self, text: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins
            .iter()
            .find(|p| p.metadata().id == *text || p.metadata().name == *text)
    }

    pub fn categories(&self) -> Vec<String> {
        let mut categories: HashSet<String> = HashSet::new();
        {
            let id_list: Vec<String> = (self.plugins)
                .iter()
                .map(|p| p.metadata().id.split_once('.').unwrap().0.to_owned())
                .collect();
            categories.extend(id_list);
        }
        let mut categories: Vec<String> = categories.iter().map(|s| s.to_owned()).collect();
        categories.sort();

        categories
    }

    pub fn plugins_in_category(&self, category: &str) -> Vec<&Box<dyn Plugin>> {
        self.plugins
            .iter()
            .filter(|p| p.metadata().id.starts_with(category))
            .collect()
    }
}
