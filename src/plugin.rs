use boa_engine::{prelude::*, property::Attribute};

#[derive(Debug)]
pub struct Plugin {
    pub metadata: PluginMetadata,
    context: Context,
    script: String,
}

impl Plugin {
    #[allow(dead_code)]
    pub fn run_warm(&mut self, state: &str) -> JsResult<String> {
        self.context
            .register_global_property("state", state, Attribute::all());

        let value = self.context.eval("run(state)")?;
        let new_state = value.to_string(&mut self.context)?.to_string();

        Ok(new_state)
    }

    #[allow(dead_code)]
    pub fn run_cold(&self, state: &str) -> JsResult<String> {
        let mut context = Plugin::create_context(&self.script)?;
        context.register_global_property("state", state, Attribute::all());

        let value = context.eval("run(state)")?;
        let new_state = value.to_string(&mut context)?.to_string();

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

    pub fn from_script(script: String) -> JsResult<Self> {
        let mut context = Plugin::create_context(&script)?;
        let metadata = PluginMetadata::from_context(&mut context)?;

        Ok(Self {
            metadata,
            context,
            script,
        })
    }
}

#[derive(Debug)]
pub struct PluginMetadata {
    pub name: String,
}

impl PluginMetadata {
    pub fn from_context(context: &mut Context) -> JsResult<Self> {
        let value = context.eval("metadata()")?;
        let obj = value.as_object().ok_or(0)?;

        let name = obj.get("name", context)?.to_string(context)?.to_string();

        Ok(Self { name })
    }
}

use std::fs;
use std::io;
pub fn load_plugin_folder(path: &str) -> io::Result<Vec<Plugin>> {
    let entries = fs::read_dir(path)?;

    let mut plugins: Vec<Plugin> = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            let entry_path = entry_path.to_str().unwrap();

            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let mut nested_plugins = load_plugin_folder(entry_path)?;
                    plugins.append(&mut nested_plugins)
                } else {
                    let script = fs::read_to_string(entry_path)?;
                    plugins.push(Plugin::from_script(script).unwrap());
                }

                println!("{:?}: {:?}", entry.path(), file_type);
            } else {
                println!("Couldn't get file type for {:?}", entry.path());
            }
        }
    }

    Ok(plugins)
}
