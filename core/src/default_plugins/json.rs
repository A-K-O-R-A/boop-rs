use crate::plugin::{Plugin, PluginMetadata};

#[derive(Debug)]
pub struct JsonStringifyPlugin;

impl Plugin for JsonStringifyPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "json.stringify".to_owned(),
            name: "Stringify json".to_owned(),
            description: "This stringifies a json string".to_owned(),
            input_type: "json".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        match json::parse(state) {
            Ok(json_value) => json::stringify(json::stringify(json_value)),
            Err(_e) => json::stringify(state),
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
            id: "json.parse".to_owned(),
            name: "De-Stringify json".to_owned(),
            description: "This de-stringifies a double stringified json object".to_owned(),
            input_type: "text".to_owned(),
            output_type: "json".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        match json::parse(state) {
            Ok(stringified_json_value) => {
                if let Some(json_value) = stringified_json_value.as_str() {
                    json_value.into()
                } else {
                    state.into()
                }
            }
            Err(e) => {
                println!("{e:?}");
                state.into()
            }
        }
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}
