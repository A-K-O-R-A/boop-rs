use std::vec;

use urlencoding::{decode, encode};

use crate::plugin::{Plugin, PluginMetadata, PluginResult};

#[derive(Debug)]
pub struct UrlDecodePlugin;

impl Plugin for UrlDecodePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["url", "decode"]
                .iter()
                .map(|&s| s.to_owned())
                .collect(),
            id: "url.decode".to_owned(),
            name: "URL Decode".to_owned(),
            description: "This decodes a url encoded string".to_owned(),
            input_type: "text".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        match decode(state) {
            Ok(new_state) => Ok(new_state.into()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}

#[derive(Debug)]
pub struct UrlEncodePlugin;

impl Plugin for UrlEncodePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["url", "encode"]
                .iter()
                .map(|&s| s.to_owned())
                .collect(),
            id: "url.encode".to_owned(),
            name: "URL Encode".to_owned(),
            description: "This encodes a string with url encoding".to_owned(),
            input_type: "text".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        Ok(encode(state).into())
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}
