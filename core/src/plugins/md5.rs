use std::fmt::Debug;

use md5::compute;

use crate::plugin::{Plugin, PluginMetadata, PluginResult};

#[derive(Debug)]
pub struct Md5HashPlugin;

impl Plugin for Md5HashPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["md5", "hash"].iter().map(|&s| s.to_owned()).collect(),
            id: "md5.hash".to_owned(),
            name: "MD5 Hash".to_owned(),
            description: "This computes the MD5 Hash of the string".to_owned(),
            input_type: "text".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        let digest = compute(state);

        Ok(format!("{:x}", digest))
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}
