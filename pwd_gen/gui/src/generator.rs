use crate::settings;
use magic_crypt::*;
use rand::prelude::*;
use rayon::prelude::*;

pub fn generate(len: u8, chars: Vec<String>) -> Vec<String> {
    (0..len)
        .into_par_iter()
        .map(|_| chars[rand::thread_rng().gen_range(0..chars.len())].clone())
        .collect()
}

#[derive(Clone)]
pub struct Generator {
    pub cache_dir: std::path::PathBuf,
    pub calls: i32,
    pub chars: Vec<String>,
    pub len: u8,
    pub passwords: Vec<String>,
    pub special: bool,
    pub thread_pool: rusty_pool::ThreadPool,
    pub upper: bool,
}

impl Generator {
    fn new(passwords: Vec<String>, cache_dir: std::path::PathBuf) -> Self {
        Self {
            passwords,
            calls: 0,
            upper: true,
            special: true,
            len: 12,
            thread_pool: rusty_pool::ThreadPool::default(),
            chars: Vec::default(),
            cache_dir,
        }
    }

    pub fn init() {
        let window_size = eframe::egui::Vec2::new(253.0, 430.0);

        let native_options = eframe::NativeOptions {
            initial_window_size: Some(window_size),
            min_window_size: Some(window_size),
            ..Default::default()
        };

        let cache_dir = dirs::cache_dir().unwrap_or_default();

        let passwords: Vec<String> =
            match std::fs::read_to_string(cache_dir.join(settings::PASSWORDS_FILE_NAME)) {
                Ok(encrypted_passwords) => {
                    match std::fs::read_to_string(cache_dir.join(settings::KEY_FILE_NAME)) {
                        Ok(key) => match magic_crypt::new_magic_crypt!(key, 256)
                            .decrypt_base64_to_string(&encrypted_passwords)
                        {
                            Ok(string) => string.split('\n').map(|a| a.to_string()).collect(),
                            Err(_) => Vec::default(),
                        },
                        Err(_) => Vec::default(),
                    }
                }
                Err(_) => Vec::default(),
            };

        eframe::run_native(
            "Password Generator",
            native_options,
            Box::new(|_| Box::new(Generator::new(passwords, cache_dir))),
        )
    }
}
