use crate::plugin::PluginResult;
use crate::plugin::{Plugin, PluginMetadata};
use quick_js::{Context, ExecutionError, JsValue};
use std::fs;
use std::io;

#[derive(Debug)]
pub struct JsPlugin {
    pub metadata: PluginMetadata,
    script: String,
}

impl Plugin for JsPlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn run(&self, state: &str) -> PluginResult {
        let context = JsPlugin::create_context(&self.script).expect("Unable to create JS Context");

        let result = context.call_function("run", vec![state]);

        match result {
            Ok(value) => match value {
                JsValue::String(new_state) => Ok(new_state),
                _ => Err(format!(
                    "Plugin {} returned value taht is not a string",
                    self.metadata.id
                )),
            },
            Err(e) => Err(format!(
                "Plugin {} threw JS runtime error: {}",
                self.metadata.id,
                e.to_string()
            )),
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
        //let obj = context.eval_as::<String>("metadata()")?;

        let id = context.eval_as::<String>("metadata().id")?;

        let author = context.eval_as::<String>("metadata().author")?;

        let version = context.eval_as::<String>("metadata().version")?;

        let name = context.eval_as::<String>("metadata().name")?;

        let description = context.eval_as::<String>("metadata().description")?;

        let input_type = context.eval_as::<String>("metadata().inputType")?;

        let output_type = context.eval_as::<String>("metadata().outputType")?;

        let tags = context.eval_as::<Vec<String>>("metadata().tags")?;

        Ok(Self {
            id,
            author,
            version,
            tags,
            name,
            description,
            input_type,
            output_type,
        })
    }
}
