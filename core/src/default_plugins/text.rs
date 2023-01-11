use crate::plugin::{Plugin, PluginMetadata};

#[derive(Debug)]
pub struct TextReversePlugin;

impl Plugin for TextReversePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "text.reverse".to_owned(),
            name: "Reverse text".to_owned(),
            description: "This reverses the text".to_owned(),
            input_type: "text".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        state.chars().rev().collect()
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
            id: "text.lowercase".to_owned(),
            name: "Text to lowercase".to_owned(),
            description: "Converts the text to lowercase only letters".to_owned(),
            input_type: "text".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        state.to_lowercase()
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
            id: "text.uppercase".to_owned(),
            name: "Text to uppercase".to_owned(),
            description: "Converts the text to uppercase letters only".to_owned(),
            input_type: "text".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        state.to_uppercase()
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
            id: "text.remove_newlines".to_owned(),
            name: "Remove newlines".to_owned(),
            description: "Removes all newlines from text".to_owned(),
            input_type: "text".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        state.chars().filter(|c| *c != '\n').collect()
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
            id: "text.count_chars".to_owned(),
            name: "Count characters".to_owned(),
            description: "Returns the total count of characters".to_owned(),
            input_type: "text".to_owned(),
            output_type: "number".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        state.chars().count().to_string()
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
            id: "text.count_lines".to_owned(),
            name: "Count lines".to_owned(),
            description: "Returns the total count of lines".to_owned(),
            input_type: "text".to_owned(),
            output_type: "number".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        state.lines().count().to_string()
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}
