use base64::{decode, encode};

use crate::plugin::{Plugin, PluginMetadata};

#[derive(Debug)]
pub struct Base64DecodePlugin;

impl Plugin for Base64DecodePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "base64.decode".to_owned(),
            name: "Base64 Decode".to_owned(),
            description: "This decodes a base64 encoded string".to_owned(),
            input_type: "base64".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        match decode(state) {
            Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
                Ok(new_state) => new_state,
                Err(e) => {
                    println!("{e}");
                    state.to_owned()
                }
            },
            Err(e) => {
                println!("{e}");
                state.to_owned()
            }
        }
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}

#[derive(Debug)]
pub struct Base64EncodePlugin;

impl Plugin for Base64EncodePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "base64.encode".to_owned(),
            name: "Base64 Encode".to_owned(),
            description: "This encodes a string to base64".to_owned(),
            input_type: "text".to_owned(),
            output_type: "base64".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        encode(state)
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}
