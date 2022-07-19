#![windows_subsystem = "windows"]

mod generator;
mod settings;
mod ui;
mod utils;

fn main() -> Result<(), std::io::Error> {
    utils::hide_console();

    generator::Generator::init()?;

    Ok(())
}
