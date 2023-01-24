use crate::plugin::{Plugin, PluginMetadata, PluginResult};

#[derive(Debug)]
pub struct TextReversePlugin;

impl Plugin for TextReversePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["reverse"].iter().map(|&s| s.to_owned()).collect(),
            id: "text.reverse".to_owned(),
            name: "Reverse text".to_owned(),
            description: "This reverses the text".to_owned(),
            input_type: "text".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        Ok(state.chars().rev().collect())
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}

#[derive(Debug)]
pub struct TextLowercasePlugin;

impl Plugin for TextLowercasePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["lowercase"].iter().map(|&s| s.to_owned()).collect(),
            id: "text.lowercase".to_owned(),
            name: "Text to lowercase".to_owned(),
            description: "Converts the text to lowercase only letters".to_owned(),
            input_type: "text".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        Ok(state.to_lowercase())
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}

#[derive(Debug)]
pub struct TextUppercasePlugin;

impl Plugin for TextUppercasePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["uppercase"].iter().map(|&s| s.to_owned()).collect(),
            id: "text.uppercase".to_owned(),
            name: "Text to uppercase".to_owned(),
            description: "Converts the text to uppercase letters only".to_owned(),
            input_type: "text".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        Ok(state.to_uppercase())
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}

#[derive(Debug)]
pub struct TextRemoveNewlinesPlugin;

impl Plugin for TextRemoveNewlinesPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["remove", "newlines"]
                .iter()
                .map(|&s| s.to_owned())
                .collect(),
            id: "text.remove_newlines".to_owned(),
            name: "Remove newlines".to_owned(),
            description: "Removes all newlines from text".to_owned(),
            input_type: "text".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        Ok(state.chars().filter(|c| *c != '\n').collect())
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}

#[derive(Debug)]
pub struct TextCountCharactersPlugin;

impl Plugin for TextCountCharactersPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["count", "chars"]
                .iter()
                .map(|&s| s.to_owned())
                .collect(),
            id: "text.count_chars".to_owned(),
            name: "Count characters".to_owned(),
            description: "Returns the total count of characters".to_owned(),
            input_type: "text".to_owned(),
            output_type: "number".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        Ok(state.chars().count().to_string())
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}

#[derive(Debug)]
pub struct TextCountLinesPlugin;

impl Plugin for TextCountLinesPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["count", "lines"]
                .iter()
                .map(|&s| s.to_owned())
                .collect(),
            id: "text.count_lines".to_owned(),
            name: "Count lines".to_owned(),
            description: "Returns the total count of lines".to_owned(),
            input_type: "text".to_owned(),
            output_type: "number".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        Ok(state.lines().count().to_string())
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}
