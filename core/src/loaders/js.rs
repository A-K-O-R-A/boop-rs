use crate::plugin::{Plugin, PluginMetadata};
use quick_js::{Context, ExecutionError, JsValue};
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct JsPlugin {
    pub metadata: PluginMetadata,
    //context: Option<Context>,
    script: String,
}

impl Plugin for JsPlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn run(&self, state: &str) -> String {
        let context = JsPlugin::create_context(&self.script).expect("Unable to create JS Context");

        let value = context.call_function("run", vec![state]).expect(
            format!(
                "Unable to call run function of JS plugin {}",
                self.metadata.id
            )
            .as_str(),
        );

        match value {
            JsValue::String(new_state) => new_state,
            _ => match value.into_string() {
                Some(new_state) => new_state,
                None => {
                    println!(
                        "Unable to convert output of plugin {} to string",
                        self.metadata.id
                    );
                    state.to_owned()
                }
            },
        }
    }

    fn plugin_type(&self) -> String {
        "js".into()
    }
}

impl JsPlugin {
    fn create_context<S>(script: S) -> Result<Context, ExecutionError>
    where
        S: AsRef<str>,
    {
        let context = Context::new().unwrap();

        // Populate the script definition to the context.
        context.eval(script.as_ref())?;

        Ok(context)
    }

    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> io::Result<Self> {
        let path_str = path.as_ref().to_str().unwrap();
        let script = fs::read_to_string(path.as_ref().clone())?;

        let mut context = JsPlugin::create_context(&script)
            .expect(format!("Unable to parse JS plugin {}", path_str).as_str());
        let metadata = PluginMetadata::from_js_context(&mut context)
            .expect(format!("Unable to find metadata in JS Plugin {}", path_str).as_str());

        Ok(Self {
            metadata,
            // context: None, // Some(context)
            script,
        })
    }
}

impl PluginMetadata {
    pub fn from_js_context(context: &mut Context) -> Result<Self, ExecutionError> {
        let obj = context.eval_as::<HashMap<String, String>>("metadata()")?;

        let id = obj.get("id").expect("Plugin id is needed").to_owned();

        let name = obj.get("name").expect("Plugin name is needed").to_owned();

        let description = obj
            .get("description")
            .expect("Plugin description is needed")
            .to_owned();

        let input_type = obj
            .get("inputType")
            .expect("Plugin name is needed")
            .to_owned();

        let output_type = obj
            .get("outputType")
            .expect("Plugin id is needed")
            .to_owned();

        Ok(Self {
            id,
            name,
            description,
            input_type,
            output_type,
        })
    }
}
