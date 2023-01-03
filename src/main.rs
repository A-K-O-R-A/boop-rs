use eframe::egui;

mod editor;
mod plugin;
mod syntax_highlighting;

#[derive(Default)]
struct MyApp {
    code_editor: editor::CodeEditor,
    plugins: Vec<plugin::Plugin>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.code_editor.ui(ui);
        });
    }
}

fn main() {
    let plugins = plugin::load_plugin_folder("./plugins").unwrap();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| {
            Box::new(MyApp {
                plugins,
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
