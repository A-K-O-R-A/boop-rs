use core::plugin_manager::PluginManager;
use eframe::egui::{self, Window};

use crate::views;

use views::*;

#[derive(Default)]
pub struct BoopRs {
    pub code_editor: editor::CodeEditor,
    pub plugin_manager: PluginManager,
    pub settings_open: bool,
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
