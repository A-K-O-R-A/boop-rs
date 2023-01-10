use core::{manager::PluginManager, plugin::Plugin};
use eframe::egui::{self, Layout, Modifiers, Window};
use egui_extras::{Column, TableBuilder};

use crate::{views, SEARCH_SIZE, WINDOW_SIZE};

use views::*;

#[derive(Default)]
pub struct BoopRs {
    pub code_editor: editor::CodeEditor,
    pub plugin_manager: PluginManager,
    pub settings_open: bool,
    pub palette_open: bool,
    pub selected_index: usize,
    pub selected_command: Option<Box<dyn Plugin>>,
}

impl eframe::App for BoopRs {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.add(egui::Button::new("Settings")).clicked() {
                self.settings_open = !self.settings_open; //Open Settings window
            }

            let shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::D);
            let mut input = ui.input().to_owned();
            if input.consume_shortcut(&shortcut) {
                self.palette_open = !self.palette_open; //Open command palette
            } else if input.consume_key(Modifiers::NONE, egui::Key::Enter) {
                self.code_editor.code =
                    self.plugin_manager.plugins[self.selected_index].run(&self.code_editor.code);
                self.palette_open = false;
            } else if input.consume_key(Modifiers::NONE, egui::Key::ArrowDown) {
                if self.selected_index < self.plugin_manager.plugins.len() - 1 {
                    self.selected_index += 1;
                }
            } else if input.consume_key(Modifiers::NONE, egui::Key::ArrowUp) {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }

            self.code_editor.ui(ui);
        });

        Window::new("Command Palette")
            .open(&mut self.palette_open)
            .collapsible(false)
            .vscroll(true)
            .title_bar(false)
            .fixed_size(SEARCH_SIZE)
            .fixed_pos([WINDOW_SIZE.x / 2.0 - SEARCH_SIZE.x / 2.0, 30.0])
            .show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                    TableBuilder::new(ui)
                        .column(Column::auto().resizable(true))
                        .column(Column::remainder())
                        .striped(true)
                        .vscroll(true)
                        .body(|mut body| {
                            for i in 0..self.plugin_manager.plugins.len() {
                                let plugin = &self.plugin_manager.plugins[i];

                                body.row(25.0, |mut row| {
                                    row.col(|ui| {
                                        if self.selected_index == i {
                                            ui.strong(&plugin.metadata().name);
                                        } else {
                                            ui.label(&plugin.metadata().name);
                                        }
                                    });
                                    row.col(|ui| {
                                        ui.weak(&plugin.metadata().description);
                                    });
                                });
                            }
                        });
                });
            });

        Window::new("Settings")
            .open(&mut self.settings_open)
            .auto_sized() // !TODO maybe change
            .collapsible(false)
            .show(ctx, |ui| {
                settings::settings_window(&self.plugin_manager, ui)
            });
    }
}
