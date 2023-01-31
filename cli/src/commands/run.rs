use core::{manager::PluginManager, plugin::Plugin};
use std::{
    io::{self, BufReader, Read, Write},
    path::PathBuf,
};

use crate::BoopError;

pub fn run(
    plugins_folder: &Option<PathBuf>,
    // file_paths,
    inputs: &Option<Vec<String>>,
    command: &String,
) -> Result<(), BoopError> {
    let manager = if let Some(path) = plugins_folder {
        PluginManager::from_path(path).map_err(|e| BoopError::IoError(e))?
    } else {
        PluginManager::default()
    };

    let plugin = manager.find_plugin(command);

    if let Some(plugin) = plugin {
        if let Some(input) = inputs {
            if input.len() > 1 {
                for i in 0..input.len() {
                    println!("Input {}:", i + 1);
                    run_plugin(plugin, &input[i]);

                    if i != input.len() - 1 {
                        print!("\n")
                    }
                }
            } else {
                run_plugin(plugin, &input[0]);
            }
        } else {
            let mut reader = BufReader::new(io::stdin());
            let mut input_state = String::new();

            reader
                .read_to_string(&mut input_state)
                .map_err(|e| BoopError::IoError(e))?;

            run_plugin(plugin, &input_state);
        };

        io::stdout().flush().map_err(|e| BoopError::IoError(e))?;

        Ok(())
    } else {
        //panic!("Unable to find plugin with id or name \"{}\"", command);
        Err(BoopError::PluginNotFound(command.clone()))
    }
}

fn run_plugin(p: &Box<dyn Plugin>, input: &str) {
    let result = p.run(input).map_err(|e| BoopError::PluginError {
        plugin_id: p.metadata().id,
        error: e,
    });

    match result {
        Ok(new_state) => print!("{new_state}"),
        Err(err) => {
            match err {
                BoopError::IoError(e) => eprintln!("IO Error: {}", e.to_string()),
                BoopError::PluginError { plugin_id, error } => {
                    eprintln!("Plugin {} threw: {}", plugin_id, error)
                }
                BoopError::PluginNotFound(e) => eprintln!("Unable to find plugin {e}"),
            }
            std::process::exit(1)
        }
    }
}
