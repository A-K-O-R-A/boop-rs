use crate::plugin::{Plugin, PluginMetadata};

#[derive(Debug)]
pub struct HTMLDecodePlugin;

impl Plugin for HTMLDecodePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "html.decode".to_owned(),
            name: "HTML Entities Decode".to_owned(),
            description: "This decodes HTML entities".to_owned(),
            input_type: "html".to_owned(),
            output_type: "html".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        html_escape::decode_html_entities(state).into_owned()
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}

#[derive(Debug)]
pub struct HTMLEncodePlugin;

impl Plugin for HTMLEncodePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "html.encode".to_owned(),
            name: "HTML Entities Encode".to_owned(),
            description: "This encodes HTML entities".to_owned(),
            input_type: "html".to_owned(),
            output_type: "html".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        html_escape::encode_safe(state).to_string()
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}
