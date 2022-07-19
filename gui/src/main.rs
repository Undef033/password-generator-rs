#![windows_subsystem = "windows"]

mod generator;
mod settings;
mod ui;
mod utils;

fn main() {
    utils::hide_console();

    generator::Generator::init();
}
