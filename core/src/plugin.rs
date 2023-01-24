use std::cmp::Ordering;

pub type PluginError = String;
pub type PluginResult = Result<String, PluginError>;

pub trait Plugin
where
    Self: std::fmt::Debug,
{
    fn metadata(&self) -> PluginMetadata;
    fn plugin_type(&self) -> String;
    fn run(&self, state: &str) -> PluginResult;
}

#[derive(Debug, Clone)]
pub struct PluginMetadata {
    // !TODO Icon for GUI
    pub version: String,
    pub id: String,
    pub author: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub input_type: String,
    pub output_type: String,
}

/*

Implementations



*/

impl PartialEq for dyn Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.metadata().id == other.metadata().id
    }
}
impl Eq for dyn Plugin {}

impl Ord for dyn Plugin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.metadata().id.cmp(&other.metadata().id)
    }
}

impl PartialOrd for dyn Plugin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PluginMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for PluginMetadata {}

impl Ord for PluginMetadata {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for PluginMetadata {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
