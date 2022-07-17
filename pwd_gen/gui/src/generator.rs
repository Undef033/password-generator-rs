use clipboard::{ClipboardContext, ClipboardProvider};
use magic_crypt::*;
use rand::prelude::*;
use std::io::Write;

static SPACING: f32 = 5.0;

fn generate_key() -> String {
    let mut chars = "abcdefghijklmnopqrstuvwxyz".to_string();
    let mut rng = rand::thread_rng();

    chars += &chars.to_uppercase();

    let pwd: Vec<String> = (0..64)
        .into_iter()
        .map(|_| {
            let n = rng.gen_range(0..chars.len());
            chars.get(n..n + 1).unwrap_or("").to_string()
        })
        .collect();

    pwd.join("")
}

pub struct Generator {
    pub passwords: Vec<String>,
    pub special: bool,
    pub upper: bool,
    pub length: u8,
    pub chars: Vec<String>,
    pub cache_dir: std::path::PathBuf,
    calls: i32,
    pub clipboard: ClipboardContext,
}

impl eframe::App for crate::generator::Generator {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(eframe::egui::style::Visuals::dark());
        self.update_ui(ctx);
    }
}

impl Generator {
    fn new(passwords: Vec<String>, cache_dir: std::path::PathBuf, _cc: &eframe::CreationContext<'_>) -> Self {

        Self {
            passwords,
            calls: 0,
            upper: true,
            special: true,
            length: 12,
            chars: Vec::default(),
            cache_dir,
            clipboard: ClipboardProvider::new().unwrap(),
        }
    }

    pub fn init() -> Result<(), std::io::Error> {
        let window_size = eframe::egui::Vec2::new(253.0, 430.0);

        let native_options = eframe::NativeOptions {
            initial_window_size: Some(window_size),
            ..Default::default()
        };

        let cache_dir = dirs::cache_dir().unwrap_or_default();

        let passwords: Vec<String> = match std::fs::read_to_string(cache_dir.join("\\pwdg_p")) {
            Ok(encrypted_passwords) => match std::fs::read_to_string(cache_dir.join("\\pwdg_k")) {
                Ok(key) => match magic_crypt::new_magic_crypt!(key, 256)
                    .decrypt_base64_to_string(&encrypted_passwords)
                {
                    Ok(string) => string.split('\n').map(|a| a.to_string()).collect(),
                    Err(_) => Vec::default(),
                },
                Err(_) => Vec::default(),
            },
            Err(_) => Vec::default(),
        };

        eframe::run_native(
            "Password Generator",
            native_options,
            Box::new(|cc| Box::new(Generator::new(passwords, cache_dir, cc))),
        )
    }

    fn generate(&mut self) {
        let mut rng = rand::thread_rng();

        let pwd: Vec<String> = (0..self.length)
            .map(|_| self.chars[rng.gen_range(0..self.chars.len())].clone())
            .collect();

        self.passwords.push(pwd.join(""));
        self.update_passwords(true);
    }

    pub fn update_chars(&mut self) {
        let mut chars = "abcdefghijklmnopqrstuvwxyz".to_string();

        if self.upper {
            chars += &chars.to_uppercase()
        }

        if self.special {
            chars += "!#$%*+-/?@^_"
        }

        self.chars = chars.split("").map(|c| c.to_string()).collect();
    }

    pub fn update_passwords(&mut self, skip_calls_check: bool) {
        if !skip_calls_check {
            self.calls += 1;

            if self.calls % 64 != 1 {
                return;
            }
        }

        if self.passwords.is_empty() {
            return;
        }

        let passwords = self.passwords.join("\n");
        let key = generate_key();

        let mcrypt = new_magic_crypt!(&key, 256);

        if let Ok(mut key_f) = std::fs::File::create(self.cache_dir.join("\\pwdg_k")) {
            write!(key_f, "{}", key).unwrap_or(())
        }

        if let Ok(mut pwds) = std::fs::File::create(self.cache_dir.join("\\pwdg_p")) {
            write!(pwds, "{}", mcrypt.encrypt_str_to_base64(passwords)).unwrap_or(());
        }

        if !skip_calls_check {
            self.calls = 0
        }
    }

    pub fn update_ui(&mut self, ctx: &eframe::egui::Context) {
        self.update_passwords(false);
        self.update_chars();

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if !self.passwords.is_empty() {
                eframe::egui::ScrollArea::new([false, true]).show(ui, |ui| {
                    for pwd in self.passwords.clone().into_iter().rev() {
                        ui.label(&pwd);

                        if ui.button("Copy").clicked() {
                            self.clipboard.set_contents(pwd.to_string()).unwrap_or(());
                        }

                        ui.add(eframe::egui::Separator::default());
                    }
                });
            }
        });

        eframe::egui::TopBottomPanel::bottom("0").show(ctx, |ui| {
            ui.add_space(SPACING);

            ui.with_layout(
                eframe::egui::Layout::top_down(eframe::egui::Align::Center),
                |ui| {
                    if ui
                        .add_sized(
                            eframe::egui::Vec2::new(100.0, 20.0),
                            eframe::egui::Button::new("Generate"),
                        )
                        .clicked()
                    {
                        self.generate();
                    }
                },
            );

            ui.add_space(SPACING);

            ui.with_layout(
                eframe::egui::Layout::top_down(eframe::egui::Align::Center),
                |ui| {
                    if ui.button(" + ").clicked() {
                        self.length = (self.length + 1).clamp(4, 255);
                    }

                    ui.add_space(1.0);

                    ui.add(
                        eframe::egui::TextEdit::multiline(&mut self.length.to_string())
                            .desired_width(16.0)
                            .desired_rows(1)
                            .interactive(false),
                    );

                    ui.add_space(1.0);

                    if ui.button(" - ").clicked() {
                        self.length = (self.length - 1).clamp(4, 255);
                    }

                    if ui
                        .button(format!(
                            "Uppercase {}",
                            if self.upper { "[on] " } else { "[off]" }
                        ))
                        .clicked()
                    {
                        self.upper = !self.upper
                    }

                    if ui
                        .button(format!(
                            "Special characters {}",
                            if self.special { "[on] " } else { "[off]" }
                        ))
                        .clicked()
                    {
                        self.special = !self.special
                    }
                },
            );

            ui.add_space(SPACING);
        });
    }
}
