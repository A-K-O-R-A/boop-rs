use core::plugin_manager::PluginManager;
use eframe::egui::{self, Layout, RichText, Ui};
use egui_extras::{Column, TableBuilder};

pub fn settings_window(plugin_manager: &PluginManager, ui: &mut Ui) {
    ui.label(RichText::new("Plugins").heading().strong());
    ui.spacing();

    TableBuilder::new(ui)
        .column(Column::auto().resizable(true))
        .column(Column::remainder())
        .striped(true)
        .vscroll(true)
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.label(RichText::new("Name").size(15.0));
            });
            header.col(|ui| {
                ui.label(RichText::new("Description").size(15.0));
            });
        })
        .body(|mut body| {
            for plugin in &plugin_manager.plugins {
                body.row(17.0, |mut row| {
                    row.col(|ui| {
                        ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(&plugin.metadata().name);
                        });
                    });
                    row.col(|ui| {
                        ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(&plugin.metadata().description);
                        });
                    });
                });
            }
        });
}
