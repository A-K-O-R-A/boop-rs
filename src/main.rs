use std::{fs::read_to_string, time::Instant};
mod plugin;

fn main() -> std::io::Result<()> {
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
