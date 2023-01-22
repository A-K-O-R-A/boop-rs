use std::fmt::Debug;

use md5::compute;

use crate::plugin::{Plugin, PluginMetadata};

#[derive(Debug)]
pub struct Md5HashPlugin;

impl Plugin for Md5HashPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "md5.hash".to_owned(),
            name: "MD5 Hash".to_owned(),
            description: "This computes the MD5 Hash of the string".to_owned(),
            input_type: "text".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        let digest = compute(state);

        format!("{:x}", digest)
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}
