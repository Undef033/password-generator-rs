use crate::generator;
use crate::settings;
use magic_crypt::*;
use std::io::Write;

pub fn hide_console() {
    #[cfg(windows)]
    {
        let window = unsafe { winapi::um::wincon::GetConsoleWindow() };

        if window != std::ptr::null_mut() {
            unsafe {
                winapi::um::winuser::ShowWindow(window, winapi::um::winuser::SW_HIDE);
            }
        }
    }
}

pub fn get_characters(upper: bool, special: bool) -> Vec<String> {
    let mut chars = "abcdefghijklmnopqrstuvwxyz".to_string();

    if upper {
        chars += &chars.to_uppercase()
    }

    if special {
        chars += "!#$%*+-/?@^_"
    }

    let mut chars_vec = chars
        .split("")
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    chars_vec.retain(|c| !c.is_empty());

    chars_vec
}

pub fn save_passwords(generator: &mut generator::Generator, should_skip_calls_check: bool) {
    if !should_skip_calls_check {
        generator.calls += 1;
        if generator.calls % 64 != 1 {
            return;
        }
    }

    if generator.passwords.is_empty() {
        return;
    }

    let key = generator::generate(64, get_characters(true, true)).join("");
    let mcrypt = new_magic_crypt!(&key, 256);
    let passwords = generator.passwords.join("\n");

    if let Ok(mut key_file) =
        std::fs::File::create(generator.cache_dir.join(settings::KEY_FILE_NAME))
    {
        write!(key_file, "{}", key).unwrap_or(())
    }

    if let Ok(mut pws_file) =
        std::fs::File::create(generator.cache_dir.join(settings::PASSWORDS_FILE_NAME))
    {
        write!(pws_file, "{}", mcrypt.encrypt_str_to_base64(passwords)).unwrap_or(());
    }

    if !should_skip_calls_check {
        generator.calls = 0
    }
}
