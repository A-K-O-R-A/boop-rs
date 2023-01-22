use core::manager::PluginManager;
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
                    println!("{}", plugin.run(&input[i]));

                    if i != input.len() - 1 {
                        print!("\n")
                    }
                }
            } else {
                print!("{}", plugin.run(&input[0]));
            }
            io::stdout().flush().unwrap();
        } else {
            let mut reader = BufReader::new(io::stdin());
            let mut input_state = String::new();

            let res = reader.read_to_string(&mut input_state);
            if res.is_err() {
                return Err(BoopError::IoError(res.unwrap_err()));
            }

            print!("{}", plugin.run(&input_state));
            io::stdout().flush().unwrap();
        };

        Ok(())
    } else {
        //panic!("Unable to find plugin with id or name \"{}\"", command);
        Err(BoopError::PluginNotFound(command.clone()))
    }
}
