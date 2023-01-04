pub mod js_loader;

pub trait Plugin
where
    Self: std::fmt::Debug,
{
    fn metadata(&self) -> PluginMetadata;
    fn run(&self, state: &str) -> String;
    fn warm(&self) -> bool;

    /*
    fn from_path<P: AsRef<std::path::Path>>(path: P) -> io::Result<Self>
    where
        Self: Sized;
     */
}

#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub input_type: String,
    pub output_type: String,
}
