use super::{Plugin, PluginMetadata};
use boa_engine::{prelude::*, property::Attribute};
use std::fs;
use std::io;

#[derive(Debug)]
pub struct JsPlugin {
    pub metadata: PluginMetadata,
    context: Option<Context>,
    script: String,
}

impl Plugin for JsPlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn run(&self, state: &str) -> String {
        let mut context =
            JsPlugin::create_context(&self.script).expect("Unable to create JS Context");
        context.register_global_property("state", state, Attribute::all());

        let value = context.eval("run(state)").expect("Unable to run JS Plugin");
        let new_state = value
            .to_string(&mut context)
            .expect("Unable to read plugin output")
            .to_string();

        new_state
    }

    fn warm(&self) -> bool {
        self.context.is_some()
    }
}

impl JsPlugin {
    #[allow(dead_code)]
    pub fn run_warm(&mut self, state: &str) -> JsResult<String> {
        let context = self.context.as_mut().unwrap();
        context.register_global_property("state", state, Attribute::all());

        let value = context.eval("run(state)")?;
        let new_state = value.to_string(context)?.to_string();

        Ok(new_state)
    }

    fn create_context<S>(script: S) -> JsResult<Context>
    where
        S: AsRef<[u8]>,
    {
        let mut context = Context::default();

        // Populate the script definition to the context.
        context.eval(script)?;

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
            context: None, // Some(context)
            script,
        })
    }
}

impl PluginMetadata {
    pub fn from_js_context(context: &mut Context) -> JsResult<Self> {
        let value = context.eval("metadata()")?;
        let obj = value.as_object().ok_or(0)?;

        let id = obj
            .get("id", context)?
            .as_string()
            .expect("Plugin id is needed")
            .to_string();
        let name = obj
            .get("name", context)?
            .as_string()
            .expect("Plugin name is needed")
            .to_string();
        let description = obj
            .get("description", context)?
            .as_string()
            .expect("Plugin description is needed")
            .to_string();
        let input_type = obj
            .get("inputType", context)?
            .as_string()
            .expect("Plugin inputType is needed")
            .to_string();
        let output_type = obj
            .get("outputType", context)?
            .as_string()
            .expect("Plugin outputType is needed")
            .to_string();

        Ok(Self {
            id,
            name,
            description,
            input_type,
            output_type,
        })
    }
}
