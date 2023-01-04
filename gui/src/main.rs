use core::plugin_manager::PluginManager;
use eframe::egui::{self, Window};

mod editor;
mod syntax_highlighting;

#[derive(Default)]
struct BoopRs {
    code_editor: editor::CodeEditor,
    plugin_manager: PluginManager,
    settings_open: bool,
}

impl eframe::App for BoopRs {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.add(egui::Button::new("Settings")).clicked() {
                println!("Open settings");
                self.settings_open = !self.settings_open;
            }

            self.code_editor.ui(ui);
        });

        Window::new("Settings")
            .open(&mut self.settings_open)
            .show(ctx, |ui| {
                ui.label("A");
                ui.vertical(|ui| {
                    for plugin in &self.plugin_manager.plugins {
                        ui.label(&plugin.metadata().name);
                    }
                })
            });
    }
}

fn main() {
    let plugin_manager =
        PluginManager::load_plugin_folder("./plugins").expect("Unable to load plugin folder");

    //println!("{:?}", plugin_manager.plugins);

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(700.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| {
            Box::new(BoopRs {
                plugin_manager,
                ..Default::default()
            })
        }),
    )
}

/*
use std::{fs::read_to_string, time::Instant};

fn plugin_test() -> std::io::Result<()> {
    let script = read_to_string("./test.js")?;
    let mut plugin = plugin::Plugin::from_script(script).unwrap();
    println!("{:?}", plugin.metadata);

    let state = r#"
        This is a really long text
        Maybe it is a code(snippet) {

        }
        Or maybe just a token
    "#;
    println!("Current State:\n{}", state);

    let now = Instant::now();
    let state = plugin.run_warm(state).unwrap();
    let elapsed = now.elapsed();

    println!("New State:\n{}", state);
    println!("Took {:?}", elapsed);

    Ok(())
}
*/
