use eframe::egui;

pub struct CodeEditor {
    language: String,
    pub code: String,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            language: "rs".into(),
            code: r#"// A very simple example
fn main() {
    println!("Hello world!");
}
"#
            .into(),
        }
    }
}

impl CodeEditor {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { language, code } = self;

        let mut theme = crate::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
        ui.collapsing("Theme", |ui| {
            ui.group(|ui| {
                theme.ui(ui);
                theme.clone().store_in_memory(ui.ctx());
            });
        });

        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job =
                crate::syntax_highlighting::highlight(ui.ctx(), &theme, string, language);
            layout_job.wrap.max_width = wrap_width;
            ui.fonts().layout_job(layout_job)
        };

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(code)
                    //.font(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(30)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
            );
        });
    }
}
