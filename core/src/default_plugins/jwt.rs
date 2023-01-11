use base64::decode;

use crate::plugin::{Plugin, PluginMetadata};

#[derive(Debug)]
pub struct JwtDecodePlugin;

fn pad_base64(str: &str) -> String {
    let pad_len = 4 - (str.len() % 4);
    let mut new_str = String::from(str);
    if pad_len != 4 {
        for _ in 0..pad_len {
            new_str += "=";
        }
    }

    new_str
}

impl Plugin for JwtDecodePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "jwt.decode".to_owned(),
            name: "Decode JWT".to_owned(),
            description: "This decodes a JWT".to_owned(),
            input_type: "jwt".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        let jwt_parts: Vec<&str> = state.split(".").collect();
        let header = pad_base64(jwt_parts[0]);
        let body = pad_base64(jwt_parts[1]);

        let Some(header)  = (match decode(header) {
            Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
                Ok(new_state) => Some(new_state),
                Err(e) => {
                    println!("{e}");
                    None
                }
            },
            Err(e) => {
                println!("{e}");
                None
            }
        }) else {
            return state.into();
        };

        let Some(body) = (match decode(body) {
            Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
                Ok(new_state) => Some(new_state),
                Err(e) => {
                    println!("{e}");
                    None
                }
            },
            Err(e) => {
                println!("{e}");
                None
            }
        }) else {
            return state.into();
        };

        (header + "." + &body + "." + jwt_parts[2]).into()
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}

#[derive(Debug)]
pub struct JwtFormatPlugin;

impl Plugin for JwtFormatPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "jwt.format".to_owned(),
            name: "JWT Format".to_owned(),
            description: "This formats a JWT to its pretty-printed JSON parts".to_owned(),
            input_type: "jwt".to_owned(),
            output_type: "text".to_owned(),
        }
    }

    fn run(&self, state: &str) -> String {
        let jwt_parts: Vec<&str> = state.split(".").collect();
        let header = pad_base64(jwt_parts[0]);
        let body = pad_base64(jwt_parts[1]);

        let Some(header)  = (match decode(header) {
            Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
                Ok(new_state) => Some(new_state),
                Err(e) => {
                    println!("{e}");
                    None
                }
            },
            Err(e) => {
                println!("{e}");
                None
            }
        }) else {
            return state.into();
        };

        let Some(body) = (match decode(body) {
            Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
                Ok(new_state) => Some(new_state),
                Err(e) => {
                    println!("{e}");
                    None
                }
            },
            Err(e) => {
                println!("{e}");
                None
            }
        }) else {
            return state.into();
        };

        let header = crate::default_plugins::json::JsonFormatPlugin.run(&header);
        let body = crate::default_plugins::json::JsonFormatPlugin.run(&body);

        (header + "." + &body + "." + jwt_parts[2]).into()
    }

    fn plugin_type(&self) -> String {
        "rs".into()
    }
}