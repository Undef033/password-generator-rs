use crate::generator;
use crate::settings;
use crate::utils;
use clipboard::ClipboardProvider;

impl eframe::App for generator::Generator {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(eframe::egui::style::Visuals::dark());

        let mut self_clone = self.clone();

        let result: rusty_pool::JoinHandle<Self> = self.clone().thread_pool.evaluate(move || {
            self_clone.chars = utils::get_characters(self_clone.upper, self_clone.special);
            utils::save_passwords(&mut self_clone, false);
            self_clone
        });

        eframe::egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(settings::SPACING + 3.0);

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
                        self.passwords
                            .push(generator::generate(self.len, self.chars.clone()).join(""));
                        utils::save_passwords(self, true);
                    }
                },
            );

            ui.add_space(settings::SPACING);

            ui.with_layout(
                eframe::egui::Layout::top_down(eframe::egui::Align::Center),
                |ui| {
                    if ui.button(" + ").clicked() {
                        self.len = (self.len + 1).clamp(4, 255);
                    }

                    ui.add_space(settings::SPACING / 2.0);

                    ui.add(
                        eframe::egui::TextEdit::multiline(&mut self.len.to_string())
                            .desired_width(16.0)
                            .desired_rows(1)
                            .interactive(false),
                    );

                    ui.add_space(settings::SPACING / 2.0);

                    if ui.button(" - ").clicked() {
                        self.len = (self.len - 1).clamp(4, 255);
                    }

                    ui.add_space(settings::SPACING);

                    if ui
                        .button(format!(
                            "Uppercase {}",
                            if self.upper { "[on] " } else { "[off]" }
                        ))
                        .clicked()
                    {
                        self.upper = !self.upper
                    }

                    ui.add_space(settings::SPACING);

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

            ui.add_space(settings::SPACING);
        });

        let updated = result.await_complete();

        self.cache_dir = updated.cache_dir;
        self.chars = updated.chars;
        self.calls = updated.calls;

        if self.passwords.is_empty() {
            return;
        }

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            eframe::egui::ScrollArea::new([false, true]).show(ui, |ui| {
                for (i, pwd) in self.passwords.clone().into_iter().enumerate().rev() {
                    ui.label(&pwd);

                    if ui.button("Copy").clicked() {
                        if let Ok(mut clipboard_ctx) = clipboard::ClipboardContext::new() {
                            clipboard_ctx.set_contents(pwd.to_string()).unwrap_or(());
                        }
                    }

                    if i != 0 {
                        ui.add(eframe::egui::Separator::default());
                    }
                }
            });
        });
    }
}
