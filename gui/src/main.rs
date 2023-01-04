use core::plugin_manager::PluginManager;
use eframe::egui;

mod app;
mod syntax_highlighting;
mod views;

fn main() {
    let plugin_manager =
        PluginManager::load_plugin_folder("./plugins").expect("Unable to load plugin folder");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(700.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| {
            Box::new(app::BoopRs {
                plugin_manager,
                ..Default::default()
            })
        }),
    )
}

/*
use std::{fs::read_to_string, time::Instant};


    let now = Instant::now();
    let state = plugin.run_warm(state).unwrap();
    let elapsed = now.elapsed();
*/
