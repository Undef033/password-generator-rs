#![windows_subsystem = "windows"]

mod generator;
mod ui;

fn main() -> Result<(), std::io::Error> {
    #[cfg(windows)]
    {
        let window = unsafe { winapi::um::wincon::GetConsoleWindow() };

        if window != std::ptr::null_mut() {
            unsafe {
                winapi::um::winuser::ShowWindow(window, winapi::um::winuser::SW_HIDE);
            }
        }
    }

    generator::Generator::init()?;

    Ok(())
}
