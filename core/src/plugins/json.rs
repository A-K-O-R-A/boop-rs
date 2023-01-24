use crate::plugin::{Plugin, PluginMetadata, PluginResult};

#[derive(Debug)]
pub struct JsonStringifyPlugin;

impl Plugin for JsonStringifyPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["json", "stringify"]
                .iter()
                .map(|&s| s.to_owned())
                .collect(),
            id: "json.stringify".to_owned(),
            name: "Stringify json".to_owned(),
            description: "This stringifies a json string".to_owned(),
            input_type: "json".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        match json::parse(state) {
            Ok(json_value) => Ok(json::stringify(json::stringify(json_value))),
            Err(e) => Err(e.to_string()),
        }
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}

#[derive(Debug)]
pub struct JsonParsePlugin;

impl Plugin for JsonParsePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["json", "parse"]
                .iter()
                .map(|&s| s.to_owned())
                .collect(),
            id: "json.parse".to_owned(),
            name: "De-Stringify json".to_owned(),
            description: "This de-stringifies a double stringified json object".to_owned(),
            input_type: "text".to_owned(),
            output_type: "json".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        match json::parse(state) {
            Ok(stringified_json_value) => {
                if let Some(json_value) = stringified_json_value.as_str() {
                    Ok(json_value.into())
                } else {
                    Err("Unable to convert JSON back to string".to_string())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}

#[derive(Debug)]
pub struct JsonFormatPlugin;

impl Plugin for JsonFormatPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["json", "format"]
                .iter()
                .map(|&s| s.to_owned())
                .collect(),
            id: "json.format".to_owned(),
            name: "Format/Pretty-print json".to_owned(),
            description: "This pretty prints an json string".to_owned(),
            input_type: "json".to_owned(),
            output_type: "json".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        match json::parse(state) {
            Ok(json_value) => Ok(json::stringify_pretty(json_value, 2)),
            Err(e) => Err(e.to_string()),
        }
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}

#[derive(Debug)]
pub struct JsonMinifyPlugin;

impl Plugin for JsonMinifyPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            version: "1.0.0".to_owned(),
            author: "AKORA".to_owned(),
            tags: vec!["json", "minify"]
                .iter()
                .map(|&s| s.to_owned())
                .collect(),
            id: "json.minify".to_owned(),
            name: "Minify json".to_owned(),
            description: "This minfies an json string".to_owned(),
            input_type: "json".to_owned(),
            output_type: "json".to_owned(),
        }
    }

    fn run(&self, state: &str) -> PluginResult {
        match json::parse(state) {
            Ok(json_value) => Ok(json::stringify(json_value)),
            Err(e) => Err(e.to_string()),
        }
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}
